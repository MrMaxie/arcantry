import { lstat, mkdir, readFile, readdir, readlink, realpath, rm, writeFile } from 'node:fs/promises';
import { basename, dirname, isAbsolute, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execa } from 'execa';
import { z } from 'zod';
import { containsManagedSection, removeManagedSection, upsertManagedSection } from './managedContent.js';
import {
  type ProjectConfig,
  parseProjectConfig,
  privateProjectConfigPath,
  projectConfigFilename,
  projectConfigVersion,
  renderProjectConfig,
  resolveProject,
} from './projectConfig.js';

export const repositoryScopeSchema = z.enum(['shared', 'private']);
export const repositoryCompatibilitySchema = z.enum(['claude']);
export type RepositoryScope = z.infer<typeof repositoryScopeSchema>;
export type RepositoryCompatibility = z.infer<typeof repositoryCompatibilitySchema>;
export type RepositoryOperation = 'init' | 'update' | 'remove';
export type RepositoryChangeAction = 'create' | 'update' | 'remove';

export type RepositoryChange = {
  action: RepositoryChangeAction;
  path: string;
  content: string | null;
  expectedContent: string | null;
};

export type RepositoryConflict = { path: string; reason: string };

export type RepositoryPlan = {
  operation: RepositoryOperation;
  scope: RepositoryScope;
  compatibility: RepositoryCompatibility | null;
  root: string;
  changes: RepositoryChange[];
  conflicts: RepositoryConflict[];
};

export type RepositoryDiagnostic = {
  severity: 'error' | 'warning';
  path: string;
  message: string;
  repair?: string;
};

export type RepositoryReport = {
  root: string;
  valid: boolean;
  diagnostics: RepositoryDiagnostic[];
  configPath: string | null;
  scope: RepositoryScope | null;
  config: ProjectConfig | null;
};

export type RepositoryResult = { root: string; applied: RepositoryChange[] };

type LocalTrackingPolicy =
  | { kind: 'private' }
  | { kind: 'remote-tracked'; reference: string }
  | { kind: 'index-only' };

const sharedGuidanceBody = [
  '## Arcantry',
  '',
  'Use `arcantry.toml` for shared Arcantry configuration.',
  'Treat configured OpenSpec sources as accepted product and engineering intent.',
  'Use configured todo.txt sources for quick intake and changelog sources for consumer-facing release history.',
].join('\n');

const privateGuidanceBody = [
  '## Arcantry local context',
  '',
  'Treat `.local/` as private operational state and read `.local/arcantry.toml` for private Arcantry configuration.',
  'Keep private and shared sources independent. Promote or relocate content only through an explicit reviewed operation.',
].join('\n');

export const detectRepository = async (cwd: string): Promise<{ cwd: string; root: string | null; isRepository: boolean }> => {
  try {
    const result = await execa('git', ['rev-parse', '--show-toplevel'], { cwd, reject: true });
    return { cwd, root: resolve(result.stdout.trim()), isRepository: true };
  } catch {
    return { cwd, root: null, isRepository: false };
  }
};

export const resolveRepositoryRoot = async (cwd: string): Promise<string> => {
  const repository = await detectRepository(cwd);
  if (repository.root === null) throw new Error(`No Git repository found from ${cwd}.`);
  return repository.root;
};

export const repositoryConfigPath = (scope: RepositoryScope): string =>
  scope === 'private' ? privateProjectConfigPath : projectConfigFilename;

export const repositoryGuidancePath = (scope: RepositoryScope): string =>
  scope === 'private' ? join('.local', 'AGENTS.md') : 'AGENTS.md';

export const claudeGuidancePath = (scope: RepositoryScope): string =>
  scope === 'private' ? 'CLAUDE.local.md' : 'CLAUDE.md';

export const createRepositoryConfig = (): ProjectConfig => ({
  configVersion: projectConfigVersion,
  sources: {},
});

export const readArcantryConfig = async (root: string, scope: RepositoryScope): Promise<ProjectConfig | null> => {
  const content = await readText(join(root, repositoryConfigPath(scope)));
  return content === null ? null : parseProjectConfig(content);
};

export const planRepositoryInit = async (
  cwd: string,
  scope: RepositoryScope,
  compatibility: RepositoryCompatibility | null = null,
): Promise<RepositoryPlan> => {
  const plan = await createPlan(cwd, 'init', scope, compatibility);
  if (scope === 'private' && !(await planPrivateLocalPolicy(plan))) return plan;
  const path = repositoryConfigPath(scope);
  const existing = await readText(join(plan.root, path));
  if (existing === null) {
    plan.changes.push({ action: 'create', path, content: renderProjectConfig(createRepositoryConfig()), expectedContent: null });
  } else {
    try {
      parseProjectConfig(existing);
    } catch {
      plan.conflicts.push({ path, reason: 'Existing Arcantry configuration is invalid and will not be replaced.' });
    }
  }

  await planSection(plan, repositoryGuidancePath(scope), guidanceFor(scope));
  if (compatibility === 'claude') await planClaudeCompatibility(plan);
  if (scope === 'private') await planGitExclude(plan, compatibility === 'claude' ? ['.local/', 'CLAUDE.local.md'] : ['.local/']);
  return plan;
};

export const planRepositoryUpdate = async (
  cwd: string,
  scope: RepositoryScope,
  compatibility: RepositoryCompatibility | null = null,
): Promise<RepositoryPlan> => {
  const plan = await createPlan(cwd, 'update', scope, compatibility);
  if (scope === 'private' && !(await planPrivateLocalPolicy(plan))) return plan;
  if (!(await validatePlanConfig(plan))) return plan;
  await planSection(plan, repositoryGuidancePath(scope), guidanceFor(scope));
  if (compatibility === 'claude') await planClaudeCompatibility(plan);
  if (scope === 'private') await planGitExclude(plan, compatibility === 'claude' ? ['.local/', 'CLAUDE.local.md'] : ['.local/']);
  return plan;
};

export const planRepositoryRemove = async (cwd: string, scope: RepositoryScope): Promise<RepositoryPlan> => {
  const plan = await createPlan(cwd, 'remove', scope, null);
  const path = repositoryConfigPath(scope);
  const existing = await readText(join(plan.root, path));
  if (existing !== null) {
    try {
      parseProjectConfig(existing);
      plan.changes.push({ action: 'remove', path, content: null, expectedContent: existing });
    } catch {
      plan.conflicts.push({ path, reason: 'Invalid configuration is preserved because ownership cannot be verified.' });
    }
  }
  await planSectionRemoval(plan, repositoryGuidancePath(scope));
  await planClaudeCompatibilityRemoval(plan);
  return plan;
};

export const applyRepositoryPlan = async (plan: RepositoryPlan): Promise<RepositoryResult> => {
  if (plan.conflicts.length > 0) {
    throw new Error(`Cannot apply ${plan.operation}: ${plan.conflicts.map((conflict) => `${conflict.path}: ${conflict.reason}`).join('; ')}`);
  }

  for (const change of plan.changes) {
    if ((await readText(resolvePlanPath(plan.root, change.path))) !== change.expectedContent) {
      throw new Error(`Refusing to change ${change.path}; it changed after the plan was created.`);
    }
  }

  for (const change of plan.changes) {
    const absolutePath = resolvePlanPath(plan.root, change.path);
    if (change.content === null) await rm(absolutePath, { force: true });
    else {
      await mkdir(dirname(absolutePath), { recursive: true });
      await writeFile(absolutePath, change.content);
    }
  }
  return { root: plan.root, applied: plan.changes };
};

export const initRepository = async (
  cwd: string,
  scope: RepositoryScope,
  compatibility: RepositoryCompatibility | null = null,
): Promise<RepositoryResult> => applyRepositoryPlan(await planRepositoryInit(cwd, scope, compatibility));

export const updateRepository = async (
  cwd: string,
  scope: RepositoryScope,
  compatibility: RepositoryCompatibility | null = null,
): Promise<RepositoryResult> => applyRepositoryPlan(await planRepositoryUpdate(cwd, scope, compatibility));

export const removeRepository = async (cwd: string, scope: RepositoryScope): Promise<RepositoryResult> =>
  applyRepositoryPlan(await planRepositoryRemove(cwd, scope));

export const validateRepository = async (cwd: string, doctor = false, configPath?: string): Promise<RepositoryReport> => {
  const repository = await detectRepository(cwd);
  const root = repository.root ?? resolve(cwd);
  const diagnostics: RepositoryDiagnostic[] = [];
  const project = await resolveProject({ cwd: root, configPath, cwdExplicit: true });
  if (project.configPath === null || project.config === null) {
    return { root, valid: true, diagnostics, configPath: null, scope: null, config: null };
  }

  const scope = project.scope === 'private' || project.scope === 'shared' ? project.scope : null;
  for (const shadowed of project.shadowedConfigPaths) {
    diagnostics.push({
      severity: 'warning',
      path: relativePath(root, shadowed),
      message: `Configuration is shadowed by ${relativePath(root, project.configPath)}.`,
    });
  }
  if (scope !== null) {
    await validateSection(root, repositoryGuidancePath(scope), guidanceFor(scope), diagnostics, doctor, scope);
    await validateClaudeCompatibility(root, scope, diagnostics, doctor);
    if (scope === 'private' && repository.isRepository) await validateGitExclude(root, diagnostics, doctor);
  }
  return {
    root,
    valid: diagnostics.every((diagnostic) => diagnostic.severity !== 'error'),
    diagnostics,
    configPath: project.configPath,
    scope,
    config: project.config,
  };
};

export const doctorRepository = async (cwd: string, configPath?: string): Promise<RepositoryReport> =>
  validateRepository(cwd, true, configPath);

const createPlan = async (
  cwd: string,
  operation: RepositoryOperation,
  scope: RepositoryScope,
  compatibility: RepositoryCompatibility | null,
): Promise<RepositoryPlan> => ({
  operation,
  scope,
  compatibility,
  root: await resolveRepositoryRoot(cwd),
  changes: [],
  conflicts: [],
});

const planClaudeCompatibility = async (plan: RepositoryPlan): Promise<void> => {
  const target = claudeGuidancePath(plan.scope);
  const source = repositoryGuidancePath(plan.scope);
  const linkStatus = await guidanceSymlinkStatus(plan.root, target, source);
  if (linkStatus === 'exact') return;
  if (linkStatus === 'other') {
    plan.conflicts.push({ path: target, reason: `Existing link does not point to ${source}.` });
    return;
  }
  await planSection(plan, target, `@${source.replaceAll('\\', '/')}`);
};

const planClaudeCompatibilityRemoval = async (plan: RepositoryPlan): Promise<void> => {
  const target = claudeGuidancePath(plan.scope);
  const source = repositoryGuidancePath(plan.scope);
  const linkStatus = await guidanceSymlinkStatus(plan.root, target, source);
  if (linkStatus === 'exact') return;
  if (linkStatus === 'other') {
    plan.conflicts.push({ path: target, reason: 'Existing compatibility link is preserved because ownership cannot be verified.' });
    return;
  }
  await planSectionRemoval(plan, target);
};

const validatePlanConfig = async (plan: RepositoryPlan): Promise<boolean> => {
  const path = repositoryConfigPath(plan.scope);
  const existing = await readText(join(plan.root, path));
  if (existing === null) {
    plan.conflicts.push({ path, reason: `Run repo init --scope ${plan.scope} before repo update.` });
    return false;
  }
  try {
    parseProjectConfig(existing);
    return true;
  } catch {
    plan.conflicts.push({ path, reason: 'Configuration is invalid and will not be replaced.' });
    return false;
  }
};

const planSection = async (plan: RepositoryPlan, path: string, body: string): Promise<void> => {
  const existingContent = await readText(join(plan.root, path));
  const result = upsertManagedSection(existingContent ?? '', body);
  if (result.status === 'conflict') plan.conflicts.push({ path, reason: result.reason });
  else if (result.status === 'changed') {
    plan.changes.push({
      action: existingContent === null ? 'create' : 'update',
      path,
      content: result.content,
      expectedContent: existingContent,
    });
  }
};

const planSectionRemoval = async (plan: RepositoryPlan, path: string): Promise<void> => {
  const existing = await readText(join(plan.root, path));
  if (existing === null) return;
  const result = removeManagedSection(existing);
  if (result.status === 'conflict') plan.conflicts.push({ path, reason: result.reason });
  else if (result.status === 'changed') {
    plan.changes.push({
      action: result.content.length === 0 ? 'remove' : 'update',
      path,
      content: result.content || null,
      expectedContent: existing,
    });
  }
};

const planGitExclude = async (plan: RepositoryPlan, requiredEntries: string[]): Promise<void> => {
  const path = await resolveGitPath(plan.root, 'info/exclude');
  const existingContent = await readText(path);
  const existing = existingContent ?? '';
  const lines = new Set(existing.split(/\r?\n/));
  const missingEntries = requiredEntries.filter((entry) => !lines.has(entry));
  if (missingEntries.length === 0) return;
  const separator = existing.length > 0 && !existing.endsWith('\n') ? '\n' : '';
  plan.changes.push({
    action: existingContent === null ? 'create' : 'update',
    path,
    content: `${existing}${separator}${missingEntries.map((entry) => `${entry}\n`).join('')}`,
    expectedContent: existingContent,
  });
};

const planPrivateLocalPolicy = async (plan: RepositoryPlan): Promise<boolean> => {
  const policy = await localTrackingPolicy(plan.root);
  if (policy.kind === 'private') return true;
  plan.conflicts.push({
    path: '.local/',
    reason:
      policy.kind === 'remote-tracked'
        ? `Configured default remote branch ${policy.reference} tracks .local; preserving that repository policy instead of applying Arcantry's private-local convention.`
        : 'The current Git index tracks .local while the configured default remote branch does not. Remove it from the index in a separate explicitly authorized operation before private adoption.',
  });
  return false;
};

const localTrackingPolicy = async (root: string): Promise<LocalTrackingPolicy> => {
  const reference = await configuredDefaultRemoteReference(root);
  if (reference !== null && (await gitRead(root, ['ls-tree', '-r', '--name-only', reference, '--', '.local'])).trim() !== '') {
    return { kind: 'remote-tracked', reference };
  }
  if ((await gitRead(root, ['ls-files', '--', '.local'])).trim() !== '') return { kind: 'index-only' };
  return { kind: 'private' };
};

const configuredDefaultRemoteReference = async (root: string): Promise<string | null> => {
  const remotes = (await gitRead(root, ['remote']))
    .split(/\r?\n/u)
    .filter(Boolean)
    .sort((left, right) => Number(right === 'origin') - Number(left === 'origin'));
  for (const remote of remotes) {
    const result = await execa('git', ['symbolic-ref', '--quiet', `refs/remotes/${remote}/HEAD`], {
      cwd: root,
      reject: false,
    });
    if (result.exitCode === 0 && result.stdout.trim() !== '') return result.stdout.trim();
  }
  return null;
};

const gitRead = async (root: string, arguments_: string[]): Promise<string> =>
  (await execa('git', arguments_, { cwd: root, reject: true })).stdout;

const validateClaudeCompatibility = async (
  root: string,
  scope: RepositoryScope,
  diagnostics: RepositoryDiagnostic[],
  doctor: boolean,
): Promise<void> => {
  const path = claudeGuidancePath(scope);
  const source = repositoryGuidancePath(scope);
  const linkStatus = await guidanceSymlinkStatus(root, path, source);
  if (linkStatus === 'exact') return;
  if (linkStatus === 'other') {
    diagnostics.push({
      severity: 'error',
      path,
      message: `Claude compatibility link does not point to ${source}.`,
      ...(doctor ? { repair: `Replace the link or run \`arcantry repo update --scope ${scope} --compat claude\`.` } : {}),
    });
    return;
  }
  const content = await readText(join(root, path));
  if (content === null || !containsManagedSection(content)) return;
  if (upsertManagedSection(content, `@${source.replaceAll('\\', '/')}`).status !== 'unchanged') {
    diagnostics.push({
      severity: 'error',
      path,
      message: 'Arcantry managed Claude compatibility import is outdated.',
      ...(doctor ? { repair: `Run \`arcantry repo update --scope ${scope} --compat claude\`.` } : {}),
    });
  }
};

const validateSection = async (
  root: string,
  path: string,
  body: string,
  diagnostics: RepositoryDiagnostic[],
  doctor: boolean,
  scope: RepositoryScope,
): Promise<void> => {
  const content = await readText(join(root, path));
  if (content === null || upsertManagedSection(content, body).status !== 'unchanged') {
    diagnostics.push({
      severity: 'error',
      path,
      message: 'Arcantry managed section is missing or outdated.',
      ...(doctor ? { repair: `Run \`arcantry repo update --scope ${scope}\`.` } : {}),
    });
  }
};

const validateGitExclude = async (root: string, diagnostics: RepositoryDiagnostic[], doctor: boolean): Promise<void> => {
  const policy = await localTrackingPolicy(root);
  if (policy.kind !== 'private') {
    diagnostics.push({
      severity: 'error',
      path: '.local/',
      message:
        policy.kind === 'remote-tracked'
          ? `Configured default remote branch ${policy.reference} tracks .local; this conflicts with Arcantry's private-local convention.`
          : 'The current Git index tracks .local while the configured default remote branch does not. Remove it from the index only through a separate explicitly authorized operation.',
    });
    return;
  }
  const path = await resolveGitPath(root, 'info/exclude');
  const lines = new Set(((await readText(path)) ?? '').split(/\r?\n/));
  const privateClaudeContent = await readText(join(root, claudeGuidancePath('private')));
  const requiredEntries = [
    '.local/',
    ...(privateClaudeContent !== null && containsManagedSection(privateClaudeContent) ? ['CLAUDE.local.md'] : []),
  ];
  for (const entry of requiredEntries) if (!lines.has(entry)) {
    diagnostics.push({
      severity: 'error',
      path: '.git/info/exclude',
      message: `${entry} must be excluded locally.`,
      ...(doctor ? { repair: `Run \`arcantry repo update --scope private${entry === 'CLAUDE.local.md' ? ' --compat claude' : ''}\`.` } : {}),
    });
  }
};

const guidanceSymlinkStatus = async (
  root: string,
  targetPath: string,
  sourcePath: string,
): Promise<'none' | 'exact' | 'other'> => {
  const target = join(root, targetPath);
  try {
    const stats = await lstat(target);
    if (!stats.isSymbolicLink()) return 'none';
    const raw = await readlink(target);
    const [actual, expected] = await Promise.all([
      realpath(resolve(dirname(target), raw)),
      realpath(join(root, sourcePath)),
    ]);
    return normalizePath(actual) === normalizePath(expected) ? 'exact' : 'other';
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') return 'none';
    return 'other';
  }
};

const normalizePath = (path: string): string => process.platform === 'win32' ? path.toLowerCase() : path;

const guidanceFor = (scope: RepositoryScope): string => scope === 'private' ? privateGuidanceBody : sharedGuidanceBody;

const resolveGitPath = async (root: string, gitPath: string): Promise<string> => {
  const result = await execa('git', ['rev-parse', '--git-path', gitPath], { cwd: root, reject: true });
  return resolve(root, result.stdout.trim());
};

export const getOpenSpecAssetFiles = async (): Promise<Record<string, string>> => {
  const moduleDirectory = dirname(fileURLToPath(import.meta.url));
  const packageRoot = ['src', 'dist'].includes(basename(moduleDirectory)) ? dirname(moduleDirectory) : moduleDirectory;
  const candidates = [join(packageRoot, 'assets', 'openspec'), resolve(packageRoot, '..', '..', 'openspec', 'schemas', 'arcantry')];
  const assetRoot = (await firstDirectory(candidates)) ?? null;
  if (assetRoot === null) throw new Error('Arcantry OpenSpec assets are missing from the package.');

  const files: Record<string, string> = {};
  for (const absolutePath of await listFiles(assetRoot)) {
    const relativePath = relative(assetRoot, absolutePath).replaceAll('\\', '/');
    files[`openspec/schemas/arcantry/${relativePath}`] = await readFile(absolutePath, 'utf8');
  }
  return files;
};

const listFiles = async (root: string): Promise<string[]> => {
  const files: string[] = [];
  for (const entry of await readdir(root, { withFileTypes: true })) {
    const path = join(root, entry.name);
    if (entry.isDirectory()) files.push(...(await listFiles(path)));
    else if (entry.isFile()) files.push(path);
  }
  return files.sort((left, right) => left.localeCompare(right));
};

const firstDirectory = async (paths: string[]): Promise<string | undefined> => {
  for (const path of paths) if ((await pathKind(path)) === 'directory') return path;
  return undefined;
};

const resolvePlanPath = (root: string, path: string): string => isAbsolute(path) ? path : join(root, path);
const relativePath = (root: string, path: string): string => {
  const value = relative(root, path).replaceAll('\\', '/');
  return value.startsWith('../') || value === '..' ? path : value;
};

const readText = async (path: string): Promise<string | null> => {
  try {
    return await readFile(path, 'utf8');
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') return null;
    throw error;
  }
};

const pathKind = async (path: string): Promise<'file' | 'directory' | 'missing'> => {
  try {
    const stats = await lstat(path);
    return stats.isDirectory() ? 'directory' : 'file';
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') return 'missing';
    throw error;
  }
};

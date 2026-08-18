import { lstat, mkdir, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import { basename, dirname, isAbsolute, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execa } from 'execa';
import { z } from 'zod';
import { removeManagedSection, upsertManagedSection } from './managedContent.js';
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
export type RepositoryScope = z.infer<typeof repositoryScopeSchema>;
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

export const createRepositoryConfig = (): ProjectConfig => ({
  configVersion: projectConfigVersion,
  sources: {},
});

export const readArcantryConfig = async (root: string, scope: RepositoryScope): Promise<ProjectConfig | null> => {
  const content = await readText(join(root, repositoryConfigPath(scope)));
  return content === null ? null : parseProjectConfig(content);
};

export const planRepositoryInit = async (cwd: string, scope: RepositoryScope): Promise<RepositoryPlan> => {
  const plan = await createPlan(cwd, 'init', scope);
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
  if (scope === 'private') await planGitExclude(plan);
  return plan;
};

export const planRepositoryUpdate = async (cwd: string, scope: RepositoryScope): Promise<RepositoryPlan> => {
  const plan = await createPlan(cwd, 'update', scope);
  if (!(await validatePlanConfig(plan))) return plan;
  await planSection(plan, repositoryGuidancePath(scope), guidanceFor(scope));
  if (scope === 'private') await planGitExclude(plan);
  return plan;
};

export const planRepositoryRemove = async (cwd: string, scope: RepositoryScope): Promise<RepositoryPlan> => {
  const plan = await createPlan(cwd, 'remove', scope);
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

export const initRepository = async (cwd: string, scope: RepositoryScope): Promise<RepositoryResult> =>
  applyRepositoryPlan(await planRepositoryInit(cwd, scope));

export const updateRepository = async (cwd: string, scope: RepositoryScope): Promise<RepositoryResult> =>
  applyRepositoryPlan(await planRepositoryUpdate(cwd, scope));

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

const createPlan = async (cwd: string, operation: RepositoryOperation, scope: RepositoryScope): Promise<RepositoryPlan> => ({
  operation,
  scope,
  root: await resolveRepositoryRoot(cwd),
  changes: [],
  conflicts: [],
});

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

const planGitExclude = async (plan: RepositoryPlan): Promise<void> => {
  const path = await resolveGitPath(plan.root, 'info/exclude');
  const existingContent = await readText(path);
  const existing = existingContent ?? '';
  const lines = new Set(existing.split(/\r?\n/));
  if (lines.has('.local/')) return;
  const separator = existing.length > 0 && !existing.endsWith('\n') ? '\n' : '';
  plan.changes.push({
    action: existingContent === null ? 'create' : 'update',
    path,
    content: `${existing}${separator}.local/\n`,
    expectedContent: existingContent,
  });
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
  const path = await resolveGitPath(root, 'info/exclude');
  const lines = new Set(((await readText(path)) ?? '').split(/\r?\n/));
  if (!lines.has('.local/')) {
    diagnostics.push({
      severity: 'error',
      path: '.git/info/exclude',
      message: '.local/ must be excluded locally.',
      ...(doctor ? { repair: 'Run `arcantry repo update --scope private`.' } : {}),
    });
  }
};

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

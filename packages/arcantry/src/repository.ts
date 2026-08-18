import { lstat, mkdir, readFile, readdir, rm, writeFile } from 'node:fs/promises';
import { basename, dirname, isAbsolute, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execa } from 'execa';
import { ZodError } from 'zod';
import { agentArtifacts, getAgentArtifact } from './agents.js';
import {
  type AgentName,
  type ArcantryConfig,
  type CreateArcantryConfigInput,
  arcantryConfigSchema,
  createArcantryConfig,
  renderArcantryConfig,
} from './config.js';
import { containsManagedSection, removeManagedSection, renderManagedSection, upsertManagedSection } from './managedContent.js';

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
  config: ArcantryConfig | null;
};

export type RepositoryResult = { root: string; applied: RepositoryChange[] };

const configPath = '.local/arcantry.json';

const generatedScaffoldContents = {
  justfile: [
    '# arcantry:generated',
    'set dotenv-load := false',
    '',
    'setup:',
    '    pnpm install',
    '',
    'check:',
    '    pnpm run check',
    '',
    'build:',
    '    pnpm run build',
    '',
    'ci: check build',
    '',
  ].join('\n'),
  'mise.toml': [
    '# arcantry:generated',
    '[tools]',
    'node = "24"',
    'pnpm = "10"',
    'just = "1"',
    '',
  ].join('\n'),
  'openspec/config.yaml': [
    '# arcantry:generated',
    'schema: arcantry',
    '',
    'context: |',
    '  OpenSpec is the only source of product and engineering specifications in this repository.',
    '',
  ].join('\n'),
};

const generatedScaffolds: Record<string, { content: string; compatible: (content: string) => boolean }> = {
  justfile: {
    content: generatedScaffoldContents.justfile,
    compatible: (content) =>
      ['setup', 'check', 'build', 'ci'].every((recipe) => new RegExp(`^${recipe}(?:\\s[^:]*)?:`, 'm').test(content)),
  },
  'mise.toml': {
    content: generatedScaffoldContents['mise.toml'],
    compatible: (content) => hasToolMajor(content, 'node', 24) && hasToolMajor(content, 'pnpm', 10) && hasTool(content, 'just'),
  },
  'openspec/config.yaml': {
    content: generatedScaffoldContents['openspec/config.yaml'],
    compatible: (content) => /^schema:\s*arcantry\s*$/m.test(content),
  },
};

const localGuidanceBody = [
  '## Arcantry local context',
  '',
  'Treat `.local/` as private operational state. Read `.local/arcantry.json` for configured agents and ordered task sources.',
  'A source configured as `readwrite` still requires explicit authorization before each external write.',
  'Keep product specifications and release intent in `openspec/`.',
].join('\n');

const sharedGuidanceBody = [
  '## Arcantry',
  '',
  'Use `openspec/` as the only source of product and engineering specifications.',
  'Read `.local/arcantry.json` when present for private operational configuration.',
].join('\n');

const cursorRuleBody = [
  '---',
  'description: Arcantry repository guidance',
  'alwaysApply: true',
  '---',
  '',
  renderManagedSection(sharedGuidanceBody),
  '',
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
  if (repository.root === null) {
    throw new Error(`No Git repository found from ${cwd}.`);
  }
  return repository.root;
};

export const readArcantryConfig = async (root: string): Promise<ArcantryConfig | null> => {
  const content = await readText(join(root, configPath));
  if (content === null) {
    return null;
  }
  return arcantryConfigSchema.parse(JSON.parse(content));
};

export const planRepositoryInit = async (cwd: string, input: CreateArcantryConfigInput): Promise<RepositoryPlan> => {
  const root = await resolveRepositoryRoot(cwd);
  const config = createArcantryConfig(input);
  const plan: RepositoryPlan = { operation: 'init', root, changes: [], conflicts: [] };

  await planExactFile(plan, configPath, renderArcantryConfig(config), 'Existing Arcantry configuration differs from the requested setup.');
  for (const [path, scaffold] of Object.entries(generatedScaffolds)) {
    await planCompatibleFile(plan, path, scaffold, `Existing ${path} is incompatible with the Arcantry repository contract.`);
  }
  for (const [path, content] of Object.entries(await getOpenSpecAssetFiles())) {
    await planExactFile(plan, path, content, `Existing ${path} differs from the Arcantry OpenSpec schema.`);
  }
  await planSection(plan, '.local/AGENTS.md', localGuidanceBody, false);
  await planConfiguredEntrypoints(plan, config.agents, false);
  await planGitExcludes(plan, config);

  return plan;
};

export const planRepositoryUpdate = async (cwd: string): Promise<RepositoryPlan> => {
  const root = await resolveRepositoryRoot(cwd);
  const plan: RepositoryPlan = { operation: 'update', root, changes: [], conflicts: [] };
  const config = await readConfigForPlan(plan);
  if (config === null) {
    return plan;
  }

  for (const [path, scaffold] of Object.entries(generatedScaffolds)) {
    await planCompatibleFile(plan, path, scaffold, `Existing ${path} is incompatible with the Arcantry repository contract.`);
  }
  for (const [path, content] of Object.entries(await getOpenSpecAssetFiles())) {
    await planExactFile(plan, path, content, `Existing ${path} differs from the Arcantry OpenSpec schema.`);
  }
  await planSection(plan, '.local/AGENTS.md', localGuidanceBody, true);
  await planConfiguredEntrypoints(plan, config.agents, true);
  await planGitExcludes(plan, config);

  return plan;
};

export const planRepositoryRemove = async (cwd: string): Promise<RepositoryPlan> => {
  const root = await resolveRepositoryRoot(cwd);
  const plan: RepositoryPlan = { operation: 'remove', root, changes: [], conflicts: [] };

  const configContent = await readText(join(root, configPath));
  if (configContent !== null) {
    try {
      arcantryConfigSchema.parse(JSON.parse(configContent));
      plan.changes.push({ action: 'remove', path: configPath, content: null, expectedContent: configContent });
    } catch {
      plan.conflicts.push({ path: configPath, reason: 'Invalid configuration is preserved because ownership cannot be verified.' });
    }
  }

  for (const [path, scaffold] of Object.entries(generatedScaffolds)) {
    if (path.startsWith('openspec/')) {
      continue;
    }
    const existing = await readText(join(root, path));
    if (existing === scaffold.content) {
      plan.changes.push({ action: 'remove', path, content: null, expectedContent: existing });
    }
  }

  await planSectionRemoval(plan, '.local/AGENTS.md');
  for (const artifact of agentArtifacts) {
    await planSectionRemoval(plan, artifact.path);
  }

  return plan;
};

export const applyRepositoryPlan = async (plan: RepositoryPlan): Promise<RepositoryResult> => {
  if (plan.conflicts.length > 0) {
    throw new Error(`Cannot apply ${plan.operation}: ${plan.conflicts.map((conflict) => `${conflict.path}: ${conflict.reason}`).join('; ')}`);
  }

  for (const change of plan.changes) {
    const current = await readText(resolvePlanPath(plan.root, change.path));
    if (current !== change.expectedContent) {
      throw new Error(`Refusing to change ${change.path}; it changed after the plan was created.`);
    }
  }

  for (const change of plan.changes) {
    const absolutePath = resolvePlanPath(plan.root, change.path);
    if (change.content === null) {
      await rm(absolutePath, { force: true });
      continue;
    }
    await mkdir(dirname(absolutePath), { recursive: true });
    await writeFile(absolutePath, change.content);
  }

  return { root: plan.root, applied: plan.changes };
};

export const initRepository = async (cwd: string, input: CreateArcantryConfigInput): Promise<RepositoryResult> =>
  applyRepositoryPlan(await planRepositoryInit(cwd, input));

export const updateRepository = async (cwd: string): Promise<RepositoryResult> =>
  applyRepositoryPlan(await planRepositoryUpdate(cwd));

export const removeRepository = async (cwd: string): Promise<RepositoryResult> =>
  applyRepositoryPlan(await planRepositoryRemove(cwd));

export const validateRepository = async (cwd: string): Promise<RepositoryReport> => {
  const root = await resolveRepositoryRoot(cwd);
  const diagnostics: RepositoryDiagnostic[] = [];
  let config: ArcantryConfig | null = null;

  try {
    config = await readArcantryConfig(root);
    if (config === null) {
      diagnostics.push({ severity: 'error', path: configPath, message: 'Arcantry configuration is missing.' });
    }
  } catch (error) {
    const detail = error instanceof ZodError ? error.issues.map((issue) => issue.message).join(' ') : 'Configuration is not valid JSON.';
    diagnostics.push({ severity: 'error', path: configPath, message: detail });
  }

  for (const [path, scaffold] of Object.entries(generatedScaffolds)) {
    const content = await readText(join(root, path));
    if (content === null) {
      diagnostics.push({ severity: 'error', path, message: 'Required repository artifact is missing.' });
    } else if (!scaffold.compatible(content)) {
      diagnostics.push({ severity: 'error', path, message: 'Repository artifact is incompatible with the Arcantry contract.' });
    }
  }
  for (const [path, expected] of Object.entries(await getOpenSpecAssetFiles())) {
    const content = await readText(join(root, path));
    if (content === null) {
      diagnostics.push({ severity: 'error', path, message: 'Required Arcantry OpenSpec schema asset is missing.' });
    } else if (content !== expected) {
      diagnostics.push({ severity: 'error', path, message: 'OpenSpec schema asset differs from the installed Arcantry version.' });
    }
  }

  if (config !== null) {
    await validateSection(root, '.local/AGENTS.md', localGuidanceBody, diagnostics);
    for (const agent of config.agents) {
      await validateSection(root, getAgentArtifact(agent).path, sharedGuidanceBody, diagnostics);
    }
    await validateGitExcludes(root, config, diagnostics);
  }

  return { root, valid: diagnostics.every((diagnostic) => diagnostic.severity !== 'error'), diagnostics, config };
};

export const doctorRepository = async (cwd: string): Promise<RepositoryReport> => {
  const report = await validateRepository(cwd);
  return {
    ...report,
    diagnostics: report.diagnostics.map((diagnostic) => ({
      ...diagnostic,
      repair: repairForDiagnostic(diagnostic),
    })),
  };
};

const planExactFile = async (plan: RepositoryPlan, path: string, desired: string, conflictReason: string): Promise<void> => {
  const existing = await readText(join(plan.root, path));
  if (existing === null) {
    plan.changes.push({ action: 'create', path, content: desired, expectedContent: null });
  } else if (existing !== desired) {
    plan.conflicts.push({ path, reason: conflictReason });
  }
};

const planCompatibleFile = async (
  plan: RepositoryPlan,
  path: string,
  scaffold: { content: string; compatible: (content: string) => boolean },
  conflictReason: string,
): Promise<void> => {
  const existing = await readText(join(plan.root, path));
  if (existing === null) {
    plan.changes.push({ action: 'create', path, content: scaffold.content, expectedContent: null });
  } else if (!scaffold.compatible(existing)) {
    plan.conflicts.push({ path, reason: conflictReason });
  }
};

const planConfiguredEntrypoints = async (plan: RepositoryPlan, agents: AgentName[], update: boolean): Promise<void> => {
  for (const agent of agents) {
    const artifact = getAgentArtifact(agent);
    if (artifact.kind === 'cursor-rule') {
      if (update && containsManagedSection((await readText(join(plan.root, artifact.path))) ?? '')) {
        await planSection(plan, artifact.path, sharedGuidanceBody, true);
      } else {
        await planExactFile(plan, artifact.path, cursorRuleBody, 'Existing Cursor rule is not owned by Arcantry.');
      }
    } else {
      await planSection(plan, artifact.path, sharedGuidanceBody, update);
    }
  }
};

const planSection = async (plan: RepositoryPlan, path: string, body: string, update: boolean): Promise<void> => {
  const existingContent = await readText(join(plan.root, path));
  const existing = existingContent ?? '';
  const result = upsertManagedSection(existing, body);
  if (result.status === 'conflict') {
    plan.conflicts.push({ path, reason: result.reason });
  } else if (result.status === 'changed') {
    if (!update && containsManagedSection(existing)) {
      plan.conflicts.push({ path, reason: 'Existing Arcantry section differs; run repo update.' });
    } else {
      plan.changes.push({
        action: existingContent === null ? 'create' : 'update',
        path,
        content: result.content,
        expectedContent: existingContent,
      });
    }
  }
};

const planSectionRemoval = async (plan: RepositoryPlan, path: string): Promise<void> => {
  const existing = await readText(join(plan.root, path));
  if (existing === null) {
    return;
  }
  const result = removeManagedSection(existing);
  if (result.status === 'conflict') {
    plan.conflicts.push({ path, reason: result.reason });
  } else if (result.status === 'changed') {
    plan.changes.push({ action: result.content.length === 0 ? 'remove' : 'update', path, content: result.content || null, expectedContent: existing });
  }
};

const readConfigForPlan = async (plan: RepositoryPlan): Promise<ArcantryConfig | null> => {
  try {
    const config = await readArcantryConfig(plan.root);
    if (config === null) {
      plan.conflicts.push({ path: configPath, reason: 'Run repo init before repo update.' });
    }
    return config;
  } catch {
    plan.conflicts.push({ path: configPath, reason: 'Configuration is invalid and will not be replaced.' });
    return null;
  }
};

const planGitExcludes = async (plan: RepositoryPlan, _config: ArcantryConfig): Promise<void> => {
  const gitPath = await resolveGitPath(plan.root, 'info/exclude');
  const existingContent = await readText(gitPath);
  const existing = existingContent ?? '';
  const required = ['.local/'];
  const lines = new Set(existing.split(/\r?\n/));
  const missing = required.filter((line) => !lines.has(line));
  if (missing.length === 0) {
    return;
  }
  const separator = existing.length > 0 && !existing.endsWith('\n') ? '\n' : '';
  plan.changes.push({
    action: existingContent === null ? 'create' : 'update',
    path: gitPath,
    content: `${existing}${separator}${missing.join('\n')}\n`,
    expectedContent: existingContent,
  });
};

const validateSection = async (
  root: string,
  path: string,
  body: string,
  diagnostics: RepositoryDiagnostic[],
): Promise<void> => {
  const content = await readText(join(root, path));
  if (content === null || upsertManagedSection(content, body).status !== 'unchanged') {
    diagnostics.push({ severity: 'error', path, message: 'Arcantry managed section is missing or outdated.' });
  }
};

const validateGitExcludes = async (root: string, _config: ArcantryConfig, diagnostics: RepositoryDiagnostic[]): Promise<void> => {
  const path = await resolveGitPath(root, 'info/exclude');
  const lines = new Set(((await readText(path)) ?? '').split(/\r?\n/));
  for (const pattern of ['.local/']) {
    if (!lines.has(pattern)) {
      diagnostics.push({ severity: 'error', path: '.git/info/exclude', message: `${pattern} must be excluded locally.` });
    }
  }
};

const repairForDiagnostic = (diagnostic: RepositoryDiagnostic): string => {
  if (diagnostic.path === configPath) {
    return 'Run `arcantry repo init --docs none` to create the legacy private contract.';
  }
  if (diagnostic.message.includes('missing or outdated') || diagnostic.message.includes('excluded locally')) {
    return 'Run `arcantry repo update`.';
  }
  if (diagnostic.message.includes('incompatible')) {
    return 'Resolve the project-owned conflict, then run `arcantry repo update`.';
  }
  return 'Restore the required artifact, then run `arcantry repo validate`.';
};

const resolveGitPath = async (root: string, gitPath: string): Promise<string> => {
  const result = await execa('git', ['rev-parse', '--git-path', gitPath], { cwd: root, reject: true });
  return resolve(root, result.stdout.trim());
};

export const getOpenSpecAssetFiles = async (): Promise<Record<string, string>> => {
  const moduleDirectory = dirname(fileURLToPath(import.meta.url));
  const packageRoot = ['src', 'dist'].includes(basename(moduleDirectory)) ? dirname(moduleDirectory) : moduleDirectory;
  const candidates = [join(packageRoot, 'assets', 'openspec'), resolve(packageRoot, '..', '..', 'openspec', 'schemas', 'arcantry')];
  const assetRoot = (await firstDirectory(candidates)) ?? null;
  if (assetRoot === null) {
    throw new Error('Arcantry OpenSpec assets are missing from the package.');
  }

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
  for (const path of paths) {
    if ((await pathKind(path)) === 'directory') return path;
  }
  return undefined;
};

const hasTool = (content: string, tool: string): boolean => new RegExp(`^\\s*${tool}\\s*=\\s*["'][^"']+["']\\s*$`, 'm').test(content);

const hasToolMajor = (content: string, tool: string, major: number): boolean => {
  const match = content.match(new RegExp(`^\\s*${tool}\\s*=\\s*["']([^"']+)["']\\s*$`, 'm'));
  return match?.[1]?.split('.')[0] === String(major);
};

const resolvePlanPath = (root: string, path: string): string => (isAbsolute(path) ? path : join(root, path));

const readText = async (path: string): Promise<string | null> => {
  try {
    return await readFile(path, 'utf8');
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') {
      return null;
    }
    throw error;
  }
};

const pathExists = async (path: string): Promise<boolean> => {
  try {
    await lstat(path);
    return true;
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') {
      return false;
    }
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

import { lstat, readFile, readdir, readlink, realpath, rename, rm, symlink } from 'node:fs/promises';
import { homedir } from 'node:os';
import { basename, dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { validRange } from 'semver';
import { z } from 'zod';

const skillNamePattern = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;
export const skillFamilySchema = z.enum(['self-improvement', 'repo-safely', 'content-safely']);

export const catalogEntrySchema = z.object({
  name: z.string().regex(skillNamePattern).max(63),
  family: skillFamilySchema,
  tags: z.array(z.string().regex(skillNamePattern)).min(1),
}).strict();

export const catalogSchema = z.object({
  $schema: z.literal('./schemas/catalog.schema.json'),
  skills: z.array(catalogEntrySchema),
}).strict();

export const skillScenarioSchema = z.object({
  title: z.string().trim().min(5).max(60),
  prompt: z.string().trim().min(15).max(280),
  outcome: z.string().trim().min(15).max(220),
}).strict();

export const skillCompatibilitySchema = z.object({
  sourceKinds: z.array(z.enum(['openspec', 'changelog', 'todo-txt'])).min(1),
  adapters: z.array(z.object({
    name: z.string().regex(/^[a-z][a-z0-9-]*$/),
    versions: z.string().trim().refine((value) => validRange(value) !== null, 'Invalid adapter version range.'),
  }).strict()).optional(),
}).strict();

export const skillLearningSchema = z.object({
  prerequisites: z.array(z.string().trim().min(5).max(160)).min(1).optional(),
  outcomes: z.array(z.string().trim().min(5).max(160)).min(1),
}).strict();

export const skillMetadataSchema = z.object({
  $schema: z.literal('../../schemas/skill-metadata.schema.json'),
  summary: z.string().trim().min(30).max(180),
  scenarios: z.array(skillScenarioSchema).length(2),
  compatibility: skillCompatibilitySchema.optional(),
  learning: skillLearningSchema.optional(),
}).strict();

export type CatalogEntry = z.infer<typeof catalogEntrySchema>;
export type Catalog = z.infer<typeof catalogSchema>;
export type SkillMetadata = z.infer<typeof skillMetadataSchema>;
export type SkillInspection = {
  entry: CatalogEntry;
  metadata: SkillMetadata;
  directory: string;
  skillFile: string;
  agentFile: string;
};

export type PrivateSkillInspection = {
  name: string;
  description: string;
  directory: string;
  skillFile: string;
};

export type CatalogValidation = { valid: boolean; errors: string[]; catalog: Catalog | null };

export type SkillLinkResult = {
  status: 'linked' | 'unchanged' | 'unlinked';
  source: string;
  target: string;
  backup: string | null;
};

export type SkillLinkOptions = {
  catalogRoot?: string;
  sourceDirectory?: string;
  name: string;
  targetRoot?: string;
  replace?: boolean;
  backupLabel?: string;
};

export const userSkillTargetRoot = (): string => join(homedir(), '.agents', 'skills');
export const repositorySkillTargetRoot = (root: string): string => join(resolve(root), '.agents', 'skills');
export const userClaudeSkillTargetRoot = (): string => join(homedir(), '.claude', 'skills');
export const repositoryClaudeSkillTargetRoot = (root: string): string => join(resolve(root), '.claude', 'skills');
export const defaultSkillTargetRoot = userSkillTargetRoot;

export const privateSkillRoot = (root: string): string => join(resolve(root), '.local', 'skills');
export const privateSkillExists = async (root: string, name: string): Promise<boolean> =>
  (await pathKind(join(privateSkillRoot(root), name))) === 'directory';

export const loadCatalog = async (root: string): Promise<Catalog> => {
  const content = await readFile(join(root, 'catalog.json'), 'utf8');
  return catalogSchema.parse(JSON.parse(content));
};

export const inspectSkill = async (root: string, name: string): Promise<SkillInspection> => {
  const catalog = await loadCatalog(root);
  const entry = catalog.skills.find((candidate) => candidate.name === name);
  if (entry === undefined) {
    throw new Error(`Skill is not present in catalog.json: ${name}`);
  }

  const directory = join(root, 'skills', name);
  const metadata = skillMetadataSchema.parse(JSON.parse(await readFile(join(directory, 'arcantry.json'), 'utf8')));
  return {
    entry,
    metadata,
    directory,
    skillFile: join(directory, 'SKILL.md'),
    agentFile: join(directory, 'agents', 'openai.yaml'),
  };
};

export const inspectPrivateSkill = async (root: string, name: string): Promise<PrivateSkillInspection> => {
  if (!skillNamePattern.test(name)) throw new Error(`Invalid skill name: ${name}`);
  const directory = join(privateSkillRoot(root), name);
  const skillFile = join(directory, 'SKILL.md');
  if ((await pathKind(directory)) !== 'directory' || (await pathKind(skillFile)) !== 'file') {
    throw new Error(`Private skill is missing: .local/skills/${name}/SKILL.md`);
  }
  const frontmatter = readSkillFrontmatter(await readFile(skillFile, 'utf8'));
  if (frontmatter.name !== name) {
    throw new Error(`Private skill frontmatter name must match .local/skills/${name}.`);
  }
  return { name, description: frontmatter.description, directory, skillFile };
};

export const listPrivateSkills = async (root: string): Promise<PrivateSkillInspection[]> => {
  const directory = privateSkillRoot(root);
  if ((await pathKind(directory)) === 'missing') return [];
  if ((await pathKind(directory)) !== 'directory') throw new Error('.local/skills must be a directory.');
  const names = (await readdir(directory, { withFileTypes: true }))
    .filter((entry) => entry.isDirectory() || entry.isSymbolicLink())
    .map((entry) => entry.name)
    .sort((left, right) => left.localeCompare(right));
  return Promise.all(names.map(async (name) => inspectPrivateSkill(root, name)));
};

export const findCatalogRoot = async (start: string): Promise<string> => {
  let current = resolve(start);
  while (true) {
    if ((await pathKind(join(current, 'catalog.json'))) === 'file' && (await pathKind(join(current, 'skills'))) === 'directory') {
      return current;
    }
    const parent = dirname(current);
    if (parent === current) {
      break;
    }
    current = parent;
  }

  const packageRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
  if ((await pathKind(join(packageRoot, 'catalog.json'))) === 'file' && (await pathKind(join(packageRoot, 'skills'))) === 'directory') {
    return packageRoot;
  }
  throw new Error(`No Arcantry catalog found from ${start}. Use --catalog-root.`);
};

export const validateCatalog = async (root: string): Promise<CatalogValidation> => {
  const errors: string[] = [];
  let catalog: Catalog;
  try {
    catalog = await loadCatalog(root);
  } catch (error) {
    return { valid: false, errors: [formatError(error)], catalog: null };
  }

  const names = catalog.skills.map((entry) => entry.name);
  const summaries = new Map<string, string>();
  const descriptions = new Map<string, string>();
  const sortedNames = [...names].sort((left, right) => left.localeCompare(right));
  if (JSON.stringify(names) !== JSON.stringify(sortedNames)) {
    errors.push('catalog.json skills must be sorted by name.');
  }
  if (new Set(names).size !== names.length) {
    errors.push('catalog.json skill names must be unique.');
  }
  for (const entry of catalog.skills) {
    if (new Set(entry.tags).size !== entry.tags.length) {
      errors.push(`skills/${entry.name} tags must be unique.`);
    }
  }

  const skillDirectories = (await readdir(join(root, 'skills'), { withFileTypes: true }))
    .filter((entry) => entry.isDirectory())
    .map((entry) => entry.name)
    .sort((left, right) => left.localeCompare(right));
  if (JSON.stringify(skillDirectories) !== JSON.stringify(sortedNames)) {
    errors.push('catalog.json membership must exactly match skills/ directories.');
  }

  for (const name of names) {
    const directory = join(root, 'skills', name);
    const requiredFiles = ['SKILL.md', 'arcantry.json', join('agents', 'openai.yaml')];
    for (const file of requiredFiles) {
      if ((await pathKind(join(directory, file))) !== 'file') {
        errors.push(`skills/${name} is missing ${file}.`);
      }
    }

    try {
      const skillSource = await readFile(join(directory, 'SKILL.md'), 'utf8');
      const frontmatter = readSkillFrontmatter(skillSource);
      if (frontmatter.name !== name) {
        errors.push(`skills/${name} frontmatter name must match its directory.`);
      }
      if (frontmatter.description.length < 30) {
        errors.push(`skills/${name} description is too short.`);
      }
      const normalizedDescription = normalizeText(frontmatter.description);
      const existingDescription = descriptions.get(normalizedDescription);
      if (existingDescription !== undefined) {
        errors.push(`skills/${name} description duplicates skills/${existingDescription}.`);
      } else descriptions.set(normalizedDescription, name);
    } catch (error) {
      errors.push(`skills/${name}/SKILL.md: ${formatError(error)}`);
    }

    try {
      const metadata = skillMetadataSchema.parse(JSON.parse(await readFile(join(directory, 'arcantry.json'), 'utf8')));
      const normalizedSummary = normalizeText(metadata.summary);
      const existingSummary = summaries.get(normalizedSummary);
      if (existingSummary !== undefined) {
        errors.push(`skills/${name} summary duplicates skills/${existingSummary}.`);
      } else summaries.set(normalizedSummary, name);
    } catch (error) {
      errors.push(`skills/${name}/arcantry.json: ${formatError(error)}`);
    }

    try {
      const agentSource = await readFile(join(directory, 'agents', 'openai.yaml'), 'utf8');
      if (!agentSource.includes(`$${name}`)) {
        errors.push(`skills/${name}/agents/openai.yaml must mention $${name}.`);
      }
    } catch {
      // The required-file error is already actionable.
    }
  }

  return { valid: errors.length === 0, errors, catalog };
};

export const linkSkill = async (options: SkillLinkOptions): Promise<SkillLinkResult> => {
  const source = await resolveSkillSource(options);
  const targetRoot = resolve(options.targetRoot ?? defaultSkillTargetRoot());
  const target = join(targetRoot, options.name);
  const kind = await pathKind(target);

  if (kind === 'symlink') {
    if (await linkPointsTo(target, source)) {
      return { status: 'unchanged', source, target, backup: null };
    }
    if (options.replace !== true) {
      throw new Error(`${target} links to a different target. Use --replace to replace that link.`);
    }
    const backup = await nextBackupPath(target, options.backupLabel);
    await rename(target, backup);
    try {
      await createDirectoryLink(source, target);
    } catch (error) {
      await rename(backup, target);
      throw error;
    }
    return { status: 'linked', source, target, backup };
  }

  if (kind === 'file' || kind === 'directory') {
    if (options.replace !== true) {
      throw new Error(`${target} is not a link. Use --replace to back it up before linking.`);
    }
    const backup = await nextBackupPath(target, options.backupLabel);
    await rename(target, backup);
    try {
      await createDirectoryLink(source, target);
    } catch (error) {
      await rename(backup, target);
      throw error;
    }
    return { status: 'linked', source, target, backup };
  }

  await createDirectoryLink(source, target);
  return { status: 'linked', source, target, backup: null };
};

export const unlinkSkill = async (options: Omit<SkillLinkOptions, 'replace' | 'backupLabel'>): Promise<SkillLinkResult> => {
  const source = await resolveSkillSource(options);
  const target = join(resolve(options.targetRoot ?? defaultSkillTargetRoot()), options.name);

  const kind = await pathKind(target);
  if (kind === 'missing') return { status: 'unchanged', source, target, backup: null };
  if (kind !== 'symlink' || !(await linkPointsTo(target, source))) {
    throw new Error(`${target} is not an exact link to the selected Arcantry skill; nothing was removed.`);
  }
  await rm(target, { force: true });
  return { status: 'unlinked', source, target, backup: null };
};

export const linkSkillTargets = async (
  options: Omit<SkillLinkOptions, 'targetRoot'>,
  targetRoots: string[],
): Promise<SkillLinkResult[]> => {
  const roots = uniquePaths(targetRoots);
  await Promise.all(roots.map(async (targetRoot) => preflightLink({ ...options, targetRoot })));
  const results: SkillLinkResult[] = [];
  try {
    for (const targetRoot of roots) results.push(await linkSkill({ ...options, targetRoot }));
    return results;
  } catch (error) {
    for (const result of [...results].reverse()) await rollbackLink(result);
    throw error;
  }
};

export const unlinkSkillTargets = async (
  options: Omit<SkillLinkOptions, 'targetRoot' | 'replace' | 'backupLabel'>,
  targetRoots: string[],
): Promise<SkillLinkResult[]> => {
  const roots = uniquePaths(targetRoots);
  const source = await resolveSkillSource(options);
  for (const targetRoot of roots) {
    const target = join(resolve(targetRoot), options.name);
    const kind = await pathKind(target);
    if (kind !== 'missing' && (kind !== 'symlink' || !(await linkPointsTo(target, source)))) {
      throw new Error(`${target} is not an exact link to the selected Arcantry skill; nothing was removed.`);
    }
  }
  const results: SkillLinkResult[] = [];
  try {
    for (const targetRoot of roots) results.push(await unlinkSkill({ ...options, targetRoot }));
    return results;
  } catch (error) {
    for (const result of [...results].reverse()) {
      if (result.status === 'unlinked') await createDirectoryLink(result.source, result.target);
    }
    throw error;
  }
};

export const doctorSkills = async (
  root: string,
  targetRoots?: string | string[],
): Promise<{ valid: boolean; errors: string[]; warnings: string[] }> => {
  const validation = await validateCatalog(root);
  const warnings: string[] = [];
  if (validation.catalog !== null && targetRoots !== undefined) {
    for (const targetRoot of uniquePaths(Array.isArray(targetRoots) ? targetRoots : [targetRoots])) {
      for (const entry of validation.catalog.skills) {
        const source = join(root, 'skills', entry.name);
        const target = join(resolve(targetRoot), entry.name);
        const kind = await pathKind(target);
        if (kind === 'missing') {
          warnings.push(`${entry.name} is not linked in ${targetRoot}.`);
        } else if (kind !== 'symlink' || !(await linkPointsTo(target, source))) {
          warnings.push(`${entry.name} is not an exact Arcantry link in ${targetRoot}.`);
        }
      }
    }
  }
  return { valid: validation.valid, errors: validation.errors, warnings };
};

export const doctorPrivateSkills = async (
  root: string,
  targetRoots?: string[],
  publicNames: ReadonlySet<string> = new Set(),
): Promise<{ valid: boolean; errors: string[]; warnings: string[] }> => {
  const errors: string[] = [];
  const warnings: string[] = [];
  let skills: PrivateSkillInspection[] = [];
  try {
    skills = await listPrivateSkills(root);
  } catch (error) {
    errors.push(formatError(error));
  }
  for (const skill of skills) {
    if (publicNames.has(skill.name)) {
      errors.push(`Skill name conflict: ${skill.name} exists in both the public catalog and .local/skills.`);
    }
  }
  for (const targetRoot of uniquePaths(targetRoots ?? [])) {
    for (const skill of skills) {
      const target = join(targetRoot, skill.name);
      const kind = await pathKind(target);
      if (kind === 'missing') warnings.push(`${skill.name} is not linked in ${targetRoot}.`);
      else if (kind !== 'symlink' || !(await linkPointsTo(target, skill.directory))) {
        warnings.push(`${skill.name} is not an exact private skill link in ${targetRoot}.`);
      }
    }
  }
  return { valid: errors.length === 0, errors, warnings };
};

const createDirectoryLink = async (source: string, target: string): Promise<void> => {
  await import('node:fs/promises').then(({ mkdir }) => mkdir(dirname(target), { recursive: true }));
  await symlink(source, target, process.platform === 'win32' ? 'junction' : 'dir');
};

const resolveSkillSource = async (options: Pick<SkillLinkOptions, 'catalogRoot' | 'sourceDirectory' | 'name'>): Promise<string> => {
  if (options.sourceDirectory !== undefined) {
    const skillFile = join(options.sourceDirectory, 'SKILL.md');
    if ((await pathKind(skillFile)) !== 'file') throw new Error(`Skill package is missing SKILL.md: ${options.sourceDirectory}`);
    const frontmatter = readSkillFrontmatter(await readFile(skillFile, 'utf8'));
    if (frontmatter.name !== options.name) throw new Error(`Skill frontmatter name must match ${options.name}.`);
    return realpath(options.sourceDirectory);
  }
  if (options.catalogRoot === undefined) throw new Error('A catalog root or source directory is required.');
  return realpath((await inspectSkill(options.catalogRoot, options.name)).directory);
};

const preflightLink = async (options: SkillLinkOptions): Promise<void> => {
  const source = await resolveSkillSource(options);
  const target = join(resolve(options.targetRoot ?? defaultSkillTargetRoot()), options.name);
  const kind = await pathKind(target);
  if (kind === 'missing') return;
  if (kind === 'symlink' && await linkPointsTo(target, source)) return;
  if (options.replace !== true) {
    throw new Error(kind === 'symlink'
      ? `${target} links to a different target. Use --replace to replace that link.`
      : `${target} is not a link. Use --replace to back it up before linking.`);
  }
};

const rollbackLink = async (result: SkillLinkResult): Promise<void> => {
  if (result.status !== 'linked') return;
  await rm(result.target, { recursive: true, force: true });
  if (result.backup !== null) await rename(result.backup, result.target);
};

export const rollbackSkillLinks = async (results: SkillLinkResult[]): Promise<void> => {
  for (const result of [...results].reverse()) await rollbackLink(result);
};

const linkPointsTo = async (target: string, source: string): Promise<boolean> => {
  try {
    const rawTarget = await readlink(target);
    const resolvedTarget = resolve(dirname(target), rawTarget);
    const [targetRealPath, sourceRealPath] = await Promise.all([realpath(resolvedTarget), realpath(source)]);
    return normalizePath(targetRealPath) === normalizePath(sourceRealPath);
  } catch {
    return false;
  }
};

const nextBackupPath = async (target: string, label?: string): Promise<string> => {
  const safeLabel = label ?? new Date().toISOString().replace(/[:.]/g, '-');
  const base = `${target}.backup-${safeLabel}`;
  let candidate = base;
  let index = 2;
  while ((await pathKind(candidate)) !== 'missing') {
    candidate = `${base}-${index}`;
    index += 1;
  }
  return candidate;
};

const pathKind = async (path: string): Promise<'file' | 'directory' | 'symlink' | 'missing'> => {
  try {
    const stats = await lstat(path);
    if (stats.isSymbolicLink()) return 'symlink';
    if (stats.isDirectory()) return 'directory';
    return 'file';
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') return 'missing';
    throw error;
  }
};

const readSkillFrontmatter = (source: string): { name: string; description: string } => {
  const match = source.match(/^---\r?\n([\s\S]*?)\r?\n---/);
  if (match?.[1] === undefined) {
    throw new Error('Missing YAML frontmatter.');
  }
  const name = readFrontmatterScalar(match[1], 'name');
  const description = readFrontmatterScalar(match[1], 'description');
  return { name, description };
};

const readFrontmatterScalar = (frontmatter: string, key: string): string => {
  const match = frontmatter.match(new RegExp(`^${key}:\\s*(.+)$`, 'm'));
  if (match?.[1] === undefined) {
    throw new Error(`Missing ${key} in YAML frontmatter.`);
  }
  return match[1].trim().replace(/^(['"])(.*)\1$/, '$2');
};

const normalizePath = (path: string): string => (process.platform === 'win32' ? path.toLowerCase() : path);
const normalizeText = (value: string): string => value.trim().replace(/\s+/g, ' ').toLowerCase();
const uniquePaths = (paths: string[]): string[] => Array.from(
  new Map(paths.map((path) => [normalizePath(resolve(path)), resolve(path)])).values(),
);
const formatError = (error: unknown): string => (error instanceof Error ? error.message : String(error));

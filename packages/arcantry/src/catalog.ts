import { lstat, readFile, readdir, readlink, realpath, rename, rm, symlink } from 'node:fs/promises';
import { homedir } from 'node:os';
import { basename, dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { z } from 'zod';

const skillNamePattern = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;

export const catalogEntrySchema = z.object({
  name: z.string().regex(skillNamePattern),
  tags: z.array(z.string().trim().min(1)).min(1),
});

export const catalogSchema = z.object({
  $schema: z.string().optional(),
  skills: z.array(catalogEntrySchema),
});

export const skillScenarioSchema = z.object({
  title: z.string().trim().min(5).max(60),
  prompt: z.string().trim().min(15).max(280),
  outcome: z.string().trim().min(15).max(220),
});

export const skillMetadataSchema = z.object({
  $schema: z.string().optional(),
  summary: z.string().trim().min(30).max(180),
  scenarios: z.array(skillScenarioSchema).length(2),
});

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

export type CatalogValidation = { valid: boolean; errors: string[]; catalog: Catalog | null };

export type SkillLinkResult = {
  status: 'linked' | 'unchanged' | 'unlinked';
  source: string;
  target: string;
  backup: string | null;
};

export type SkillLinkOptions = {
  catalogRoot: string;
  name: string;
  targetRoot?: string;
  replace?: boolean;
  backupLabel?: string;
};

export const defaultSkillTargetRoot = (): string => join(process.env.CODEX_HOME ?? join(homedir(), '.codex'), 'skills');

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
    } catch (error) {
      errors.push(`skills/${name}/SKILL.md: ${formatError(error)}`);
    }

    try {
      skillMetadataSchema.parse(JSON.parse(await readFile(join(directory, 'arcantry.json'), 'utf8')));
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
  const inspection = await inspectSkill(options.catalogRoot, options.name);
  const source = await realpath(inspection.directory);
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
    await rm(target, { force: true });
    await createDirectoryLink(source, target);
    return { status: 'linked', source, target, backup: null };
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
  const inspection = await inspectSkill(options.catalogRoot, options.name);
  const source = await realpath(inspection.directory);
  const target = join(resolve(options.targetRoot ?? defaultSkillTargetRoot()), options.name);

  if ((await pathKind(target)) !== 'symlink' || !(await linkPointsTo(target, source))) {
    throw new Error(`${target} is not an exact link to the selected Arcantry skill; nothing was removed.`);
  }
  await rm(target, { force: true });
  return { status: 'unlinked', source, target, backup: null };
};

export const doctorSkills = async (root: string, targetRoot?: string): Promise<{ valid: boolean; errors: string[]; warnings: string[] }> => {
  const validation = await validateCatalog(root);
  const warnings: string[] = [];
  if (validation.catalog !== null && targetRoot !== undefined) {
    for (const entry of validation.catalog.skills) {
      const source = join(root, 'skills', entry.name);
      const target = join(resolve(targetRoot), entry.name);
      const kind = await pathKind(target);
      if (kind === 'missing') {
        warnings.push(`${entry.name} is not linked.`);
      } else if (kind !== 'symlink' || !(await linkPointsTo(target, source))) {
        warnings.push(`${entry.name} is not an exact Arcantry link.`);
      }
    }
  }
  return { valid: validation.valid, errors: validation.errors, warnings };
};

const createDirectoryLink = async (source: string, target: string): Promise<void> => {
  await import('node:fs/promises').then(({ mkdir }) => mkdir(dirname(target), { recursive: true }));
  await symlink(source, target, process.platform === 'win32' ? 'junction' : 'dir');
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
const formatError = (error: unknown): string => (error instanceof Error ? error.message : String(error));

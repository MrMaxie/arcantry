import { readFile, readdir, stat } from 'node:fs/promises';
import { isAbsolute, join, relative, resolve } from 'node:path';
import { cutoverChangelog, migrateChangelogToV2, renderKeepAChangelogV2 } from './changelog.js';
import { type KnowledgeInspection, type ProjectSource, adapterFor } from './knowledge.js';
import {
  createDeleteOperation,
  createDeleteTreeOperation,
  createProjectPlan,
  createWriteOperation,
  hashContent,
  hashPath,
  type PlanOperation,
  type ProjectPlan,
} from './projectPlan.js';
import { type Transition, transitionSchema, type Visibility } from './projectConfig.js';
import { getOpenSpecAssetFiles } from './repository.js';
import { planLocalGitExclude } from './privateState.js';

export type SourceTransitionOptions = {
  sourceId: string;
  transition: Transition;
  toolVersion: string;
  targetPath?: string;
  targetAdapter?: string;
  managedFrom?: string;
  deleteSource?: boolean;
};

export const planSourceTransition = async (
  inspection: KnowledgeInspection,
  options: SourceTransitionOptions,
): Promise<ProjectPlan> => {
  const transition = transitionSchema.parse(options.transition);
  const source = inspection.sources.find((candidate) => candidate.id === options.sourceId)
    ?? (transition === 'adopt' ? standardMissingSource(inspection.root, options.sourceId) : undefined);
  if (source === undefined) throw new Error(`Unknown source: ${options.sourceId}`);

  const operations: PlanOperation[] = [];
  const conflicts: string[] = [];
  const notes: string[] = [];
  let desiredPath = source.path;
  let desiredAdapter = options.targetAdapter ?? source.adapter;
  let desiredManagement = source.management;
  let desiredManagedFrom = source.managedFrom;

  if (transition === 'preserve') {
    notes.push('The source remains unchanged and keeps its current management policy.');
  } else if (transition === 'adopt') {
    desiredManagement = 'manage';
    if (adapterFor(source) === null) conflicts.push(`Adapter ${source.adapter} is not available for adoption.`);
    if (!source.exists && conflicts.length === 0) {
      await planSourceInitialization(inspection.root, source, operations);
      notes.push(`Missing ${source.kind} source will be initialized without project-language scaffolding.`);
    }
  } else if (transition === 'rebind') {
    desiredManagement = 'manage';
    if (options.targetPath === undefined) conflicts.push('Rebind requires --to-path.');
    else {
      desiredPath = options.targetPath;
      if (!(await sourcePathExists(inspection.root, desiredPath, source.kind))) {
        conflicts.push(`Rebind target does not contain a ${source.kind} source: ${desiredPath}.`);
      }
    }
  } else if (transition === 'cutover') {
    desiredManagement = 'manage';
    desiredAdapter = options.targetAdapter ?? 'keep-a-changelog@2';
    desiredManagedFrom = options.managedFrom;
    if (source.kind !== 'changelog') conflicts.push('Cutover is currently defined only for changelog sources.');
    if (!source.exists) conflicts.push(`Source ${source.id} does not exist.`);
    if (options.managedFrom === undefined) conflicts.push('Changelog cutover requires --managed-from <version>.');
    if (conflicts.length === 0) {
      try {
        const current = await readFile(source.absolutePath, 'utf8');
        const desired = cutoverChangelog(current, options.managedFrom!);
        if (desired !== current) operations.push(await createWriteOperation(inspection.root, source.path, desired, source.visibility));
      } catch (error) {
        conflicts.push(error instanceof Error ? error.message : String(error));
      }
    }
  } else if (transition === 'migrate') {
    desiredManagement = 'manage';
    desiredAdapter = options.targetAdapter ?? 'keep-a-changelog@2';
    desiredManagedFrom = undefined;
    if (source.kind !== 'changelog') conflicts.push('No semantic migration is available for this source kind.');
    if (!source.exists) conflicts.push(`Source ${source.id} does not exist.`);
    if (conflicts.length === 0) {
      try {
        const current = await readFile(source.absolutePath, 'utf8');
        const desired = migrateChangelogToV2(current);
        if (desired !== current) operations.push(await createWriteOperation(inspection.root, source.path, desired, source.visibility));
      } catch (error) {
        conflicts.push(error instanceof Error ? error.message : String(error));
      }
    }
  } else if (transition === 'relocate') {
    if (options.targetPath === undefined) conflicts.push('Relocate requires --to-path.');
    else if (resolveSourcePath(inspection.root, options.targetPath) === source.absolutePath) conflicts.push('Relocate target must differ from the source path.');
    else if (!source.exists) conflicts.push(`Source ${source.id} does not exist.`);
    else {
      desiredPath = options.targetPath;
      const target = resolveSourcePath(inspection.root, desiredPath);
      if (source.kind === 'openspec') {
        if (options.deleteSource === true && isWithin(source.absolutePath, target)) {
          conflicts.push('A relocated OpenSpec target cannot be inside a source tree that will be deleted.');
        } else {
          await planDirectoryRelocation(inspection.root, source, target, operations, conflicts, options.deleteSource === true);
        }
      } else {
        await planFileRelocation(inspection.root, source, target, operations, conflicts, options.deleteSource === true);
      }
    }
  }

  if (['adopt', 'rebind', 'cutover', 'migrate'].includes(transition) && adapterFor({ adapter: desiredAdapter, kind: source.kind }) === null) {
    conflicts.push(`Target adapter ${desiredAdapter} is not supported for ${transition}.`);
  }

  if (inspection.configPath !== null && source.origin === 'configured' && conflicts.length === 0 && transition !== 'preserve') {
    if (isAbsolute(desiredPath) && isWithin(inspection.root, inspection.configPath)) {
      conflicts.push('Embedded configuration cannot persist an absolute source path. Use an external --config file.');
    } else {
      const configContent = await readFile(inspection.configPath, 'utf8');
      const updated = patchConfiguredSource(configContent, source.id, {
        path: desiredPath,
        management: desiredManagement,
        adapter: desiredAdapter,
        managed_from: desiredManagedFrom,
      });
      if (updated !== configContent) {
        operations.push(
          await createWriteOperation(
            inspection.root,
            planPath(inspection.root, inspection.configPath),
            updated,
            configVisibility(inspection.root, inspection.configPath),
          ),
        );
      }
    }
  } else if (inspection.configPath === null && transition !== 'preserve') {
    notes.push('No configuration will be created; this transition leaves no Arcantry-specific project metadata.');
  }

  if (conflicts.length === 0 && operations.some((operation) => isLocalPlanPath(inspection.root, operation.path))) {
    await planLocalGitExclude(inspection.root, operations);
  }

  return createProjectPlan({
    toolVersion: options.toolVersion,
    root: inspection.root,
    sourceId: source.id,
    transition,
    adapter: source.adapter,
    targetAdapter: desiredAdapter,
    operations: conflicts.length === 0 ? operations : [],
    notes,
    conflicts,
  });
};

const standardMissingSource = (root: string, id: string): ProjectSource | undefined => {
  const source = {
    openspec: { kind: 'openspec', path: 'openspec', adapter: 'openspec@1', visibility: 'shared' },
    changelog: { kind: 'changelog', path: 'CHANGELOG.md', adapter: 'keep-a-changelog@2', visibility: 'shared' },
    'todo-root': { kind: 'todo-txt', path: 'todo.txt', adapter: 'todo-txt@1', visibility: 'shared' },
    'todo-local': { kind: 'todo-txt', path: '.local/todo.txt', adapter: 'todo-txt@1', visibility: 'private' },
  }[id] as Pick<ProjectSource, 'kind' | 'path' | 'adapter' | 'visibility'> | undefined;
  if (source === undefined) return undefined;
  return {
    id,
    ...source,
    absolutePath: resolve(root, source.path),
    management: 'observe',
    from: [],
    scope: '.',
    exists: false,
    origin: 'discovered',
    confidence: 'high',
    adapterStatus: 'supported',
  };
};

const planFileRelocation = async (
  root: string,
  source: ProjectSource,
  target: string,
  operations: PlanOperation[],
  conflicts: string[],
  deleteSource: boolean,
): Promise<void> => {
  const content = await readFile(source.absolutePath, 'utf8');
  const targetHash = await hashPath(target);
  const desiredHash = hashContent(content);
  if (targetHash !== null && targetHash !== desiredHash) {
    conflicts.push(`Relocate target already contains different content: ${planPath(root, target)}.`);
    return;
  }
  if (targetHash === null) operations.push(await createWriteOperation(root, planPath(root, target), content, source.visibility));
  if (deleteSource) operations.push(await createDeleteOperation(root, source.path, source.visibility));
};

const planSourceInitialization = async (
  root: string,
  source: ProjectSource,
  operations: PlanOperation[],
): Promise<void> => {
  if (source.kind === 'openspec') {
    const sourceRoot = resolveSourcePath(root, source.path);
    operations.push(
      await createWriteOperation(
        root,
        planPath(root, join(sourceRoot, 'config.yaml')),
        [
          '# arcantry:generated',
          'schema: arcantry',
          '',
          'context: |',
          '  OpenSpec is the only source of product and engineering specifications in this project.',
          '',
        ].join('\n'),
        source.visibility,
      ),
    );
    const files = await getOpenSpecAssetFiles();
    for (const [path, content] of Object.entries(files)) {
      const relativeAsset = path.replace(/^openspec\//, '');
      operations.push(
        await createWriteOperation(root, planPath(root, join(sourceRoot, relativeAsset)), content, source.visibility),
      );
    }
    return;
  }
  if (source.kind === 'changelog') {
    operations.push(
      await createWriteOperation(root, source.path, renderKeepAChangelogV2({ releases: [] }), source.visibility),
    );
    return;
  }
  operations.push(await createWriteOperation(root, source.path, '', source.visibility));
};

const planDirectoryRelocation = async (
  root: string,
  source: ProjectSource,
  target: string,
  operations: PlanOperation[],
  conflicts: string[],
  deleteSource: boolean,
): Promise<void> => {
  for (const absoluteSourceFile of await listFiles(source.absolutePath)) {
    const child = relative(source.absolutePath, absoluteSourceFile);
    const absoluteTargetFile = join(target, child);
    const content = await readFile(absoluteSourceFile, 'utf8');
    const targetHash = await hashPath(absoluteTargetFile);
    const desiredHash = hashContent(content);
    if (targetHash !== null && targetHash !== desiredHash) {
      conflicts.push(`Relocate target already contains different content: ${planPath(root, absoluteTargetFile)}.`);
      continue;
    }
    if (targetHash === null) {
      operations.push(await createWriteOperation(root, planPath(root, absoluteTargetFile), content, source.visibility));
    }
  }
  if (deleteSource && conflicts.length === 0) {
    operations.push(await createDeleteTreeOperation(root, source.path, source.visibility));
  }
};

const patchConfiguredSource = (
  content: string,
  id: string,
  updates: { path: string; management: string; adapter: string; managed_from?: string },
): string => {
  const newline = content.includes('\r\n') ? '\r\n' : '\n';
  const lines = content.split(/\r?\n/);
  const escaped = id.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const start = lines.findIndex((line) => new RegExp(`^\\[sources\\.${escaped}\\]\\s*$`).test(line));
  if (start < 0) throw new Error(`Configured source table is missing: ${id}.`);
  const next = lines.findIndex((line, index) => index > start && /^\s*\[/.test(line));
  let end = next < 0 ? lines.length : next;

  const setValue = (key: string, value: string | undefined): void => {
    const index = lines.findIndex((line, lineIndex) => lineIndex > start && lineIndex < end && new RegExp(`^\\s*${key}\\s*=`).test(line));
    if (value === undefined) {
      if (index >= 0) {
        lines.splice(index, 1);
        end -= 1;
      }
      return;
    }
    const rendered = `${key} = ${JSON.stringify(value)}`;
    if (index >= 0) lines[index] = rendered;
    else {
      lines.splice(end, 0, rendered);
      end += 1;
    }
  };

  setValue('path', updates.path);
  setValue('management', updates.management);
  setValue('adapter', updates.adapter);
  setValue('managed_from', updates.managed_from);
  return lines.join(newline);
};

const sourcePathExists = async (root: string, path: string, kind: ProjectSource['kind']): Promise<boolean> => {
  try {
    const value = await stat(resolveSourcePath(root, path));
    return kind === 'openspec' ? value.isDirectory() : value.isFile();
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') return false;
    throw error;
  }
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

const resolveSourcePath = (root: string, path: string): string => (isAbsolute(path) ? resolve(path) : resolve(root, path));

const planPath = (root: string, path: string): string => {
  const relativePath = relative(root, path);
  return relativePath === '' || relativePath.startsWith('..') || isAbsolute(relativePath)
    ? resolve(path)
    : relativePath.replaceAll('\\', '/');
};

const configVisibility = (root: string, path: string): Visibility =>
  !isWithin(root, path) || planPath(root, path).startsWith('.local/') ? 'private' : 'shared';

const isWithin = (parent: string, child: string): boolean => {
  const value = relative(resolve(parent), resolve(child));
  return value === '' || (!value.startsWith('..') && !isAbsolute(value));
};

const isLocalPlanPath = (root: string, path: string): boolean => {
  const value = planPath(root, resolveSourcePath(root, path));
  return value === '.local' || value.startsWith('.local/');
};

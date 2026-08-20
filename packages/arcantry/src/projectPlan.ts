import { createHash, randomUUID } from 'node:crypto';
import { copyFile, mkdir, mkdtemp, readFile, readdir, realpath, rename, rm, stat, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { dirname, isAbsolute, join, relative, resolve } from 'node:path';
import { z } from 'zod';
import { transitionSchema, visibilitySchema, type Transition, type Visibility } from './projectConfig.js';

export const projectPlanVersion = 1 as const;

const planOperationSchema = z.object({
  action: z.enum(['write', 'delete', 'delete-tree']),
  path: z.string().trim().min(1),
  expectedHash: z.string().length(64).nullable(),
  content: z.string().optional(),
  contentHash: z.string().length(64).optional(),
  visibility: visibilitySchema,
});

const projectPlanSchema = z
  .object({
    planVersion: z.literal(projectPlanVersion),
    toolVersion: z.string().trim().min(1),
    root: z.string().trim().min(1),
    sourceId: z.string().trim().min(1),
    transition: transitionSchema,
    adapter: z.string().trim().min(1),
    targetAdapter: z.string().trim().min(1).optional(),
    operations: z.array(planOperationSchema),
    notes: z.array(z.string()),
    conflicts: z.array(z.string()),
  })
  .superRefine((plan, context) => {
    const paths = new Set<string>();
    for (const [index, operation] of plan.operations.entries()) {
      if (!isAbsolute(operation.path) && !isWithin(plan.root, resolve(plan.root, operation.path))) {
        context.addIssue({ code: 'custom', message: `plan operation path must stay within the project: ${operation.path}`, path: ['operations', index, 'path'] });
      }
      if (paths.has(operation.path)) {
        context.addIssue({ code: 'custom', message: `plan contains duplicate operation path: ${operation.path}`, path: ['operations', index, 'path'] });
      }
      paths.add(operation.path);
      if (operation.action === 'write' && (operation.content === undefined || operation.contentHash === undefined)) {
        context.addIssue({ code: 'custom', message: 'write operations require content and contentHash.', path: ['operations', index] });
      }
      if (operation.action !== 'write' && (operation.content !== undefined || operation.contentHash !== undefined)) {
        context.addIssue({ code: 'custom', message: 'delete operations cannot contain content.', path: ['operations', index] });
      }
    }
  });

export type PlanOperation = z.infer<typeof planOperationSchema>;
export type ProjectPlan = z.infer<typeof projectPlanSchema>;

export const createProjectPlan = (input: {
  toolVersion: string;
  root: string;
  sourceId: string;
  transition: Transition;
  adapter: string;
  targetAdapter?: string;
  operations?: PlanOperation[];
  notes?: string[];
  conflicts?: string[];
}): ProjectPlan =>
  projectPlanSchema.parse({
    planVersion: projectPlanVersion,
    toolVersion: input.toolVersion,
    root: resolve(input.root),
    sourceId: input.sourceId,
    transition: input.transition,
    adapter: input.adapter,
    ...(input.targetAdapter === undefined ? {} : { targetAdapter: input.targetAdapter }),
    operations: input.operations ?? [],
    notes: input.notes ?? [],
    conflicts: input.conflicts ?? [],
  });

export const createWriteOperation = async (
  root: string,
  path: string,
  content: string,
  visibility: Visibility,
): Promise<PlanOperation> => ({
  action: 'write',
  path,
  expectedHash: await hashPath(resolvePlanPath(root, path)),
  content,
  contentHash: hashContent(content),
  visibility,
});

export const createDeleteOperation = async (
  root: string,
  path: string,
  visibility: Visibility,
): Promise<PlanOperation> => ({
  action: 'delete',
  path,
  expectedHash: await hashPath(resolvePlanPath(root, path)),
  visibility,
});

export const createDeleteTreeOperation = async (
  root: string,
  path: string,
  visibility: Visibility,
): Promise<PlanOperation> => {
  const expectedHash = await hashPath(resolvePlanPath(root, path));
  if (expectedHash === null) throw new Error(`Cannot delete missing source tree: ${path}`);
  return { action: 'delete-tree', path, expectedHash, visibility };
};

export const serializeProjectPlan = (plan: ProjectPlan): string => `${JSON.stringify(projectPlanSchema.parse(plan), null, 2)}\n`;

export const parseProjectPlan = (content: string): ProjectPlan => projectPlanSchema.parse(JSON.parse(content));

export const renderProjectPlan = (plan: ProjectPlan): string => {
  const lines = [
    `Transition: ${plan.transition}`,
    `Source: ${plan.sourceId}`,
    `Adapter: ${plan.adapter}${plan.targetAdapter === undefined ? '' : ` -> ${plan.targetAdapter}`}`,
  ];
  for (const operation of plan.operations) {
    lines.push(`${operation.action}: ${operation.path}${operation.visibility === 'private' ? ' (private)' : ''}`);
  }
  for (const note of plan.notes) lines.push(`Note: ${note}`);
  for (const conflict of plan.conflicts) lines.push(`Conflict: ${conflict}`);
  if (plan.operations.length === 0 && plan.conflicts.length === 0) lines.push('No file changes.');
  return `${lines.join('\n')}\n`;
};

export const applyProjectPlan = async (plan: ProjectPlan, toolVersion: string): Promise<{ applied: PlanOperation[] }> => {
  const validated = projectPlanSchema.parse(plan);
  if (validated.toolVersion !== toolVersion) {
    throw new Error(`Plan requires Arcantry ${validated.toolVersion}; current version is ${toolVersion}.`);
  }
  if (validated.conflicts.length > 0) throw new Error(`Cannot apply plan: ${validated.conflicts.join('; ')}`);

  for (const operation of validated.operations) {
    await ensurePlanPathBoundary(validated.root, operation.path);
    const currentHash = await hashPath(resolvePlanPath(validated.root, operation.path));
    if (currentHash !== operation.expectedHash) {
      throw new Error(`Refusing to change ${operation.path}; it changed after the plan was created.`);
    }
    if (operation.action === 'write' && hashContent(operation.content!) !== operation.contentHash) {
      throw new Error(`Refusing to apply ${operation.path}; planned content is corrupt.`);
    }
  }

  const stagingRoot = await mkdtemp(join(tmpdir(), 'arcantry-apply-'));
  const prepared = new Map<number, string>();
  try {
    for (const [index, operation] of validated.operations.entries()) {
      if (operation.action !== 'write') continue;
      const temporary = join(stagingRoot, `${index}.tmp`);
      await writeFile(temporary, operation.content!, 'utf8');
      if ((await hashPath(temporary)) !== operation.contentHash) throw new Error(`Could not prepare ${operation.path}.`);
      prepared.set(index, temporary);
    }

    const staged: Array<{ operation: PlanOperation; target: string; staged?: string; backup?: string; committed?: boolean }> = [];
    try {
      for (const [index, operation] of validated.operations.entries()) {
        const target = resolvePlanPath(validated.root, operation.path);
        await mkdir(dirname(target), { recursive: true });
        if (operation.action === 'write') {
          const temporary = `${target}.arcantry-${randomUUID()}.tmp`;
          await copyFile(prepared.get(index)!, temporary);
          const item = { operation, target, staged: temporary };
          staged.push(item);
          if ((await hashPath(temporary)) !== operation.contentHash) throw new Error(`Could not stage ${operation.path}.`);
        } else {
          staged.push({ operation, target });
        }
      }

      for (const item of staged) {
        if ((await hashPath(item.target)) !== item.operation.expectedHash) {
          throw new Error(`Refusing to change ${item.operation.path}; it changed during apply.`);
        }
        if (item.operation.expectedHash !== null) {
          const backup = `${item.target}.arcantry-${randomUUID()}.bak`;
          await rename(item.target, backup);
          item.backup = backup;
        }
        if (item.operation.action === 'write') await rename(item.staged!, item.target);
        item.committed = true;
      }

      for (const item of staged) {
        const expected = item.operation.action === 'write' ? item.operation.contentHash! : null;
        if ((await hashPath(item.target)) !== expected) throw new Error(`Verification failed for ${item.operation.path}.`);
      }

      await Promise.all(
        staged.flatMap((item) =>
          item.backup === undefined ? [] : [rm(item.backup, { force: true, recursive: item.operation.action === 'delete-tree' })],
        ),
      );
      return { applied: validated.operations };
    } catch (error) {
      for (const item of [...staged].reverse()) {
        if (item.staged !== undefined) await rm(item.staged, { force: true }).catch(() => undefined);
        if (item.backup !== undefined) {
          await rm(item.target, { force: true, recursive: item.operation.action === 'delete-tree' }).catch(() => undefined);
          await rename(item.backup, item.target).catch(() => undefined);
        } else if (item.committed === true && item.operation.action === 'write') {
          await rm(item.target, { force: true }).catch(() => undefined);
        }
      }
      throw error;
    }
  } finally {
    await rm(stagingRoot, { force: true, recursive: true });
  }
};

export const hashContent = (content: string): string => createHash('sha256').update(content, 'utf8').digest('hex');

export const hashPath = async (path: string): Promise<string | null> => {
  try {
    const value = await stat(path);
    if (value.isFile()) return createHash('sha256').update(await readFile(path)).digest('hex');
    if (value.isDirectory()) {
      const hash = createHash('sha256');
      for (const entry of (await readdir(path, { withFileTypes: true })).sort((left, right) => left.name.localeCompare(right.name))) {
        hash.update(entry.isDirectory() ? 'directory\0' : 'file\0').update(entry.name).update('\0');
        hash.update((await hashPath(join(path, entry.name))) ?? 'missing').update('\0');
      }
      return hash.digest('hex');
    }
    throw new Error(`Unsupported plan input type: ${path}`);
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') return null;
    throw error;
  }
};

export const resolvePlanPath = (root: string, path: string): string => (isAbsolute(path) ? resolve(path) : resolve(root, path));

const isWithin = (parent: string, child: string): boolean => {
  const value = relative(resolve(parent), resolve(child));
  return value === '' || (!value.startsWith('..') && !isAbsolute(value));
};

const ensurePlanPathBoundary = async (root: string, path: string): Promise<void> => {
  if (isAbsolute(path)) return;
  const canonicalRoot = await realpath(root);
  let existing = resolvePlanPath(root, path);
  while (true) {
    try {
      const canonicalExisting = await realpath(existing);
      if (!isWithin(canonicalRoot, canonicalExisting)) {
        throw new Error(`plan operation path resolves outside the project: ${path}`);
      }
      return;
    } catch (error) {
      if (!(error instanceof Error && 'code' in error && error.code === 'ENOENT')) throw error;
      const parent = dirname(existing);
      if (parent === existing) throw new Error(`Plan operation has no existing project ancestor: ${path}`);
      existing = parent;
    }
  }
};

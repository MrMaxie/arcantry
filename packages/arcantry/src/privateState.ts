import { readFile } from 'node:fs/promises';
import { isAbsolute, relative, resolve } from 'node:path';
import { execa } from 'execa';
import { createWriteOperation, type PlanOperation } from './projectPlan.js';

export const planLocalGitExclude = async (root: string, operations: PlanOperation[]): Promise<void> => {
  let excludePath: string;
  try {
    const result = await execa('git', ['rev-parse', '--git-path', 'info/exclude'], { cwd: root });
    excludePath = resolve(root, result.stdout.trim());
  } catch {
    return;
  }

  let content = '';
  try {
    content = await readFile(excludePath, 'utf8');
  } catch (error) {
    if (!(error instanceof Error && 'code' in error && error.code === 'ENOENT')) throw error;
  }
  if (new Set(content.split(/\r?\n/)).has('.local/')) return;
  const separator = content.length > 0 && !content.endsWith('\n') ? '\n' : '';
  operations.push(
    await createWriteOperation(root, planPath(root, excludePath), `${content}${separator}.local/\n`, 'private'),
  );
};

const planPath = (root: string, path: string): string => {
  const value = relative(root, path);
  return value !== '' && !value.startsWith('..') && !isAbsolute(value) ? value.replaceAll('\\', '/') : resolve(path);
};

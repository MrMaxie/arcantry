import { readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import {
  applyProjectPlan,
  createDeleteOperation,
  createProjectPlan,
  createWriteOperation,
  parseProjectPlan,
  renderProjectPlan,
  serializeProjectPlan,
} from './projectPlan.js';
import { createFixtureDirectory, removeFixtures } from './testHelpers.js';

afterEach(removeFixtures);

describe('project transition plans', () => {
  it('serializes a versioned, replayable plan', async () => {
    const root = await createFixtureDirectory('arcantry-plan-');
    const operation = await createWriteOperation(root, 'todo.txt', 'Add tests +Arcantry\n', 'shared');
    const plan = createProjectPlan({
      toolVersion: '0.3.2',
      root,
      sourceId: 'todo-root',
      transition: 'adopt',
      adapter: 'todo-txt@1',
      operations: [operation],
    });

    expect(parseProjectPlan(serializeProjectPlan(plan))).toEqual(plan);
  });

  it('rejects all writes when an input changes after planning', async () => {
    const root = await createFixtureDirectory('arcantry-drift-');
    await writeFile(join(root, 'source.txt'), 'before\n');
    const operations = [
      await createWriteOperation(root, 'target.txt', 'copy\n', 'shared'),
      await createDeleteOperation(root, 'source.txt', 'shared'),
    ];
    const plan = createProjectPlan({
      toolVersion: '0.3.2',
      root,
      sourceId: 'source',
      transition: 'relocate',
      adapter: 'todo-txt@1',
      operations,
    });
    await writeFile(join(root, 'source.txt'), 'changed\n');

    await expect(applyProjectPlan(plan, '0.3.2')).rejects.toThrow('changed after the plan');
    await expect(readFile(join(root, 'target.txt'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
    expect(await readFile(join(root, 'source.txt'), 'utf8')).toBe('changed\n');
  });

  it('writes and verifies a relocation target before removing the source', async () => {
    const root = await createFixtureDirectory('arcantry-relocate-');
    await writeFile(join(root, 'todo.txt'), 'Move me @desk\n');
    const content = await readFile(join(root, 'todo.txt'), 'utf8');
    const plan = createProjectPlan({
      toolVersion: '0.3.2',
      root,
      sourceId: 'todo-root',
      transition: 'relocate',
      adapter: 'todo-txt@1',
      operations: [
        await createWriteOperation(root, '.local/todo.txt', content, 'private'),
        await createDeleteOperation(root, 'todo.txt', 'shared'),
      ],
    });

    await applyProjectPlan(plan, '0.3.2');

    expect(await readFile(join(root, '.local/todo.txt'), 'utf8')).toBe(content);
    await expect(readFile(join(root, 'todo.txt'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('preserves the relocation source when target verification fails', async () => {
    const root = await createFixtureDirectory('arcantry-relocate-failure-');
    await writeFile(join(root, 'todo.txt'), 'Do not lose me\n');
    const plan = createProjectPlan({
      toolVersion: '0.3.2',
      root,
      sourceId: 'todo-root',
      transition: 'relocate',
      adapter: 'todo-txt@1',
      operations: [
        await createWriteOperation(root, '.local/todo.txt', 'Do not lose me\n', 'private'),
        await createDeleteOperation(root, 'todo.txt', 'shared'),
      ],
    });
    plan.operations[0]!.contentHash = '0'.repeat(64);

    await expect(applyProjectPlan(plan, '0.3.2')).rejects.toThrow('planned content is corrupt');
    expect(await readFile(join(root, 'todo.txt'), 'utf8')).toBe('Do not lose me\n');
    await expect(readFile(join(root, '.local/todo.txt'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('keeps private content out of the human plan', async () => {
    const root = await createFixtureDirectory('arcantry-private-plan-');
    const secret = 'private task details\n';
    const plan = createProjectPlan({
      toolVersion: '0.3.2',
      root,
      sourceId: 'todo-local',
      transition: 'adopt',
      adapter: 'todo-txt@1',
      operations: [await createWriteOperation(root, '.local/todo.txt', secret, 'private')],
    });

    expect(renderProjectPlan(plan)).not.toContain(secret.trim());
    expect(renderProjectPlan(plan)).toContain('.local/todo.txt (private)');
  });

  it('rejects relative operation paths that escape the project', async () => {
    const root = await createFixtureDirectory('arcantry-plan-boundary-');
    expect(() => createProjectPlan({
      toolVersion: '0.3.2',
      root,
      sourceId: 'todo-root',
      transition: 'adopt',
      adapter: 'todo-txt@1',
      operations: [{
        action: 'write',
        path: 'nested/../../outside.txt',
        expectedHash: null,
        content: 'outside\n',
        contentHash: '0'.repeat(64),
        visibility: 'shared',
      }],
    })).toThrow('plan operation path must stay within the project');
  });
});

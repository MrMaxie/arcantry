import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { inspectKnowledge } from './knowledge.js';
import { applyProjectPlan } from './projectPlan.js';
import { parseProjectConfig, type ResolvedProject } from './projectConfig.js';
import { planSourceTransition } from './sourceTransition.js';
import { createFixtureDirectory, removeFixtures } from './testHelpers.js';

afterEach(removeFixtures);

const configuredProject = async (root: string, source: string): Promise<ResolvedProject> => {
  const configPath = join(root, 'arcantry.toml');
  await writeFile(configPath, source);
  return { root, configPath, config: parseProjectConfig(source), mode: 'configured' };
};

describe('source transition planning', () => {
  it('preserves a source without writing configuration or content', async () => {
    const root = await createFixtureDirectory('arcantry-preserve-');
    await writeFile(join(root, 'todo.txt'), 'Keep me\n');
    const inspection = await inspectKnowledge({ root, configPath: null, config: null, mode: 'wild' });

    const plan = await planSourceTransition(inspection, {
      sourceId: 'todo-root',
      transition: 'preserve',
      toolVersion: '0.3.2',
    });

    expect(plan.operations).toEqual([]);
    expect(plan.conflicts).toEqual([]);
  });

  it('initializes only selected missing sources in a non-Git project', async () => {
    const root = await createFixtureDirectory('arcantry-initialize-');
    const project = await configuredProject(root, `config_version = 1

[sources.intent]
kind = "openspec"
path = "openspec"
management = "observe"
adapter = "openspec@1"
`);
    const plan = await planSourceTransition(await inspectKnowledge(project), {
      sourceId: 'intent',
      transition: 'adopt',
      toolVersion: '0.3.2',
    });

    await applyProjectPlan(plan, '0.3.2');

    expect(await readFile(join(root, 'openspec', 'config.yaml'), 'utf8')).toContain('schema: arcantry');
    await expect(readFile(join(root, 'package.json'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
    await expect(readFile(join(root, 'justfile'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('adopts a standard source in an empty configuration-free project', async () => {
    const root = await createFixtureDirectory('arcantry-wild-adopt-');
    const inspection = await inspectKnowledge({ root, configPath: null, config: null, mode: 'wild' });

    const plan = await planSourceTransition(inspection, {
      sourceId: 'todo-root',
      transition: 'adopt',
      toolVersion: '0.3.2',
    });
    await applyProjectPlan(plan, '0.3.2');

    expect(await readFile(join(root, 'todo.txt'), 'utf8')).toBe('');
    await expect(readFile(join(root, 'arcantry.toml'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('cuts over a configured changelog and persists its adapter boundary', async () => {
    const root = await createFixtureDirectory('arcantry-cutover-');
    const legacyTail = '## [0.9.0] - 2026-07-01\n\n### Added\n\n- Legacy\n';
    await writeFile(join(root, 'CHANGELOG.md'), `# Changelog\n\n## [Unreleased]\n\n${legacyTail}`);
    await mkdir(join(root, 'openspec'));
    const project = await configuredProject(root, `config_version = 1

[sources.intent]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.history]
kind = "changelog"
path = "CHANGELOG.md"
management = "observe"
adapter = "keep-a-changelog@1"
from = ["intent"]
`);
    const plan = await planSourceTransition(await inspectKnowledge(project), {
      sourceId: 'history',
      transition: 'cutover',
      managedFrom: '1.0.0',
      toolVersion: '0.3.2',
    });

    await applyProjectPlan(plan, '0.3.2');

    expect(await readFile(join(root, 'CHANGELOG.md'), 'utf8')).toMatch(new RegExp(`${legacyTail.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')}$`));
    expect(await readFile(join(root, 'arcantry.toml'), 'utf8')).toContain('adapter = "keep-a-changelog@2"');
    expect(await readFile(join(root, 'arcantry.toml'), 'utf8')).toContain('managed_from = "1.0.0"');
  });

  it('blocks ambiguous full changelog migration without producing writes', async () => {
    const root = await createFixtureDirectory('arcantry-migrate-');
    await writeFile(join(root, 'CHANGELOG.md'), '# History\n\nFree-form meaning\n');
    const inspection = await inspectKnowledge({ root, configPath: null, config: null, mode: 'wild' });

    const plan = await planSourceTransition(inspection, {
      sourceId: 'changelog',
      transition: 'migrate',
      toolVersion: '0.3.2',
    });

    expect(plan.operations).toEqual([]);
    expect(plan.conflicts).toContain('Full changelog migration requires an identified Keep a Changelog 1.x source.');
  });

  it('blocks an unsupported target adapter before producing writes', async () => {
    const root = await createFixtureDirectory('arcantry-target-adapter-');
    await writeFile(join(root, 'CHANGELOG.md'), '# Changelog\n\nBased on https://keepachangelog.com/en/1.1.0/\n\n## [Unreleased]\n');
    const inspection = await inspectKnowledge({ root, configPath: null, config: null, mode: 'wild' });

    const plan = await planSourceTransition(inspection, {
      sourceId: 'changelog',
      transition: 'migrate',
      targetAdapter: 'keep-a-changelog@9',
      toolVersion: '0.3.2',
    });

    expect(plan.operations).toEqual([]);
    expect(plan.conflicts).toContain('Target adapter keep-a-changelog@9 is not supported for migrate.');
  });

  it('relocates an OpenSpec tree only after target conflicts are excluded', async () => {
    const root = await createFixtureDirectory('arcantry-tree-');
    await mkdir(join(root, 'openspec', 'specs'), { recursive: true });
    await writeFile(join(root, 'openspec', 'specs', 'one.md'), 'one\n');
    const inspection = await inspectKnowledge({ root, configPath: null, config: null, mode: 'wild' });
    const plan = await planSourceTransition(inspection, {
      sourceId: 'openspec',
      transition: 'relocate',
      targetPath: 'contract',
      deleteSource: true,
      toolVersion: '0.3.2',
    });

    await applyProjectPlan(plan, '0.3.2');

    expect(await readFile(join(root, 'contract', 'specs', 'one.md'), 'utf8')).toBe('one\n');
    await expect(readFile(join(root, 'openspec', 'specs', 'one.md'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('leaves unrelated local changes untouched', async () => {
    const root = await createFixtureDirectory('arcantry-unrelated-');
    await writeFile(join(root, 'todo.txt'), 'Move me\n');
    await writeFile(join(root, 'unrelated.txt'), 'user change\n');
    const inspection = await inspectKnowledge({ root, configPath: null, config: null, mode: 'wild' });
    const plan = await planSourceTransition(inspection, {
      sourceId: 'todo-root',
      transition: 'relocate',
      targetPath: '.local/todo.txt',
      deleteSource: true,
      toolVersion: '0.3.2',
    });

    await applyProjectPlan(plan, '0.3.2');

    expect(await readFile(join(root, 'unrelated.txt'), 'utf8')).toBe('user change\n');
  });
});

import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { inspectKnowledge } from './knowledge.js';
import { parseProjectConfig, type ResolvedProject } from './projectConfig.js';
import { createFixtureDirectory, removeFixtures } from './testHelpers.js';

afterEach(removeFixtures);

const wild = (root: string): ResolvedProject => ({
  root,
  configPath: null,
  config: null,
  mode: 'wild',
  scope: null,
  shadowedConfigPaths: [],
});

describe('knowledge source discovery', () => {
  it('accepts an empty non-Git project without creating a source', async () => {
    const root = await createFixtureDirectory('arcantry-empty-');

    await expect(inspectKnowledge(wild(root))).resolves.toMatchObject({ mode: 'wild', sources: [], diagnostics: [] });
  });

  it('discovers OpenSpec, both todo queues and a Keep a Changelog 2 file independently', async () => {
    const root = await createFixtureDirectory('arcantry-discovery-');
    await mkdir(join(root, 'openspec'));
    await mkdir(join(root, '.local'));
    await writeFile(join(root, 'todo.txt'), '(A) Ship +Arcantry @desk\n');
    await writeFile(join(root, '.local/todo.txt'), 'Private note custom:value\n');
    await writeFile(
      join(root, 'CHANGELOG.md'),
      '# Changelog\n\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/).\n\n## [Unreleased]\n',
    );

    const inspection = await inspectKnowledge(wild(root));

    expect(inspection.sources.map(({ id, adapter, management, visibility }) => ({ id, adapter, management, visibility }))).toEqual([
      { id: 'changelog', adapter: 'keep-a-changelog@2', management: 'observe', visibility: 'shared' },
      { id: 'openspec', adapter: 'openspec@1', management: 'observe', visibility: 'shared' },
      { id: 'todo-local', adapter: 'todo-txt@1', management: 'observe', visibility: 'private' },
      { id: 'todo-root', adapter: 'todo-txt@1', management: 'observe', visibility: 'shared' },
    ]);
  });

  it('observes a changelog without inventing an OpenSpec authority', async () => {
    const root = await createFixtureDirectory('arcantry-changelog-only-');
    await writeFile(join(root, 'CHANGELOG.md'), '# Changelog\n\n## [Unreleased]\n');

    const inspection = await inspectKnowledge(wild(root));

    expect(inspection.sources).toEqual([expect.objectContaining({ id: 'changelog', management: 'observe', from: [] })]);
  });

  it('observes OpenSpec when no changelog exists', async () => {
    const root = await createFixtureDirectory('arcantry-openspec-only-');
    await mkdir(join(root, 'openspec'));

    const inspection = await inspectKnowledge(wild(root));

    expect(inspection.sources).toEqual([expect.objectContaining({ id: 'openspec', adapter: 'openspec@1' })]);
  });

  it('keeps configured sources and observes unconfigured artifacts', async () => {
    const root = await createFixtureDirectory('arcantry-partial-');
    await mkdir(join(root, 'openspec'));
    await writeFile(join(root, 'todo.txt'), 'Inspect +Project @desk\n');
    const config = parseProjectConfig(`config_version = 1

[sources.intent]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"
`);

    const inspection = await inspectKnowledge({ root, configPath: join(root, 'arcantry.toml'), config, mode: 'configured' });

    expect(inspection.sources).toEqual([
      expect.objectContaining({ id: 'intent', origin: 'configured', management: 'manage' }),
      expect.objectContaining({ id: 'todo-root', origin: 'discovered', management: 'observe' }),
    ]);
  });

  it('reports unsupported adapter families without changing the source', async () => {
    const root = await createFixtureDirectory('arcantry-adapter-');
    await writeFile(join(root, 'todo.txt'), 'Keep me\n');
    const config = parseProjectConfig(`config_version = 1

[sources.tasks]
kind = "todo-txt"
path = "todo.txt"
management = "validate"
adapter = "todo-txt@2"
`);

    const inspection = await inspectKnowledge({ root, configPath: join(root, 'arcantry.toml'), config, mode: 'configured' });

    expect(inspection.sources[0]).toMatchObject({ adapterStatus: 'unsupported', exists: true });
    expect(inspection.diagnostics).toEqual(['Source tasks requires unsupported adapter todo-txt@2.']);
  });

  it('continues to support the previous changelog adapter family', async () => {
    const root = await createFixtureDirectory('arcantry-previous-adapter-');
    await writeFile(join(root, 'CHANGELOG.md'), '# Changelog\n\n## [Unreleased]\n');
    const config = parseProjectConfig(`config_version = 1

[sources.history]
kind = "changelog"
path = "CHANGELOG.md"
management = "validate"
adapter = "keep-a-changelog@1"
`);

    const inspection = await inspectKnowledge({ root, configPath: join(root, 'arcantry.toml'), config, mode: 'configured' });

    expect(inspection.sources[0]).toMatchObject({ adapter: 'keep-a-changelog@1', adapterStatus: 'supported' });
  });
});

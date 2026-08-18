import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import {
  doctorRepository,
  initRepository,
  planRepositoryInit,
  removeRepository,
  updateRepository,
  validateRepository,
} from './repository.js';
import { createFixtureRepository, removeFixtures } from './testHelpers.js';

afterEach(removeFixtures);

describe('repository adoption', () => {
  it('initializes only minimal private state', async () => {
    const root = await createFixtureRepository();
    await writeFile(join(root, 'package.json'), '{"private":true}\n');

    await initRepository(root, 'private');

    expect(await readFile(join(root, '.local/arcantry.toml'), 'utf8')).toContain('config_version = 1');
    expect(await readFile(join(root, '.local/AGENTS.md'), 'utf8')).toContain('<!-- arcantry:start -->');
    expect(await readFile(join(root, '.git/info/exclude'), 'utf8')).toContain('.local/');
    expect(await readFile(join(root, 'package.json'), 'utf8')).toBe('{"private":true}\n');
    await expect(readFile(join(root, 'justfile'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
    expect((await validateRepository(root)).valid).toBe(true);
  });

  it('initializes shared configuration and preserves surrounding guidance', async () => {
    const root = await createFixtureRepository();
    await writeFile(join(root, 'AGENTS.md'), '# Project rules\n');

    await initRepository(root, 'shared');

    expect(await readFile(join(root, 'arcantry.toml'), 'utf8')).toContain('config_version = 1');
    expect(await readFile(join(root, 'AGENTS.md'), 'utf8')).toContain('# Project rules');
    expect(await readFile(join(root, 'AGENTS.md'), 'utf8')).toContain('Use `arcantry.toml`');
    await expect(readFile(join(root, '.local/arcantry.toml'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('adds Claude compatibility as an import without duplicating guidance', async () => {
    const root = await createFixtureRepository();
    await writeFile(join(root, 'CLAUDE.md'), '# Claude-specific note\n');

    await initRepository(root, 'shared', 'claude');

    const content = await readFile(join(root, 'CLAUDE.md'), 'utf8');
    expect(content).toContain('@AGENTS.md');
    expect(content).toContain('# Claude-specific note');
    expect(await readFile(join(root, 'AGENTS.md'), 'utf8')).toContain('Use `arcantry.toml`');
  });

  it('keeps the private Claude adapter locally excluded', async () => {
    const root = await createFixtureRepository();

    await initRepository(root, 'private', 'claude');

    expect(await readFile(join(root, 'CLAUDE.local.md'), 'utf8')).toContain('@.local/AGENTS.md');
    expect(await readFile(join(root, '.git/info/exclude'), 'utf8')).toContain('CLAUDE.local.md');
    expect((await validateRepository(root)).valid).toBe(true);
  });

  it('preserves an existing valid configuration during initialization', async () => {
    const root = await createFixtureRepository();
    const config = `config_version = 1

[sources.tasks]
kind = "todo-txt"
path = "todo.txt"
management = "observe"
adapter = "todo-txt@1"
`;
    await mkdir(join(root, '.local'), { recursive: true });
    await writeFile(join(root, '.local/arcantry.toml'), config);

    const plan = await planRepositoryInit(root, 'private');
    await initRepository(root, 'private');

    expect(plan.conflicts).toEqual([]);
    expect(await readFile(join(root, '.local/arcantry.toml'), 'utf8')).toBe(config);
  });

  it('updates managed guidance while preserving configuration and user content', async () => {
    const root = await createFixtureRepository();
    await initRepository(root, 'private');
    const config = await readFile(join(root, '.local/arcantry.toml'), 'utf8');
    const guidancePath = join(root, '.local/AGENTS.md');
    await writeFile(guidancePath, `${(await readFile(guidancePath, 'utf8')).replace('## Arcantry local context', '## Outdated')}\nUser note\n`);

    await updateRepository(root, 'private');

    expect(await readFile(join(root, '.local/arcantry.toml'), 'utf8')).toBe(config);
    expect(await readFile(guidancePath, 'utf8')).toContain('## Arcantry local context');
    expect(await readFile(guidancePath, 'utf8')).toContain('User note');
  });

  it('reports outdated guidance with a scope-specific repair', async () => {
    const root = await createFixtureRepository();
    await initRepository(root, 'shared');
    await writeFile(join(root, 'AGENTS.md'), (await readFile(join(root, 'AGENTS.md'), 'utf8')).replace('Use `arcantry.toml`', 'Ignore configuration'));

    expect((await validateRepository(root)).valid).toBe(false);
    expect((await doctorRepository(root)).diagnostics).toContainEqual(expect.objectContaining({
      path: 'AGENTS.md',
      message: 'Arcantry managed section is missing or outdated.',
      repair: 'Run `arcantry repo update --scope shared`.',
    }));
  });

  it('removes only the selected verified scope', async () => {
    const root = await createFixtureRepository();
    await writeFile(join(root, 'AGENTS.md'), '# User rules\n');
    await initRepository(root, 'shared');
    await initRepository(root, 'private');
    await writeFile(join(root, 'todo.txt'), 'Keep me\n');

    await removeRepository(root, 'private');

    expect(await readFile(join(root, 'arcantry.toml'), 'utf8')).toContain('config_version = 1');
    expect(await readFile(join(root, 'AGENTS.md'), 'utf8')).toContain('# User rules');
    expect(await readFile(join(root, 'todo.txt'), 'utf8')).toBe('Keep me\n');
    await expect(readFile(join(root, '.local/arcantry.toml'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('removes only the managed Claude adapter content', async () => {
    const root = await createFixtureRepository();
    await writeFile(join(root, 'CLAUDE.md'), '# Claude-specific note\n');
    await initRepository(root, 'shared', 'claude');

    await removeRepository(root, 'shared');

    expect(await readFile(join(root, 'CLAUDE.md'), 'utf8')).toBe('# Claude-specific note\n');
  });
});

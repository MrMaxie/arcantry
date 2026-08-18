import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { doctorRepository, initRepository, planRepositoryInit, removeRepository, updateRepository, validateRepository } from './repository.js';
import { createFixtureRepository, removeFixtures } from './testHelpers.js';

afterEach(removeFixtures);

describe('repository adoption', () => {
  it('initializes owned artifacts without creating or reading .docs', async () => {
    const root = await createFixtureRepository();
    await mkdir(join(root, '.docs'), { recursive: true });
    await writeFile(join(root, '.docs/AGENTS.md'), 'project-owned docs\n');

    await initRepository(root, { docs: 'none', agents: ['codex', 'cursor'] });

    const config = JSON.parse(await readFile(join(root, '.local/arcantry.json'), 'utf8')) as { docs: string };
    expect(config.docs).toBe('none');
    expect(await readFile(join(root, 'AGENTS.md'), 'utf8')).toContain('<!-- arcantry:start -->');
    expect(await readFile(join(root, '.cursor/rules/arcantry.mdc'), 'utf8')).toContain('description: Arcantry repository guidance');
    expect(await readFile(join(root, '.git/info/exclude'), 'utf8')).toContain('.local/');
    expect(await readFile(join(root, '.docs/AGENTS.md'), 'utf8')).toBe('project-owned docs\n');
    expect((await validateRepository(root)).valid).toBe(true);
  });

  it('stops before any writes when a protected scaffold conflicts', async () => {
    const root = await createFixtureRepository();
    await writeFile(join(root, 'justfile'), 'user-owned recipe:\n    echo hello\n');

    const plan = await planRepositoryInit(root, { docs: 'none' });

    expect(plan.conflicts).toContainEqual(expect.objectContaining({ path: 'justfile' }));
    await expect(initRepository(root, { docs: 'none' })).rejects.toThrow('Cannot apply init');
    await expect(readFile(join(root, '.local/arcantry.json'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
    expect(await readFile(join(root, 'justfile'), 'utf8')).toContain('user-owned');
  });

  it('accepts semantically compatible project-owned tooling without rewriting it', async () => {
    const root = await createFixtureRepository();
    const justfile = ['setup:', '    pnpm install', 'check:', '    pnpm test', 'build:', '    pnpm build', 'ci: check build', ''].join('\n');
    const mise = '[tools]\nnode = "24.19.0"\npnpm = "10.34.5"\njust = "1.58.0"\n';
    await writeFile(join(root, 'justfile'), justfile);
    await writeFile(join(root, 'mise.toml'), mise);
    await mkdir(join(root, 'openspec'), { recursive: true });
    await writeFile(join(root, 'openspec/config.yaml'), 'schema: arcantry\n\ncontext: project-owned\n');

    const plan = await planRepositoryInit(root, { docs: 'none' });

    expect(plan.conflicts).toEqual([]);
    await initRepository(root, { docs: 'none' });
    expect(await readFile(join(root, 'justfile'), 'utf8')).toBe(justfile);
    expect(await readFile(join(root, 'mise.toml'), 'utf8')).toBe(mise);
    expect(await readFile(join(root, 'openspec/config.yaml'), 'utf8')).toContain('project-owned');
  });

  it('updates managed sections while preserving configuration and user content', async () => {
    const root = await createFixtureRepository();
    await initRepository(root, {
      docs: 'none',
      agents: ['codex'],
      sources: [
        { name: 'local', mode: 'operational' },
        { name: 'linear', mode: 'readwrite' },
      ],
    });
    const originalConfig = await readFile(join(root, '.local/arcantry.json'), 'utf8');
    const agentsPath = join(root, 'AGENTS.md');
    await writeFile(agentsPath, (await readFile(agentsPath, 'utf8')).replace('## Arcantry', '## Outdated\n\nUser note'));

    await updateRepository(root);

    expect(await readFile(join(root, '.local/arcantry.json'), 'utf8')).toBe(originalConfig);
    expect(await readFile(agentsPath, 'utf8')).toContain('## Arcantry');
  });

  it('reports an outdated managed section and gives doctor a repair path', async () => {
    const root = await createFixtureRepository();
    await initRepository(root, { docs: 'none', agents: ['codex'] });
    const agentsPath = join(root, 'AGENTS.md');
    await writeFile(agentsPath, (await readFile(agentsPath, 'utf8')).replace('Use `openspec/`', 'Do not use OpenSpec'));

    expect((await validateRepository(root)).valid).toBe(false);
    const report = await doctorRepository(root);
    expect(report.diagnostics).toContainEqual(
      expect.objectContaining({ path: 'AGENTS.md', message: 'Arcantry managed section is missing or outdated.', repair: 'Run `arcantry repo update`.' }),
    );
  });

  it('removes only verified generated content and managed sections', async () => {
    const root = await createFixtureRepository();
    await writeFile(join(root, 'AGENTS.md'), '# User rules\n');
    await initRepository(root, { docs: 'none', agents: ['codex'] });
    await mkdir(join(root, 'openspec/specs'), { recursive: true });
    await writeFile(join(root, 'openspec/specs/user.md'), 'keep me\n');

    await removeRepository(root);

    expect(await readFile(join(root, 'AGENTS.md'), 'utf8')).toBe('# User rules\n');
    expect(await readFile(join(root, 'openspec/specs/user.md'), 'utf8')).toBe('keep me\n');
    expect(await readFile(join(root, 'openspec/config.yaml'), 'utf8')).toContain('arcantry:generated');
    await expect(readFile(join(root, '.local/arcantry.json'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('preserves unknown files during removal', async () => {
    const root = await createFixtureRepository();
    await initRepository(root, { docs: 'none', agents: ['cursor'] });
    await writeFile(join(root, '.cursor/rules/custom.mdc'), 'custom rule\n');

    await removeRepository(root);

    expect(await readFile(join(root, '.cursor/rules/custom.mdc'), 'utf8')).toBe('custom rule\n');
  });
});

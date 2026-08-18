import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { runCli } from './program.js';
import { createFixtureDirectory, createFixtureRepository, removeFixtures } from './testHelpers.js';

afterEach(removeFixtures);

const run = async (cwd: string, args: string[]) => {
  let stdout = '';
  let stderr = '';
  const result = await runCli(['node', 'arcantry', ...args], {
    cwd,
    stdout: (message) => {
      stdout += message;
    },
    stderr: (message) => {
      stderr += message;
    },
  });
  return { ...result, stdout, stderr };
};

describe('CLI', () => {
  it('requires an explicit repository scope for initialization', async () => {
    const root = await createFixtureRepository();
    const result = await run(root, ['repo', 'init']);
    expect(result.exitCode).toBe(1);
    expect(result.stderr).toContain("required option '--scope <scope>' not specified");
  });

  it('rejects unsupported repository scopes before writing anything', async () => {
    const root = await createFixtureRepository();

    const result = await run(root, ['repo', 'init', '--scope', 'team']);

    expect(result.exitCode).toBe(1);
    expect(result.stderr).toContain('Invalid option');
    await expect(readFile(join(root, '.local/arcantry.toml'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('initializes, validates, updates, and removes through the stable command surface', async () => {
    const root = await createFixtureRepository();

    expect((await run(root, ['repo', 'init', '--scope', 'private'])).exitCode).toBe(0);
    expect((await run(root, ['repo', 'validate'])).stdout).toContain('Repository adoption is valid.');
    expect((await run(root, ['repo', 'doctor'])).exitCode).toBe(0);
    expect((await run(root, ['repo', 'update', '--scope', 'private'])).stdout).toContain('No changes required.');
    expect((await run(root, ['repo', 'remove', '--scope', 'private'])).exitCode).toBe(0);
    await expect(readFile(join(root, '.local/arcantry.toml'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('keeps shared and private repository scopes independent', async () => {
    const root = await createFixtureRepository();
    expect((await run(root, ['repo', 'init', '--scope', 'shared'])).exitCode).toBe(0);
    expect((await run(root, ['repo', 'init', '--scope', 'private'])).exitCode).toBe(0);

    expect(await readFile(join(root, 'arcantry.toml'), 'utf8')).toContain('config_version = 1');
    expect(await readFile(join(root, '.local/arcantry.toml'), 'utf8')).toContain('config_version = 1');
    expect((await run(root, ['repo', 'inspect'])).stdout).toContain('Config: private');
    expect((await run(root, ['repo', 'inspect'])).stdout).toContain('Shadowed config:');
  });

  it('treats an unconfigured project as a valid wild knowledge stack', async () => {
    const root = await createFixtureDirectory('arcantry-cli-wild-');
    const result = await run(root, ['repo', 'validate']);
    expect(result.exitCode).toBe(0);
    expect(result.stdout).toContain('Knowledge stack is valid.');
  });

  it('previews and applies a configured release baseline without publishing', async () => {
    const root = await createFixtureDirectory('arcantry-cli-release-');
    await mkdir(join(root, 'openspec', 'changes', 'archive'), { recursive: true });
    await writeFile(join(root, 'package.json'), '{\n  "name": "fixture",\n  "version": "0.8.0"\n}\n');
    await writeFile(join(root, 'arcantry.toml'), `config_version = 1

[sources.openspec]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.changelog]
kind = "changelog"
path = "CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["openspec"]

[release]
adapter = "openspec-release@1"
manifests_path = "releases"
changelog_source = "changelog"
tag_prefix = "v"

[[release.version_sources]]
path = "package.json"
adapter = "json-package@1"
`);

    const preview = await run(root, ['release', 'baseline', '0.8.0', '--date', '2026-06-11']);
    expect(preview.exitCode).toBe(0);
    expect(preview.stdout).toContain('write: releases/0.8.0.yaml');
    expect(preview.stdout).toContain('Run the same command with --apply');
    await expect(readFile(join(root, 'releases', '0.8.0.yaml'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });

    const applied = await run(root, [
      'release',
      'baseline',
      '0.8.0',
      '--date',
      '2026-06-11',
      '--apply',
    ]);
    expect(applied.exitCode).toBe(0);
    expect(applied.stdout).toContain('write: releases/0.8.0.yaml');
    expect((await run(root, ['release', 'check'])).stdout).toContain('Release state is consistent.');
  });

  it('inspects, plans, and applies a source without repository metadata', async () => {
    const root = await createFixtureDirectory('arcantry-cli-plan-');
    await writeFile(join(root, 'todo.txt'), 'Move me @desk\n');

    const inspection = await run(root, ['repo', 'inspect']);
    const planned = await run(root, [
      'repo',
      'plan',
      '--source',
      'todo-root',
      '--transition',
      'relocate',
      '--to-path',
      '.local/todo.txt',
      '--delete-source',
      '--json',
    ]);
    const planPath = join(root, 'plan.json');
    await writeFile(planPath, planned.stdout);
    const applied = await run(root, ['repo', 'apply', '--plan', planPath]);

    expect(inspection.stdout).toContain('todo-root\ttodo-txt\tobserve\ttodo-txt@1');
    expect(planned.exitCode).toBe(0);
    expect(applied.stdout).toContain('write: .local/todo.txt');
    expect(await readFile(join(root, '.local/todo.txt'), 'utf8')).toBe('Move me @desk\n');
    await expect(readFile(join(root, 'todo.txt'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
    await expect(readFile(join(root, 'arcantry.toml'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('uses one explicit external configuration without copying it into the project', async () => {
    const root = await createFixtureDirectory('arcantry-cli-external-');
    const external = await createFixtureDirectory('arcantry-cli-sidecar-');
    const configPath = join(external, 'arcantry.toml');
    await writeFile(configPath, `config_version = 1

[sources.tasks]
kind = "todo-txt"
path = "todo.txt"
management = "observe"
adapter = "todo-txt@1"
`);

    const result = await run(root, ['--config', configPath, 'repo', 'inspect']);
    const validation = await run(root, ['--config', configPath, 'repo', 'validate']);

    expect(result.exitCode).toBe(0);
    expect(result.stdout).toContain('Mode: configured');
    expect(result.stdout).toContain('tasks\ttodo-txt\tobserve\ttodo-txt@1\thigh\tmissing');
    expect(validation.exitCode).toBe(0);
    expect(validation.stdout).toContain('Repository adoption is valid.');
    expect(validation.stdout).toContain('Knowledge stack is valid.');
  });

  it('previews todo writes and requires an explicit queue when both exist', async () => {
    const root = await createFixtureRepository();
    await mkdir(join(root, '.local'), { recursive: true });
    await writeFile(join(root, 'todo.txt'), 'Shared task +project\n');
    await writeFile(join(root, '.local/todo.txt'), 'Private task @desk key:value\n');

    const ambiguous = await run(root, ['todo', 'add', 'New task']);
    const preview = await run(root, ['todo', 'add', 'New private task @home', '--source', 'local']);

    expect(ambiguous.exitCode).toBe(1);
    expect(ambiguous.stderr).toContain('choose --source explicitly');
    expect(preview.stdout).toContain('Run the same command with --apply');
    expect(await readFile(join(root, '.local/todo.txt'), 'utf8')).toBe('Private task @desk key:value\n');
  });

  it('applies explicit todo moves and keeps private state locally excluded', async () => {
    const root = await createFixtureRepository();
    await writeFile(join(root, 'todo.txt'), 'Move me +project @desk\nKeep me\n');

    const result = await run(root, ['todo', 'move', '1', '--from', 'root', '--to', 'local', '--apply']);

    expect(result.exitCode).toBe(0);
    expect(await readFile(join(root, 'todo.txt'), 'utf8')).toBe('Keep me\n');
    expect(await readFile(join(root, '.local/todo.txt'), 'utf8')).toBe('Move me +project @desk\n');
    expect(await readFile(join(root, '.git/info/exclude'), 'utf8')).toContain('.local/');
  });

  it('lists, inspects, links, diagnoses, and unlinks skills', async () => {
    const root = await createFixtureDirectory('arcantry-cli-catalog-');
    const skill = join(root, 'skills', 'example-skill');
    const links = await createFixtureDirectory('arcantry-cli-links-');
    await mkdir(join(skill, 'agents'), { recursive: true });
    await writeFile(join(root, 'catalog.json'), '{"$schema":"./schemas/catalog.schema.json","skills":[{"name":"example-skill","family":"repo-safely","tags":["quality"]}]}\n');
    await writeFile(
      join(skill, 'SKILL.md'),
      '---\nname: example-skill\ndescription: Execute one concrete task with safe and fully verifiable behavior.\n---\n',
    );
    await writeFile(
      join(skill, 'arcantry.json'),
      JSON.stringify({
        $schema: '../../schemas/skill-metadata.schema.json',
        summary: 'Execute one concrete task with safe and fully verifiable behavior.',
        scenarios: [
          { title: 'First case', prompt: 'Use $example-skill once.', outcome: 'The first result is ready.' },
          { title: 'Second case', prompt: 'Use $example-skill twice.', outcome: 'The second result is ready.' },
        ],
      }),
    );
    await writeFile(join(skill, 'agents', 'openai.yaml'), 'interface:\n  default_prompt: Use $example-skill.\n');

    expect((await run(root, ['skills', 'list'])).stdout).toContain('example-skill');
    expect((await run(root, ['skills', 'inspect', 'example-skill'])).stdout).toContain('Execute one concrete task');
    expect((await run(root, ['skills', 'link', 'example-skill', '--target', links])).stdout).toContain('Linked: example-skill');
    expect((await run(root, ['skills', 'doctor', '--target', links])).exitCode).toBe(0);
    expect((await run(root, ['skills', 'unlink', 'example-skill', '--target', links])).stdout).toContain('Unlinked: example-skill');
  });

  it('links skills into the universal directory with optional Claude compatibility', async () => {
    const root = await createFixtureRepository();
    const skill = join(root, 'skills', 'example-skill');
    await mkdir(join(skill, 'agents'), { recursive: true });
    await writeFile(join(root, 'catalog.json'), '{"$schema":"./schemas/catalog.schema.json","skills":[{"name":"example-skill","family":"repo-safely","tags":["quality"]}]}\n');
    await writeFile(
      join(skill, 'SKILL.md'),
      '---\nname: example-skill\ndescription: Execute one concrete task with safe and fully verifiable behavior.\n---\n',
    );
    await writeFile(
      join(skill, 'arcantry.json'),
      JSON.stringify({
        $schema: '../../schemas/skill-metadata.schema.json',
        summary: 'Execute one concrete task with safe and fully verifiable behavior.',
        scenarios: [
          { title: 'First case', prompt: 'Use the example skill once.', outcome: 'The first result is ready.' },
          { title: 'Second case', prompt: 'Use the example skill twice.', outcome: 'The second result is ready.' },
        ],
      }),
    );
    await writeFile(join(skill, 'agents', 'openai.yaml'), 'interface:\n  default_prompt: Use $example-skill.\n');

    expect((await run(root, ['skills', 'link', 'example-skill', '--scope', 'repo', '--compat', 'claude'])).exitCode).toBe(0);
    expect((await run(root, ['skills', 'doctor', '--scope', 'repo', '--compat', 'claude'])).exitCode).toBe(0);
    expect((await run(root, ['skills', 'unlink', 'example-skill', '--scope', 'repo', '--compat', 'claude'])).exitCode).toBe(0);
  });

  it('rejects compatibility without scope or with an explicit target', async () => {
    const root = await createFixtureRepository();
    const withoutScope = await run(root, ['skills', 'doctor', '--compat', 'claude']);
    const withTarget = await run(root, ['skills', 'doctor', '--compat', 'claude', '--target', 'links']);

    expect(withoutScope.exitCode).toBe(1);
    expect(withoutScope.stderr).toContain('--compat requires --scope user|repo|private');
    expect(withTarget.exitCode).toBe(1);
    expect(withTarget.stderr).toContain('--target cannot be combined with --scope or --compat');
  });

  it('links private repository skills without adding them to the public catalog', async () => {
    const root = await createFixtureRepository();
    const privateSkill = join(root, '.local/skills/private-helper');
    await mkdir(privateSkill, { recursive: true });
    await writeFile(
      join(privateSkill, 'SKILL.md'),
      '---\nname: private-helper\ndescription: Use one private repository workflow without publishing its local instructions.\n---\n',
    );

    const result = await run(root, ['skills', 'link', 'private-helper', '--scope', 'private', '--compat', 'claude']);

    expect(result.exitCode).toBe(0);
    expect((await run(root, ['skills', 'list', '--scope', 'private'])).stdout).toContain('private-helper');
    expect(await readFile(join(root, '.git/info/exclude'), 'utf8')).toContain('.agents/skills/private-helper');
    expect(await readFile(join(root, '.git/info/exclude'), 'utf8')).toContain('.claude/skills/private-helper');
  });

  it('rejects a private skill that reuses a public catalog identity', async () => {
    const root = await createFixtureRepository();
    const privateSkill = join(root, '.local/skills/example-skill');
    await mkdir(privateSkill, { recursive: true });
    await mkdir(join(root, 'skills'));
    await writeFile(join(root, 'catalog.json'), '{"$schema":"./schemas/catalog.schema.json","skills":[{"name":"example-skill","family":"repo-safely","tags":["quality"]}]}\n');
    await writeFile(
      join(privateSkill, 'SKILL.md'),
      '---\nname: example-skill\ndescription: Keep this private package distinct from every public skill identity.\n---\n',
    );

    const link = await run(root, ['skills', 'link', 'example-skill', '--scope', 'private']);
    const doctor = await run(root, ['skills', 'doctor', '--scope', 'private']);

    expect(link.exitCode).toBe(1);
    expect(link.stderr).toContain('exists in both the public catalog and .local/skills');
    expect(doctor.exitCode).toBe(1);
    expect(doctor.stderr).toContain('exists in both the public catalog and .local/skills');
  });
});

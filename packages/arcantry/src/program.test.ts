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
  it('requires an explicit docs mode for repository initialization', async () => {
    const root = await createFixtureRepository();
    const result = await run(root, ['repo', 'init']);
    expect(result.exitCode).toBe(1);
    expect(result.stderr).toContain("required option '--docs <mode>' not specified");
  });

  it('rejects legacy docs modes before writing anything', async () => {
    const root = await createFixtureRepository();

    const result = await run(root, ['repo', 'init', '--docs', 'shared']);

    expect(result.exitCode).toBe(1);
    expect(result.stderr).toContain('Existing .docs content remains project-owned');
    await expect(readFile(join(root, '.local/arcantry.json'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
    await expect(readFile(join(root, '.docs/AGENTS.md'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('initializes, validates, updates, and removes through the stable command surface', async () => {
    const root = await createFixtureRepository();

    expect((await run(root, ['repo', 'init', '--docs', 'none'])).exitCode).toBe(0);
    expect((await run(root, ['repo', 'validate'])).stdout).toContain('Repository adoption is valid.');
    expect((await run(root, ['repo', 'doctor'])).exitCode).toBe(0);
    expect((await run(root, ['repo', 'update'])).stdout).toContain('No changes required.');
    expect((await run(root, ['repo', 'remove'])).exitCode).toBe(0);
    await expect(readFile(join(root, '.local/arcantry.json'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
  });

  it('accepts ordered source and agent selections', async () => {
    const root = await createFixtureRepository();
    const result = await run(root, [
      'repo',
      'init',
      '--docs',
      'none',
      '--agent',
      'claude',
      '--source',
      'local=operational',
      '--source',
      'linear=readwrite',
    ]);

    expect(result.exitCode).toBe(0);
    const config = JSON.parse(await readFile(join(root, '.local/arcantry.json'), 'utf8')) as { agents: string[]; sources: unknown[] };
    expect(config.agents).toEqual(['claude']);
    expect(config.sources).toHaveLength(2);
  });

  it('treats an unconfigured project as a valid wild knowledge stack', async () => {
    const root = await createFixtureDirectory('arcantry-cli-wild-');
    const result = await run(root, ['repo', 'validate']);
    expect(result.exitCode).toBe(0);
    expect(result.stdout).toContain('Knowledge stack is valid.');
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

    expect(result.exitCode).toBe(0);
    expect(result.stdout).toContain('Mode: configured');
    expect(result.stdout).toContain('tasks\ttodo-txt\tobserve\ttodo-txt@1\thigh\tmissing');
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
    await writeFile(join(root, 'catalog.json'), '{"$schema":"./schemas/catalog.schema.json","skills":[{"name":"example-skill","tags":["quality"]}]}\n');
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
});

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
      'local',
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

  it('reports validation failure with a non-zero exit code', async () => {
    const root = await createFixtureRepository();
    const result = await run(root, ['repo', 'validate']);
    expect(result.exitCode).toBe(1);
    expect(result.stderr).toContain('Arcantry configuration is missing.');
  });

  it('gives doctor an explicit repair action', async () => {
    const root = await createFixtureRepository();
    const result = await run(root, ['repo', 'doctor']);

    expect(result.exitCode).toBe(1);
    expect(result.stderr).toContain('Repair: Run `arcantry repo init --docs <shared|local|none>` after choosing the documentation mode.');
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

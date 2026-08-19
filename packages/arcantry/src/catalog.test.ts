import { mkdir, readFile, symlink, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import {
  inspectSkill,
  linkSkill,
  linkSkillTargets,
  repositoryClaudeSkillTargetRoot,
  repositorySkillTargetRoot,
  unlinkSkill,
  userClaudeSkillTargetRoot,
  userSkillTargetRoot,
  validateCatalog,
} from './catalog.js';
import { createFixtureDirectory, removeFixtures } from './testHelpers.js';

afterEach(removeFixtures);

const createCatalogFixture = async (): Promise<string> => {
  const root = await createFixtureDirectory('arcantry-catalog-');
  const skillRoot = join(root, 'skills', 'example-skill');
  await mkdir(join(skillRoot, 'agents'), { recursive: true });
  await writeFile(
    join(root, 'catalog.json'),
    `${JSON.stringify({ $schema: './schemas/catalog.schema.json', skills: [{ name: 'example-skill', family: 'repo-safely', tags: ['quality'] }] }, null, 2)}\n`,
  );
  await writeFile(
    join(skillRoot, 'SKILL.md'),
    '---\nname: example-skill\ndescription: Perform a concrete example task with a sufficiently precise public description.\n---\n\n# Example\n',
  );
  await writeFile(
    join(skillRoot, 'arcantry.json'),
    `${JSON.stringify({
      $schema: '../../schemas/skill-metadata.schema.json',
      summary: 'Perform a concrete example task with predictable and verifiable results.',
      compatibility: {
        sourceKinds: ['openspec', 'todo-txt'],
        adapters: [{ name: 'openspec', versions: '>=1 <3' }],
      },
      learning: {
        prerequisites: ['Know which project source is authoritative.'],
        outcomes: ['Choose a compatible adapter without mutating project data.'],
      },
      scenarios: [
        { title: 'First case', prompt: 'Use $example-skill for the first case.', outcome: 'The first case is handled.' },
        { title: 'Second case', prompt: 'Use $example-skill for the second case.', outcome: 'The second case is handled.' },
      ],
    })}\n`,
  );
  await writeFile(join(skillRoot, 'agents', 'openai.yaml'), 'interface:\n  default_prompt: "Use $example-skill for this task."\n');
  return root;
};

describe('skill catalog', () => {
  it('keeps the universal target separate from Claude compatibility', () => {
    expect(userSkillTargetRoot()).toMatch(/[\\/]\.agents[\\/]skills$/);
    expect(userClaudeSkillTargetRoot()).toMatch(/[\\/]\.claude[\\/]skills$/);
    expect(repositorySkillTargetRoot('/project')).toMatch(/[\\/]project[\\/]\.agents[\\/]skills$/);
    expect(repositoryClaudeSkillTargetRoot('/project')).toMatch(/[\\/]project[\\/]\.claude[\\/]skills$/);
  });

  it('validates and inspects canonical skill packages', async () => {
    const root = await createCatalogFixture();
    expect((await validateCatalog(root)).valid).toBe(true);
    const metadata = (await inspectSkill(root, 'example-skill')).metadata;
    expect(metadata.scenarios).toHaveLength(2);
    expect(metadata.compatibility?.adapters?.[0]?.versions).toBe('>=1 <3');
  });

  it('links idempotently and unlinks only the exact source', async () => {
    const root = await createCatalogFixture();
    const targetRoot = await createFixtureDirectory('arcantry-links-');

    expect((await linkSkill({ catalogRoot: root, name: 'example-skill', targetRoot })).status).toBe('linked');
    expect((await linkSkill({ catalogRoot: root, name: 'example-skill', targetRoot })).status).toBe('unchanged');
    expect((await unlinkSkill({ catalogRoot: root, name: 'example-skill', targetRoot })).status).toBe('unlinked');
  });

  it('links one canonical package through universal and Claude aliases', async () => {
    const root = await createCatalogFixture();
    const universal = await createFixtureDirectory('arcantry-universal-links-');
    const claude = await createFixtureDirectory('arcantry-claude-links-');

    const results = await linkSkillTargets({ catalogRoot: root, name: 'example-skill' }, [universal, claude]);

    expect(results.map((result) => result.status)).toEqual(['linked', 'linked']);
    expect(results[0]?.source).toBe(results[1]?.source);
  });

  it('backs up an ordinary directory only with explicit replacement', async () => {
    const root = await createCatalogFixture();
    const targetRoot = await createFixtureDirectory('arcantry-links-');
    const target = join(targetRoot, 'example-skill');
    await mkdir(target);
    await writeFile(join(target, 'user.txt'), 'keep\n');

    await expect(linkSkill({ catalogRoot: root, name: 'example-skill', targetRoot })).rejects.toThrow('Use --replace');
    const result = await linkSkill({ catalogRoot: root, name: 'example-skill', targetRoot, replace: true, backupLabel: 'test' });

    expect(result.backup).toBe(`${target}.backup-test`);
    expect(await readFile(join(`${target}.backup-test`, 'user.txt'), 'utf8')).toBe('keep\n');
  });

  it('refuses to unlink a link to another source', async () => {
    const root = await createCatalogFixture();
    const targetRoot = await createFixtureDirectory('arcantry-links-');
    const other = await createFixtureDirectory('arcantry-other-');
    await symlink(other, join(targetRoot, 'example-skill'), process.platform === 'win32' ? 'junction' : 'dir');

    await expect(unlinkSkill({ catalogRoot: root, name: 'example-skill', targetRoot })).rejects.toThrow('nothing was removed');
  });

  it('enforces the published catalog and metadata schema markers', async () => {
    const root = await createCatalogFixture();
    await writeFile(join(root, 'catalog.json'), JSON.stringify({ skills: [{ name: 'example-skill', tags: ['invalid tag'] }] }));

    const validation = await validateCatalog(root);
    expect(validation.valid).toBe(false);
    expect(validation.errors.join('\n')).toContain('$schema');
    expect(validation.errors.join('\n')).toContain('Invalid string');
  });
});

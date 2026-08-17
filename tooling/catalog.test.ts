import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { validateCatalog } from './catalog.js';

function fixture(name = 'example-skill'): string {
  const root = mkdtempSync(join(tmpdir(), 'arcantry-catalog-'));
  const skill = join(root, 'skills', name);
  mkdirSync(join(skill, 'agents'), { recursive: true });
  writeFileSync(join(root, 'catalog.json'), `${JSON.stringify({ skills: [{ name, tags: ['example'] }] })}\n`);
  writeFileSync(
    join(skill, 'SKILL.md'),
    `---\nname: ${name}\ndescription: Use this example skill for a concrete catalog validation task.\n---\n\n# Example\n`,
  );
  writeFileSync(
    join(skill, 'arcantry.json'),
    `${JSON.stringify({
      summary: 'Validate one complete skill package in the Arcantry catalog.',
      scenarios: [
        { title: 'First case', prompt: 'Use the example skill for the first task.', outcome: 'The first task is complete.' },
        { title: 'Second case', prompt: 'Use the example skill for the second task.', outcome: 'The second task is complete.' },
      ],
    })}\n`,
  );
  writeFileSync(
    join(skill, 'agents', 'openai.yaml'),
    `interface:\n  display_name: "Example"\n  short_description: "Validate an example package"\n  default_prompt: "Use $${name} to validate this example."\n`,
  );
  return root;
}

describe('skill catalog validation', () => {
  it('accepts a complete canonical package', () => {
    expect(validateCatalog(fixture())).toEqual([]);
  });

  it('rejects membership drift', () => {
    const root = fixture();
    mkdirSync(join(root, 'skills', 'unlisted-skill'));
    expect(validateCatalog(root)).toContain('catalog membership must exactly match skills/ directories');
  });

  it('requires the UI prompt to invoke the skill explicitly', () => {
    const root = fixture();
    writeFileSync(
      join(root, 'skills', 'example-skill', 'agents', 'openai.yaml'),
      'interface:\n  default_prompt: "Validate this example."\n',
    );
    expect(validateCatalog(root)).toContain('skills/example-skill default_prompt must mention $example-skill');
  });

  it('rejects broken relative resource links', () => {
    const root = fixture();
    const skillPath = join(root, 'skills', 'example-skill', 'SKILL.md');
    writeFileSync(
      skillPath,
      '---\nname: example-skill\ndescription: Use this example skill for a concrete catalog validation task.\n---\n\n[Missing](references/missing.md)\n',
    );

    expect(validateCatalog(root).some((error) => error.includes('references missing references/missing.md'))).toBe(true);
  });
});

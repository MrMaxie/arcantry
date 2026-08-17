import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { createFixtureRepository, removeFixtures } from './testHelpers.js';
import { detectAgents, getAgentArtifact } from './agents.js';

afterEach(removeFixtures);

describe('agent adapters', () => {
  it('maps supported agents to stable entrypoints', () => {
    expect(getAgentArtifact('codex').path).toBe('AGENTS.md');
    expect(getAgentArtifact('claude').path).toBe('CLAUDE.md');
    expect(getAgentArtifact('cursor').path).toBe('.cursor/rules/arcantry.mdc');
  });

  it('detects repository agent configuration without changing it', async () => {
    const root = await createFixtureRepository();
    await writeFile(join(root, 'AGENTS.md'), '# Existing instructions\n');
    await mkdir(join(root, '.claude'));

    const detections = await detectAgents(root);

    expect(detections.find((item) => item.agent === 'codex')?.detected).toBe(true);
    expect(detections.find((item) => item.agent === 'claude')?.detected).toBe(true);
    expect(detections.find((item) => item.agent === 'cursor')?.detected).toBe(false);
  });
});

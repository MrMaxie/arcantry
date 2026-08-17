import { describe, expect, it } from 'vitest';
import { arcantryConfigSchema, createArcantryConfig } from './config.js';

describe('arcantryConfigSchema', () => {
  it('creates the safe local default while requiring an explicit docs mode', () => {
    expect(createArcantryConfig({ docs: 'none' })).toEqual({
      schemaVersion: 1,
      agents: ['codex'],
      operationalSource: 'local',
      sources: [{ name: 'local', mode: 'operational' }],
      docs: 'none',
    });
  });

  it('preserves source order and accepts readonly and readwrite sources', () => {
    const config = createArcantryConfig({
      docs: 'shared',
      agents: ['codex', 'cursor'],
      operationalSource: 'local',
      sources: [
        { name: 'local', mode: 'operational' },
        { name: 'jira', mode: 'readonly' },
        { name: 'linear', mode: 'readwrite' },
      ],
    });

    expect(config.sources.map((source) => source.name)).toEqual(['local', 'jira', 'linear']);
  });

  it('rejects ambiguous operational ownership and duplicate values', () => {
    expect(() =>
      arcantryConfigSchema.parse({
        schemaVersion: 1,
        agents: ['codex', 'codex'],
        operationalSource: 'local',
        sources: [
          { name: 'local', mode: 'operational' },
          { name: 'local', mode: 'operational' },
        ],
        docs: 'local',
      }),
    ).toThrow();
  });
});

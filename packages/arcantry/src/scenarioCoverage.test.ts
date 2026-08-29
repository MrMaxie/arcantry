import { readFile } from 'node:fs/promises';
import { describe, expect, it } from 'vitest';

type Contract = {
  version: number;
  globalOptions: Array<{ syntax: string; evidence: string }>;
  commands: Array<{ path: string; syntax: string; evidence: string }>;
  trustClaims: Array<{ id: string; evidence: string }>;
  scenarios: Array<{ id: string; command: string }>;
};

describe('project knowledge scenario coverage', () => {
  it('keeps a unique executable evidence id for every public leaf command and trust claim', async () => {
    const contract = JSON.parse(
      await readFile(new URL('../../../contracts/cli-contract.json', import.meta.url), 'utf8'),
    ) as Contract;
    expect(contract.version).toBe(1);
    expect(contract.globalOptions.map(({ syntax }) => syntax)).toEqual([
      '--cwd <path>',
      '--config <path>',
      '-h, --help',
      '-V, --version',
    ]);
    for (const option of contract.globalOptions) expect(option.evidence).not.toBe('');
    expect(contract.commands).toHaveLength(22);
    expect(new Set(contract.commands.map(({ path }) => path)).size).toBe(22);
    const scenarioIds = new Set(contract.scenarios.map(({ id }) => id));
    expect(scenarioIds.size).toBe(contract.scenarios.length);
    for (const command of contract.commands) expect(command.syntax.startsWith(command.path)).toBe(true);
    for (const command of contract.commands) expect(scenarioIds.has(command.evidence), command.path).toBe(true);
    for (const option of contract.globalOptions) expect(scenarioIds.has(option.evidence), option.syntax).toBe(true);
    for (const claim of contract.trustClaims) expect(scenarioIds.has(claim.evidence), claim.id).toBe(true);
    const commands = new Set(['arcantry', ...contract.commands.map(({ path }) => path)]);
    for (const scenario of contract.scenarios) expect(commands.has(scenario.command), scenario.id).toBe(true);
  });
});

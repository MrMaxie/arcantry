import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { afterEach, describe, expect, it, vi } from 'vitest';

const { execaMock } = vi.hoisted(() => ({ execaMock: vi.fn() }));

vi.mock('execa', () => ({ execa: execaMock }));

import { createPublishProgram } from './publish.js';

const result = (overrides: Partial<{ exitCode: number; stdout: string; stderr: string }> = {}) => ({
  exitCode: 0,
  stdout: '',
  stderr: '',
  ...overrides,
});

const runCreateDraft = async (): Promise<void> => {
  await createPublishProgram().parseAsync([
    'node',
    'publish.ts',
    'create-draft-release',
    '--tag',
    'v1.0.0',
    '--artifacts',
    'target/distrib',
  ]);
};

afterEach(() => {
  execaMock.mockReset();
});

describe('GitHub Release publication', () => {
  it('creates a draft when the release does not exist', async () => {
    execaMock
      .mockResolvedValueOnce(result({ exitCode: 1, stderr: 'release not found' }))
      .mockResolvedValueOnce(result());

    await runCreateDraft();

    expect(execaMock).toHaveBeenCalledTimes(2);
    expect(execaMock.mock.calls[1]?.[1]).toEqual(expect.arrayContaining(['release', 'create', 'v1.0.0', '--draft']));
  });

  it('reuses the matching private draft during a safe retry', async () => {
    execaMock
      .mockResolvedValueOnce(result({ stdout: JSON.stringify({ isDraft: true, name: 'v1.0.0', tagName: 'v1.0.0' }) }))
      .mockResolvedValueOnce(result());

    await runCreateDraft();

    expect(execaMock.mock.calls[1]?.[1]).toEqual(expect.arrayContaining(['release', 'upload', 'v1.0.0', '--clobber']));
  });

  it('rejects an already public release', async () => {
    execaMock.mockResolvedValueOnce(
      result({ stdout: JSON.stringify({ isDraft: false, name: 'v1.0.0', tagName: 'v1.0.0' }) }),
    );

    await expect(runCreateDraft()).rejects.toThrow('already public');
    expect(execaMock).toHaveBeenCalledTimes(1);
  });
});

it('pins every GitHub artifact action to a full commit SHA', () => {
  const workflow = readFileSync(resolve('.github/workflows/release.yml'), 'utf8');
  const artifactActions = workflow.match(/actions\/(?:upload|download)-artifact@[^\s]+/gu) ?? [];

  expect(artifactActions).toHaveLength(3);
  for (const action of artifactActions) expect(action).toMatch(/@[0-9a-f]{40}$/u);
});

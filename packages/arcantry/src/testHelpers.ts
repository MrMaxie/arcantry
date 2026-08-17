import { mkdtemp, rm } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { execa } from 'execa';

const fixtureRoots: string[] = [];

export const createFixtureRepository = async (): Promise<string> => {
  const root = await createFixtureDirectory('arcantry-test-');
  await execa('git', ['init', '--quiet'], { cwd: root });
  return root;
};

export const createFixtureDirectory = async (prefix: string): Promise<string> => {
  const root = await mkdtemp(join(tmpdir(), prefix));
  fixtureRoots.push(root);
  return root;
};

export const removeFixtures = async (): Promise<void> => {
  await Promise.all(fixtureRoots.splice(0).map((root) => rm(root, { force: true, recursive: true, maxRetries: 3 })));
};

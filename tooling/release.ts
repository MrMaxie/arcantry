import { writeFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
import { Command } from 'commander';
import {
  checkChangelog,
  checkRelease,
  cutRelease,
  planRelease,
  renderChangelog,
} from '../packages/arcantry/src/release.js';

export * from '../packages/arcantry/src/release.js';

const pullRequestHead = (): string | undefined => {
  if (process.env.GITHUB_ACTIONS !== 'true' || process.env.GITHUB_EVENT_NAME !== 'pull_request') return undefined;
  return process.env.ARCANTRY_PULL_REQUEST_HEAD_SHA?.trim() || undefined;
};

export const createReleaseProgram = (): Command => {
  const program = new Command()
    .name('release.ts')
    .description('Manage the local Arcantry release story.')
    .showHelpAfterError();

  program.command('plan').action(() => {
    process.stdout.write(`${JSON.stringify(planRelease(), null, 2)}\n`);
  });
  program.command('cut').action(() => {
    process.stdout.write(`${JSON.stringify(cutRelease(), null, 2)}\n`);
  });
  program.command('render').action(() => {
    writeFileSync('CHANGELOG.md', renderChangelog(), 'utf8');
  });
  program.command('check').action(() => {
    checkChangelog(process.cwd());
  });
  program.command('seal').action(() => {
    checkRelease(process.cwd(), {}, { pullRequestHead: pullRequestHead() });
  });

  return program;
};

const main = (): void => {
  createReleaseProgram().parse();
};

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) main();

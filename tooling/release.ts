import { writeFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
import { checkRelease, cutRelease, planRelease, renderChangelog } from '../packages/arcantry/src/release.js';

export * from '../packages/arcantry/src/release.js';

function pullRequestHead(): string | undefined {
  if (process.env.GITHUB_ACTIONS !== 'true' || process.env.GITHUB_EVENT_NAME !== 'pull_request') return undefined;
  return process.env.ARCANTRY_PULL_REQUEST_HEAD_SHA?.trim() || undefined;
}

function main(): void {
  const command = process.argv[2];
  if (command === 'plan') {
    process.stdout.write(`${JSON.stringify(planRelease(), null, 2)}\n`);
    return;
  }
  if (command === 'cut') {
    process.stdout.write(`${JSON.stringify(cutRelease(), null, 2)}\n`);
    return;
  }
  if (command === 'render') {
    writeFileSync('CHANGELOG.md', renderChangelog(), 'utf8');
    return;
  }
  if (command === 'check') {
    checkRelease(process.cwd(), {}, { pullRequestHead: pullRequestHead() });
    return;
  }
  throw new Error('usage: release.ts <plan|cut|render|check>');
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) main();

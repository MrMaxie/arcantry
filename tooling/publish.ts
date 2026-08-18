import { pathToFileURL } from 'node:url';
import { validateNpmPublication } from '../packages/arcantry/src/release.js';

export * from '../packages/arcantry/src/release.js';

function option(name: string): string | undefined {
  const index = process.argv.indexOf(name);
  return index === -1 ? undefined : process.argv[index + 1];
}

function main(): void {
  if (process.argv[2] !== 'check') throw new Error('usage: publish.ts check --tag <vX.Y.Z>');
  const tag = option('--tag');
  if (!tag) throw new Error('usage: publish.ts check --tag <vX.Y.Z>');
  process.stdout.write(`${JSON.stringify(validateNpmPublication(tag), null, 2)}\n`);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) main();

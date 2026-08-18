import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const root = process.cwd();
const packageManifest = JSON.parse(readFileSync(join(root, 'packages', 'arcantry', 'package.json'), 'utf8')) as {
  name: string;
};

const publicIdentitySurfaces = [
  'src/content/docs/getting-started.mdx',
  'src/components/ArcantryCommandPicker.astro',
  'src/components/ArcantryHero.astro',
  'src/components/ArcantryCopyCommands.astro',
  'packages/arcantry/scripts/check-package.mjs',
];

describe('npm package identity', () => {
  it('uses the Arcantry organization scope', () => {
    expect(packageManifest.name).toBe('@arcantry/arcantry');
  });

  it('does not retain the personal npm scope in public or package surfaces', () => {
    for (const relativePath of publicIdentitySurfaces) {
      expect(readFileSync(join(root, relativePath), 'utf8'), relativePath).not.toContain('@maxiedev/');
    }
  });

  it('keeps authored launcher examples aligned with the manifest', () => {
    const commandPicker = readFileSync(join(root, 'src', 'components', 'ArcantryCommandPicker.astro'), 'utf8');
    expect(commandPicker).toContain('const packageName = packageManifest.name');
    expect(commandPicker).toContain('value: `npx ${packageName} repo inspect`');
    expect(commandPicker).toContain('value: `pnpm dlx ${packageName} repo inspect`');
    expect(commandPicker).toContain('value: `nubx ${packageName} repo inspect`');
  });
});

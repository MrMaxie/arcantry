import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const root = process.cwd();
const packageManifest = JSON.parse(readFileSync(join(root, 'packages', 'arcantry', 'package.json'), 'utf8')) as {
  name: string;
};

const publicIdentitySurfaces = [
  'src/content/docs/getting-started.md',
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
    const gettingStarted = readFileSync(join(root, 'src', 'content', 'docs', 'getting-started.md'), 'utf8');
    expect(gettingStarted).toContain(`npx ${packageManifest.name} repo inspect`);
    expect(gettingStarted).toContain(`pnpm dlx ${packageManifest.name} repo inspect`);
    expect(gettingStarted).toContain(`nubx ${packageManifest.name} repo inspect`);
  });
});

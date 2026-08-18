import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const root = process.cwd();
const packageManifest = JSON.parse(readFileSync(join(root, 'packages', 'arcantry', 'package.json'), 'utf8')) as {
  name: string;
  version: string;
};
const workspaceManifest = JSON.parse(readFileSync(join(root, 'package.json'), 'utf8')) as {
  name: string;
  private: boolean;
};

const publicIdentitySurfaces = [
  'src/content/docs/getting-started.mdx',
  'src/content/docs/lifecycle/releases.mdx',
  'src/components/ArcantryCommandPicker.astro',
  'src/components/ArcantryHero.astro',
  'src/components/ArcantryCopyCommands.astro',
  'packages/arcantry/scripts/check-package.mjs',
];

const agentManifestPaths = [
  '.codex-plugin/plugin.json',
  '.claude-plugin/plugin.json',
];

describe('npm package identity', () => {
  it('uses the concise public package name', () => {
    expect(packageManifest.name).toBe('arcantry');
  });

  it('keeps the private workspace identity distinct', () => {
    expect(workspaceManifest).toMatchObject({ name: 'arcantry-workspace', private: true });
    expect(workspaceManifest.name).not.toBe(packageManifest.name);
  });

  it('does not retain previous npm scopes in public or package surfaces', () => {
    for (const relativePath of publicIdentitySurfaces) {
      const source = readFileSync(join(root, relativePath), 'utf8');
      expect(source, relativePath).not.toContain('@maxiedev/');
      expect(source, relativePath).not.toContain('@arcantry/arcantry');
    }
  });

  it('keeps authored launcher examples aligned with the manifest', () => {
    const commandPicker = readFileSync(join(root, 'src', 'components', 'ArcantryCommandPicker.astro'), 'utf8');
    expect(commandPicker).toContain('const packageName = packageManifest.name');
    expect(commandPicker).toContain('value: `npx ${packageName} repo inspect`');
    expect(commandPicker).toContain('value: `pnpm dlx ${packageName} repo inspect`');
    expect(commandPicker).toContain('value: `nubx ${packageName} repo inspect`');
  });

  it('keeps supported agent manifests aligned with the package identity', () => {
    for (const relativePath of agentManifestPaths) {
      const manifest = JSON.parse(readFileSync(join(root, relativePath), 'utf8')) as { name: string; version: string };
      expect(manifest, relativePath).toMatchObject({ name: packageManifest.name, version: packageManifest.version });
    }
  });
});

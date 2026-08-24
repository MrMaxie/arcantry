import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { nativeTargets } from './native-targets.js';

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
  'apps/docs/src/content/docs/getting-started.mdx',
  'apps/docs/src/content/docs/lifecycle/releases.mdx',
  'apps/docs/src/components/ArcantryCommandPicker.astro',
  'apps/docs/src/components/ArcantryAgentPrompt.astro',
  'apps/docs/src/components/ArcantryHero.astro',
  'apps/docs/src/components/ArcantryCopyCommands.astro',
  'packages/arcantry/scripts/check-package.mjs',
];

const agentManifestPaths = ['.codex-plugin/plugin.json', '.claude-plugin/plugin.json'];

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

  it('keeps authored installation and launcher examples aligned with the manifest', () => {
    const commandPicker = readFileSync(
      join(root, 'apps', 'docs', 'src', 'components', 'ArcantryCommandPicker.astro'),
      'utf8',
    );
    const gettingStarted = readFileSync(
      join(root, 'apps', 'docs', 'src', 'content', 'docs', 'getting-started.mdx'),
      'utf8',
    );
    const agentPrompt = readFileSync(
      join(root, 'apps', 'docs', 'src', 'components', 'ArcantryAgentPrompt.astro'),
      'utf8',
    );
    expect(commandPicker).toContain('const packageName = packageManifest.name');
    expect(commandPicker).toContain('value: `npm install --global ${packageName}`');
    expect(commandPicker).toContain('value: `npx ${packageName} repo inspect`');
    expect(commandPicker).toContain('value: `pnpm dlx ${packageName} repo inspect`');
    expect(commandPicker).not.toContain('value: `bunx ${packageName} repo inspect`');
    expect(commandPicker).toContain('value: `nubx ${packageName} repo inspect`');
    expect(commandPicker).toContain('arcantry-installer.ps1');
    expect(commandPicker).toContain('arcantry-installer.sh');
    expect(commandPicker).not.toContain('Install native CLI');
    expect(commandPicker).not.toContain('Run once');
    expect(gettingStarted).toContain('cargo install --locked --path crates/arcantry-cli');
    expect(gettingStarted).toContain('public 1.0 package or GitHub Release');
    expect(gettingStarted).toContain('<ArcantryAgentPrompt variant="full" />');
    expect(agentPrompt).toContain('Install Arcantry on this computer using the official getting-started guide.');
    expect(agentPrompt).toContain('Do not adopt Arcantry into a repository or change project files unless I ask.');
  });

  it('keeps supported agent manifests aligned with the package identity', () => {
    for (const relativePath of agentManifestPaths) {
      const manifest = JSON.parse(readFileSync(join(root, relativePath), 'utf8')) as { name: string; version: string };
      expect(manifest, relativePath).toMatchObject({ name: packageManifest.name, version: packageManifest.version });
    }
  });

  it('keeps native target metadata aligned with platform packages', () => {
    const main = JSON.parse(readFileSync(join(root, 'packages', 'arcantry', 'package.json'), 'utf8')) as {
      version: string;
      optionalDependencies: Record<string, string>;
    };
    expect(new Set(nativeTargets.map((target) => target.triple)).size).toBe(nativeTargets.length);
    for (const target of nativeTargets) {
      const manifest = JSON.parse(
        readFileSync(join(root, 'packages', target.packageDirectory, 'package.json'), 'utf8'),
      ) as { name: string; version: string; os: string[]; cpu: string[]; libc?: string[] };
      expect(manifest, target.triple).toMatchObject({
        name: target.packageName,
        version: main.version,
        os: [target.os],
        cpu: [target.cpu],
      });
      expect(manifest.libc, target.triple).toBeUndefined();
      expect(main.optionalDependencies[target.packageName], target.triple).toBe(main.version);
    }
  });

  it('runs compatibility and native smoke coverage across the declared release matrix', () => {
    const workflow = readFileSync(join(root, '.github', 'workflows', 'release.yml'), 'utf8');
    for (const target of nativeTargets) expect(workflow, target.triple).toContain(`target: ${target.triple}`);
    expect(workflow).toContain('run: cargo test --workspace');
    expect(workflow).toContain('alpine:3.23');
    expect(workflow).toContain('just package-target-smoke "${{ matrix.target }}"');
    expect(workflow).toContain('installer-smoke:');
    expect(workflow).toContain('windows-2025');
    expect(workflow).toContain('Smoke installers on their native operating system');
    expect(workflow).toMatch(/publish:\s+needs:\s+- assemble\s+- installer-smoke/u);
  });

  it('documents the active trusted-publishing workflow', () => {
    const releaseGuide = readFileSync(
      join(root, 'apps', 'docs', 'src', 'content', 'docs', 'lifecycle', 'releases.mdx'),
      'utf8',
    );

    expect(releaseGuide).toContain('workflow `release.yml`');
    expect(releaseGuide).not.toContain('workflow `publish-npm.yml`');
  });
});

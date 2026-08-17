import { mkdtempSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import {
  bumpVersion,
  checkChangelog,
  cutRelease,
  highestImpact,
  parseReleaseArtifact,
  planRelease,
  renderChangelog,
  validateDistributionVersions,
  validateReleaseState,
} from './release.js';

const release = (impact = 'minor', visibility = 'public') => `---
category: changed
impact: ${impact}
visibility: ${visibility}
components:
  - repository-lifecycle
---

# Better release history

Release notes come from delivered changes rather than commit messages.
`;

function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), 'arcantry-'));
  const change = join(root, 'openspec', 'changes', 'archive', '2026-08-16-release-history');
  mkdirSync(change, { recursive: true });
  writeFileSync(join(change, 'release.md'), release());
  return root;
}

function manifest(root: string, version = '0.1.0', changes = ['release-history']): void {
  mkdirSync(join(root, 'releases'), { recursive: true });
  writeFileSync(
    join(root, 'releases', `${version}.yaml`),
    `version: ${version}\ndate: 2026-08-16\nchanges:\n${changes.map((change) => `  - ${change}`).join('\n')}\n`,
  );
}

describe('release artifact', () => {
  it('parses machine metadata and human release copy', () => {
    expect(parseReleaseArtifact(release())).toEqual({
      category: 'changed',
      impact: 'minor',
      visibility: 'public',
      components: ['repository-lifecycle'],
      title: 'Better release history',
      body: 'Release notes come from delivered changes rather than commit messages.',
    });
  });

  it('rejects unsupported metadata', () => {
    expect(() => parseReleaseArtifact(release('feature'))).toThrow('invalid release impact');
  });

  it('rejects malformed component identifiers', () => {
    expect(() => parseReleaseArtifact(release().replace('repository-lifecycle', 'Repository Lifecycle'))).toThrow(
      'release components',
    );
  });
});

describe('SemVer planning', () => {
  it('uses the highest impact', () => {
    expect(highestImpact(['patch', 'major', 'minor'])).toBe('major');
  });

  it('bumps the requested segment', () => {
    expect(bumpVersion('1.4.7', 'patch')).toBe('1.4.8');
    expect(bumpVersion('1.4.7', 'minor')).toBe('1.5.0');
    expect(bumpVersion('1.4.7', 'major')).toBe('2.0.0');
  });

  it('plans only archived and unassigned changes', () => {
    expect(planRelease(fixture())).toEqual({
      current: '0.0.0',
      next: '0.1.0',
      impact: 'minor',
      changes: ['release-history'],
    });
  });

  it('cuts the computed release manifest', () => {
    const root = fixture();
    expect(cutRelease(root, '2026-08-16')).toEqual({
      version: '0.1.0',
      date: '2026-08-16',
      changes: ['release-history'],
    });
    expect(readFileSync(join(root, 'releases', '0.1.0.yaml'), 'utf8')).toContain('release-history');
  });
});

describe('release state validation', () => {
  it('rejects unknown archived changes', () => {
    const root = fixture();
    manifest(root, '0.1.0', ['missing-change']);
    expect(() => validateReleaseState(root)).toThrow('references unknown archived change');
  });

  it('rejects changes assigned to multiple releases', () => {
    const root = fixture();
    manifest(root);
    writeFileSync(
      join(root, 'releases', '0.1.1.yaml'),
      'version: 0.1.1\ndate: 2026-08-17\nchanges:\n  - release-history\n',
    );
    expect(() => validateReleaseState(root)).toThrow('assigned more than once');
  });

  it('requires distributable versions to match the latest release', () => {
    const root = fixture();
    manifest(root);
    const packageRoot = join(root, 'packages', 'arcantry');
    mkdirSync(packageRoot, { recursive: true });
    writeFileSync(join(packageRoot, 'package.json'), '{"version":"0.2.0"}\n');

    expect(() => validateDistributionVersions(root)).toThrow('distribution version must match release 0.1.0');
    writeFileSync(join(packageRoot, 'package.json'), '{"version":"0.1.0"}\n');
    expect(() => validateDistributionVersions(root)).not.toThrow();
  });
});

describe('changelog rendering', () => {
  it('renders dated public entries with OpenSpec provenance', () => {
    const root = fixture();
    manifest(root);
    const changelog = renderChangelog(root);

    expect(changelog).toContain('## 0.1.0 - 2026-08-16');
    expect(changelog).toContain('<!-- openspec: release-history -->');
    expect(changelog).toContain('#### Better release history');
  });

  it('omits internal release prose', () => {
    const root = fixture();
    const change = join(root, 'openspec', 'changes', 'archive', '2026-08-17-internal-cleanup');
    mkdirSync(change, { recursive: true });
    writeFileSync(join(change, 'release.md'), release('patch', 'internal'));
    manifest(root, '0.1.0', ['release-history', 'internal-cleanup']);

    expect(renderChangelog(root).match(/Better release history/g)).toHaveLength(1);
  });

  it('detects a stale committed changelog', () => {
    const root = fixture();
    manifest(root);
    writeFileSync(join(root, 'CHANGELOG.md'), '# Changelog\n');
    expect(() => checkChangelog(root)).toThrow('CHANGELOG.md is stale');

    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    expect(() => checkChangelog(root)).not.toThrow();
  });
});

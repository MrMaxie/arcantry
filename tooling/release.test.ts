import { mkdtempSync, mkdirSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { bumpVersion, highestImpact, parseReleaseArtifact, planRelease, renderChangelog } from './release.js';

const release = (impact = 'minor', visibility = 'public') => `---
category: changed
impact: ${impact}
visibility: ${visibility}
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

describe('release artifact', () => {
  it('parses machine metadata and human release copy', () => {
    expect(parseReleaseArtifact(release())).toEqual({
      category: 'changed',
      impact: 'minor',
      visibility: 'public',
      title: 'Better release history',
      body: 'Release notes come from delivered changes rather than commit messages.'
    });
  });

  it('rejects unsupported metadata', () => {
    expect(() => parseReleaseArtifact(release('feature'))).toThrow('invalid release impact');
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
      changes: ['release-history']
    });
  });
});

describe('changelog rendering', () => {
  it('renders public archived changes referenced by a manifest', () => {
    const root = fixture();
    mkdirSync(join(root, 'releases'));
    writeFileSync(join(root, 'releases', '0.1.0.yaml'), 'version: 0.1.0\nchanges:\n  - release-history\n');

    expect(renderChangelog(root)).toContain('## 0.1.0');
    expect(renderChangelog(root)).toContain('#### Better release history');
  });

  it('omits internal release prose', () => {
    const root = fixture();
    const change = join(root, 'openspec', 'changes', 'archive', '2026-08-17-internal-cleanup');
    mkdirSync(change, { recursive: true });
    writeFileSync(join(change, 'release.md'), release('patch', 'internal'));
    mkdirSync(join(root, 'releases'));
    writeFileSync(join(root, 'releases', '0.1.0.yaml'), 'version: 0.1.0\nchanges:\n  - release-history\n  - internal-cleanup\n');

    expect(renderChangelog(root).match(/Better release history/g)).toHaveLength(1);
  });
});

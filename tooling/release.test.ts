import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import {
  bumpVersion,
  checkChangelog,
  checkRelease,
  cutRelease,
  highestImpact,
  parseReleaseArtifact,
  planRelease,
  renderChangelog,
  validateDistributionVersions,
  validateNpmPublication,
  validateReleaseSeal,
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

function npmPackage(root: string, name = 'arcantry', version = '0.1.0'): void {
  const packageRoot = join(root, 'packages', 'arcantry');
  mkdirSync(packageRoot, { recursive: true });
  writeFileSync(
    join(packageRoot, 'package.json'),
    `${JSON.stringify(
      {
        name,
        version,
        repository: { type: 'git', url: 'https://github.com/MrMaxie/arcantry.git' },
      },
      null,
      2,
    )}\n`,
  );
}

function git(root: string, args: string[]): string {
  return execFileSync('git', args, { cwd: root, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
}

function commitAll(root: string, message: string): void {
  try {
    git(root, ['rev-parse', '--git-dir']);
  } catch {
    git(root, ['init']);
    git(root, ['config', 'user.name', 'Arcantry Tests']);
    git(root, ['config', 'user.email', 'tests@arcantry.invalid']);
  }
  git(root, ['add', '.']);
  git(root, ['commit', '-m', message]);
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

  it('requires every release artifact to identify an affected component', () => {
    expect(() => parseReleaseArtifact(release().replace('components:\n  - repository-lifecycle\n', ''))).toThrow(
      'non-empty array of component identifiers',
    );
    expect(() => parseReleaseArtifact(release().replace('  - repository-lifecycle', ''))).toThrow(
      'non-empty array of component identifiers',
    );
  });

  it('rejects a long whitespace-only title without ambiguous regular-expression work', () => {
    const malformed = release().replace('# Better release history', `# ${' '.repeat(100_000)}`);

    expect(() => parseReleaseArtifact(malformed)).toThrow('level-one title');
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

  it('rejects completed changes without a SemVer impact', () => {
    const root = fixture();
    const path = join(root, 'openspec', 'changes', 'archive', '2026-08-16-release-history', 'release.md');
    writeFileSync(path, release('none'));

    expect(() => planRelease(root)).toThrow('completed changes must declare a SemVer impact: release-history');
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
    const codexPluginRoot = join(root, '.codex-plugin');
    const claudePluginRoot = join(root, '.claude-plugin');
    mkdirSync(packageRoot, { recursive: true });
    mkdirSync(codexPluginRoot, { recursive: true });
    mkdirSync(claudePluginRoot, { recursive: true });
    writeFileSync(join(packageRoot, 'package.json'), '{"version":"0.2.0"}\n');

    expect(() => validateDistributionVersions(root)).toThrow('distribution version must match release 0.1.0');
    writeFileSync(join(packageRoot, 'package.json'), '{"version":"0.1.0"}\n');
    writeFileSync(join(codexPluginRoot, 'plugin.json'), '{"version":"0.1.0"}\n');
    writeFileSync(join(claudePluginRoot, 'plugin.json'), '{"version":"0.1.0"}\n');
    expect(() => validateDistributionVersions(root)).not.toThrow();
    writeFileSync(join(claudePluginRoot, 'plugin.json'), '{"version":"0.2.0"}\n');
    expect(() => validateDistributionVersions(root)).toThrow('distribution version must match release 0.1.0');
  });
});

describe('changelog rendering', () => {
  it('renders dated public entries with OpenSpec provenance', () => {
    const root = fixture();
    manifest(root);
    const changelog = renderChangelog(root);

    expect(changelog).toContain('https://keepachangelog.com/en/2.0.0/');
    expect(changelog).toContain('## [Unreleased]');
    expect(changelog).toContain('## [0.1.0] - 2026-08-16');
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

  it('composes release meaning from multiple OpenSpec sources in a monorepo', () => {
    const root = fixture();
    const component = join(root, 'components', 'api', 'openspec', 'changes', 'archive', '2026-08-17-api-change');
    mkdirSync(component, { recursive: true });
    writeFileSync(join(component, 'release.md'), release('patch').replace('Better release history', 'API change'));
    manifest(root, '0.1.0', ['release-history', 'api-change']);

    const changelog = renderChangelog(root, { openSpecPaths: ['openspec', 'components/api/openspec'] });

    expect(changelog).toContain('#### Better release history');
    expect(changelog).toContain('#### API change');
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

describe('release completion', () => {
  it('rejects active and unassigned OpenSpec changes', () => {
    const root = fixture();
    expect(() => checkRelease(root)).toThrow('archived OpenSpec changes are not assigned to a release');

    manifest(root);
    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    commitAll(root, 'release: cut 0.1.0');
    const active = join(root, 'openspec', 'changes', 'in-progress');
    mkdirSync(active, { recursive: true });
    writeFileSync(join(active, 'proposal.md'), '# Why\n');

    expect(() => checkRelease(root)).toThrow('active OpenSpec changes are not release-complete: in-progress');
  });

  it('accepts a clean repository sealed by its latest release manifest', () => {
    const root = fixture();
    manifest(root);
    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    commitAll(root, 'release: cut 0.1.0');

    expect(() => validateReleaseSeal(root)).not.toThrow();
    expect(() => checkRelease(root)).not.toThrow();
  });

  it('accepts the sealed pull request head when checking a synthetic merge commit', () => {
    const root = fixture();
    commitAll(root, 'chore: initialize repository');
    manifest(root);
    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    commitAll(root, 'release: cut 0.1.0');
    const releaseHead = git(root, ['rev-parse', 'HEAD']);

    git(root, ['checkout', '-b', 'base', 'HEAD^']);
    writeFileSync(join(root, 'base.txt'), 'base change\n');
    commitAll(root, 'fix: base change');
    git(root, ['merge', '--no-ff', releaseHead, '-m', 'test: synthetic pull request merge']);

    expect(() => validateReleaseSeal(root)).toThrow('repository HEAD is not sealed by release 0.1.0');
    expect(() => validateReleaseSeal(root, {}, { pullRequestHead: releaseHead })).not.toThrow();
  });

  it('rejects a pull request release head that is not a direct merge parent', () => {
    const root = fixture();
    manifest(root);
    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    commitAll(root, 'release: cut 0.1.0');
    const releaseHead = git(root, ['rev-parse', 'HEAD']);
    writeFileSync(join(root, 'later.txt'), 'later work\n');
    commitAll(root, 'fix: later work');

    expect(() => validateReleaseSeal(root, {}, { pullRequestHead: releaseHead })).toThrow(
      'pull request release head is not a direct parent of the checked-out merge commit',
    );
  });

  it('rejects a commit after the latest release seal', () => {
    const root = fixture();
    manifest(root);
    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    commitAll(root, 'release: cut 0.1.0');
    writeFileSync(join(root, 'implementation.txt'), 'unreleased behavior\n');
    commitAll(root, 'fix: change behavior');

    expect(() => validateReleaseSeal(root)).toThrow('repository HEAD is not sealed by release 0.1.0');
  });

  it('does not let a later edit to the same manifest create a new seal', () => {
    const root = fixture();
    manifest(root);
    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    commitAll(root, 'release: cut 0.1.0');
    writeFileSync(
      join(root, 'releases', '0.1.0.yaml'),
      'version: 0.1.0\ndate: 2026-08-17\nchanges:\n  - release-history\n',
    );
    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    commitAll(root, 'release: rewrite 0.1.0');

    expect(() => validateReleaseSeal(root)).toThrow('repository HEAD is not sealed by release 0.1.0');
  });

  it('accepts postfactum OpenSpec recovery in a newer internal release', () => {
    const root = fixture();
    manifest(root);
    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    commitAll(root, 'release: cut 0.1.0');
    writeFileSync(join(root, 'implementation.txt'), 'recovered behavior\n');
    commitAll(root, 'fix: change behavior');
    expect(() => checkRelease(root)).toThrow('repository HEAD is not sealed by release 0.1.0');

    const recovered = join(root, 'openspec', 'changes', 'archive', '2026-08-17-recovered-change');
    mkdirSync(recovered, { recursive: true });
    writeFileSync(
      join(recovered, 'release.md'),
      release('patch').replace('Better release history', 'Recovered behavior'),
    );
    manifest(root, '0.1.1', ['recovered-change']);
    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    commitAll(root, 'release: cut 0.1.1');

    expect(() => checkRelease(root)).not.toThrow();
  });
});

describe('npm publication', () => {
  function sealedNpmRelease(): string {
    const root = fixture();
    manifest(root);
    npmPackage(root);
    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    commitAll(root, 'release: cut 0.1.0');
    git(root, ['tag', 'v0.1.0']);
    return root;
  }

  it('accepts the public package at the matching sealed release tag', () => {
    expect(validateNpmPublication('v0.1.0', sealedNpmRelease())).toEqual({
      packageName: 'arcantry',
      version: '0.1.0',
      tag: 'v0.1.0',
      repositoryUrl: 'https://github.com/MrMaxie/arcantry.git',
    });
  });

  it('rejects malformed or mismatched release tags', () => {
    const root = sealedNpmRelease();
    expect(() => validateNpmPublication('0.1.0', root)).toThrow('invalid npm release tag');
    expect(() => validateNpmPublication('v0.1.1', root)).toThrow('does not match release 0.1.0');
  });

  it('rejects the wrong package identity', () => {
    const root = fixture();
    manifest(root);
    npmPackage(root, '@other/arcantry');
    writeFileSync(join(root, 'CHANGELOG.md'), renderChangelog(root));
    commitAll(root, 'release: cut 0.1.0');
    git(root, ['tag', 'v0.1.0']);

    expect(() => validateNpmPublication('v0.1.0', root)).toThrow('npm package name must be arcantry');
  });

  it('rejects an unsealed commit after the release tag', () => {
    const root = sealedNpmRelease();
    writeFileSync(join(root, 'later.txt'), 'later\n');
    commitAll(root, 'fix: later work');

    expect(() => validateNpmPublication('v0.1.0', root)).toThrow('repository HEAD is not sealed by release 0.1.0');
  });
});

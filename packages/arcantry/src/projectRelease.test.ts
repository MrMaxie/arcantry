import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { applyProjectPlan } from './projectPlan.js';
import { checkProjectRelease, inspectReleasePlan, planReleaseBaseline, planReleaseCut } from './projectRelease.js';
import { resolveProject } from './projectConfig.js';
import { createFixtureDirectory, removeFixtures } from './testHelpers.js';

afterEach(removeFixtures);

const toolVersion = '1.0.0';

describe('configured project release planning', () => {
  it('baselines a Cargo workspace and cuts public and internal changes atomically', async () => {
    const root = await createReleaseProject();
    const project = await resolveProject({ cwd: root, toolVersion });

    const baseline = await planReleaseBaseline(project, '0.8.0', '2026-06-11', toolVersion);
    expect(baseline.conflicts).toEqual([]);
    expect(baseline.operations.map((operation) => operation.path)).toEqual(['releases/0.8.0.yaml', 'CHANGELOG.md']);
    await expect(readFile(join(root, 'releases/0.8.0.yaml'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });

    await applyProjectPlan(baseline, toolVersion);
    expect(await readFile(join(root, 'releases/0.8.0.yaml'), 'utf8')).toContain('baseline: true');
    const baselineChangelog = await readFile(join(root, 'CHANGELOG.md'), 'utf8');
    expect(baselineChangelog).toContain('Arcantry release baseline: 0.8.0 (2026-06-11)');
    expect(baselineChangelog).not.toContain('## [0.8.0]');
    expect(inspectReleasePlan(await resolveProject({ cwd: root, toolVersion }))).toMatchObject({
      current: '0.8.0',
      next: '0.8.0',
      impact: 'none',
      changes: [],
    });

    await writeArchivedChange(root, '2026-08-01-public-change', 'added', 'minor', 'public', 'Public capability');
    await writeArchivedChange(root, '2026-08-02-internal-change', 'changed', 'major', 'internal', 'Private migration detail');
    const cut = await planReleaseCut(await resolveProject({ cwd: root, toolVersion }), '2026-08-18', toolVersion);
    expect(cut.conflicts).toEqual([]);
    expect(cut.operations.map((operation) => operation.path)).toEqual([
      'releases/1.0.0.yaml',
      'Cargo.toml',
      'CHANGELOG.md',
    ]);

    await applyProjectPlan(cut, toolVersion);
    const cargo = await readFile(join(root, 'Cargo.toml'), 'utf8');
    expect(cargo).toContain('[workspace.package]\nversion = "1.0.0"');
    expect(cargo).toContain('[package]\nname = "fixture"\nversion = "9.9.9"');
    const changelog = await readFile(join(root, 'CHANGELOG.md'), 'utf8');
    expect(changelog).toContain('Public capability');
    expect(changelog).not.toContain('Private migration detail');
    expect(changelog).toContain('[1.0.0]: https://github.com/example/cadder/compare/v0.8.0...v1.0.0');
    const releasedProject = await resolveProject({ cwd: root, toolVersion });
    expect(() => checkProjectRelease(releasedProject, false)).not.toThrow();
  });

  it('refuses an outdated plan before changing any release file', async () => {
    const root = await createReleaseProject();
    const project = await resolveProject({ cwd: root, toolVersion });
    const baseline = await planReleaseBaseline(project, '0.8.0', '2026-06-11', toolVersion);
    await writeFile(join(root, 'CHANGELOG.md'), 'User changed this after planning.\n');

    await expect(applyProjectPlan(baseline, toolVersion)).rejects.toThrow('changed after the plan was created');
    await expect(readFile(join(root, 'releases/0.8.0.yaml'), 'utf8')).rejects.toMatchObject({ code: 'ENOENT' });
    expect(await readFile(join(root, 'Cargo.toml'), 'utf8')).toContain('version = "0.8.0"');
  });

  it('reports a baseline conflict when configured versions drift', async () => {
    const root = await createReleaseProject();
    const plan = await planReleaseBaseline(
      await resolveProject({ cwd: root, toolVersion }),
      '1.0.0',
      '2026-08-18',
      toolVersion,
    );

    expect(plan.operations).toEqual([]);
    expect(plan.conflicts.join('\n')).toContain('Cargo.toml contains 0.8.0');
  });

  it('rejects non-canonical versions and invalid calendar dates', async () => {
    const root = await createReleaseProject();
    const project = await resolveProject({ cwd: root, toolVersion });

    const prefixed = await planReleaseBaseline(project, 'v0.8.0', '2026-06-11', toolVersion);
    const invalidDate = await planReleaseBaseline(project, '0.8.0', '2026-02-31', toolVersion);

    expect(prefixed.operations).toEqual([]);
    expect(prefixed.conflicts.join('\n')).toContain('full stable SemVer');
    expect(invalidDate.operations).toEqual([]);
    expect(invalidDate.conflicts.join('\n')).toContain('Invalid release date');
  });

  it('keeps normal consistency separate from final release sealing', async () => {
    const root = await createReleaseProject();
    const project = await resolveProject({ cwd: root, toolVersion });
    await applyProjectPlan(await planReleaseBaseline(project, '0.8.0', '2026-06-11', toolVersion), toolVersion);
    await mkdir(join(root, 'openspec', 'changes', 'active-work'));

    const current = await resolveProject({ cwd: root, toolVersion });
    expect(() => checkProjectRelease(current, false)).not.toThrow();
    expect(() => checkProjectRelease(current, true)).toThrow('active OpenSpec changes are not release-complete: active-work');
  });
});

async function createReleaseProject(): Promise<string> {
  const root = await createFixtureDirectory('arcantry-release-project-');
  await mkdir(join(root, 'openspec', 'changes', 'archive'), { recursive: true });
  await writeFile(join(root, 'Cargo.toml'), `[workspace]
members = []

[workspace.package]
version = "0.8.0"
edition = "2024"

[package]
name = "fixture"
version = "9.9.9"
`);
  await writeFile(join(root, 'arcantry.toml'), `config_version = 1

[sources.openspec]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.changelog]
kind = "changelog"
path = "CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["openspec"]

[release]
adapter = "openspec-release@1"
manifests_path = "releases"
changelog_source = "changelog"
tag_prefix = "v"
repository_url = "https://github.com/example/cadder"

[[release.version_sources]]
path = "Cargo.toml"
adapter = "cargo-workspace@1"
`);
  return root;
}

async function writeArchivedChange(
  root: string,
  directory: string,
  category: string,
  impact: string,
  visibility: string,
  title: string,
): Promise<void> {
  const archive = join(root, 'openspec', 'changes', 'archive', directory);
  await mkdir(archive, { recursive: true });
  await writeFile(join(archive, 'release.md'), `---
category: ${category}
impact: ${impact}
visibility: ${visibility}
components:
  - cli
---

# ${title}

Delivered outcome for ${title.toLowerCase()}.
`);
}

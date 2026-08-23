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

  it('plans independent unit versions from shared OpenSpec outcomes', async () => {
    const root = await createMultiUnitProject('independent');
    let project = await resolveProject({ cwd: root, toolVersion });
    await applyProjectPlan(await planReleaseBaseline(project, '1.0.0', '2026-08-01', toolVersion, 'core'), toolVersion);
    project = await resolveProject({ cwd: root, toolVersion });
    await applyProjectPlan(await planReleaseBaseline(project, '2.0.0', '2026-08-01', toolVersion, 'app'), toolVersion);
    await writeSchemaAwareChange(root, '2026-08-20-shared-outcome', `---
category: added
impact: patch
unit_impacts:
  app: minor
visibility: public
components:
  - product:core
  - product:app
---

# Improve both release units

Each matching unit receives the shared product outcome.
`);

    project = await resolveProject({ cwd: root, toolVersion });
    expect(inspectReleasePlan(project, 'core')).toMatchObject({ unit: 'core', topology: 'independent', current: '1.0.0', next: '1.0.1', impact: 'patch', ready: true });
    expect(inspectReleasePlan(project, 'app')).toMatchObject({ unit: 'app', topology: 'independent', current: '2.0.0', next: '2.1.0', impact: 'minor', ready: true });
    await applyProjectPlan(await planReleaseCut(project, '2026-08-23', toolVersion, 'core'), toolVersion);
    const manifest = await readFile(join(root, 'releases/core/1.0.1.yaml'), 'utf8');
    expect(manifest).toContain('format: 2');
    expect(manifest).toContain('unit: core');
    expect(inspectReleasePlan(await resolveProject({ cwd: root, toolVersion }), 'app').changes).toEqual(['shared-outcome']);
  });

  it('keeps composed dependency adoption explicit', async () => {
    const root = await createMultiUnitProject('composed');
    let project = await resolveProject({ cwd: root, toolVersion });
    await applyProjectPlan(await planReleaseBaseline(project, '1.0.0', '2026-08-01', toolVersion, 'core'), toolVersion);
    project = await resolveProject({ cwd: root, toolVersion });
    await applyProjectPlan(await planReleaseBaseline(project, '2.0.0', '2026-08-01', toolVersion, 'app'), toolVersion);
    await writeSchemaAwareChange(root, '2026-08-20-core-update', releaseArtifact('Patch core runtime', 'patch', ['product:core']));
    project = await resolveProject({ cwd: root, toolVersion });
    await applyProjectPlan(await planReleaseCut(project, '2026-08-22', toolVersion, 'core'), toolVersion);
    expect(await readFile(join(root, 'apps/app/package.json'), 'utf8')).toContain('"version": "2.0.0"');
    expect(() => checkProjectRelease(project, true, 'app')).toThrow('pending dependencies');

    await writeSchemaAwareChange(root, '2026-08-21-app-update', releaseArtifact('Update app integration', 'patch', ['product:app']));
    project = await resolveProject({ cwd: root, toolVersion });
    expect(inspectReleasePlan(project, 'app')).toMatchObject({ pendingDependencies: { core: '1.0.1' }, ready: false });
    expect((await planReleaseCut(project, '2026-08-23', toolVersion, 'app')).conflicts.join('\n')).toContain('unacknowledged pending dependencies');

    await writeSchemaAwareChange(root, '2026-08-21-app-update', releaseArtifact('Update app integration', 'patch', ['product:app'], `dependency_updates:
  app:
    - core
`));
    project = await resolveProject({ cwd: root, toolVersion });
    expect(inspectReleasePlan(project, 'app').ready).toBe(true);
    await applyProjectPlan(await planReleaseCut(project, '2026-08-23', toolVersion, 'app'), toolVersion);
    expect(await readFile(join(root, 'releases/app/2.0.1.yaml'), 'utf8')).toContain('core: 1.0.1');
  });

  it('skips non-release schemas and rejects unmatched release-bearing outcomes', async () => {
    const root = await createMultiUnitProject('independent');
    await mkdir(join(root, 'openspec/schemas/notes'), { recursive: true });
    await writeFile(join(root, 'openspec/schemas/notes/schema.yaml'), 'name: notes\nversion: 1\nartifacts: []\n');
    const notes = join(root, 'openspec/changes/archive/2026-08-20-notes-only');
    await mkdir(notes, { recursive: true });
    await writeFile(join(notes, '.openspec.yaml'), 'schema: notes\n');
    let project = await resolveProject({ cwd: root, toolVersion });
    expect((await planReleaseBaseline(project, '1.0.0', '2026-08-01', toolVersion, 'core')).conflicts).toEqual([]);

    await writeSchemaAwareChange(root, '2026-08-21-unowned', releaseArtifact('Unowned surface', 'patch', ['product:other']));
    project = await resolveProject({ cwd: root, toolVersion });
    expect((await planReleaseBaseline(project, '1.0.0', '2026-08-01', toolVersion, 'core')).conflicts.join('\n')).toContain('match no release unit');
  });

  it('keeps the flat v2 single topology unit-aware without accepting impact none', async () => {
    const root = await createSingleV2Project();
    let project = await resolveProject({ cwd: root, toolVersion });
    await applyProjectPlan(await planReleaseBaseline(project, '1.0.0', '2026-08-01', toolVersion), toolVersion);
    expect(await readFile(join(root, 'releases/1.0.0.yaml'), 'utf8')).toContain('unit: root');
    await writeSchemaAwareChange(root, '2026-08-20-invalid-none', releaseArtifact('Invalid none impact', 'none', ['cli']));
    project = await resolveProject({ cwd: root, toolVersion });
    expect(() => inspectReleasePlan(project)).toThrow('does not support impact: none');
    await writeSchemaAwareChange(root, '2026-08-20-invalid-none', releaseArtifact('Valid patch impact', 'patch', ['cli']));
    expect(inspectReleasePlan(await resolveProject({ cwd: root, toolVersion }))).toMatchObject({
      unit: 'root', topology: 'single', current: '1.0.0', next: '1.0.1', ready: true,
    });
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

async function createMultiUnitProject(topology: 'independent' | 'composed'): Promise<string> {
  const root = await createFixtureDirectory('arcantry-release-units-');
  await mkdir(join(root, 'openspec/changes/archive'), { recursive: true });
  await mkdir(join(root, 'openspec/schemas/arcantry'), { recursive: true });
  await mkdir(join(root, 'packages/core'), { recursive: true });
  await mkdir(join(root, 'apps/app'), { recursive: true });
  await writeFile(join(root, 'openspec/config.yaml'), 'schema: arcantry\n');
  await writeFile(join(root, 'openspec/schemas/arcantry/schema.yaml'), `name: arcantry
version: 1
artifacts:
  - id: release
    generates: release.md
`);
  await writeFile(join(root, 'packages/core/package.json'), '{\n  "version": "1.0.0"\n}\n');
  await writeFile(join(root, 'apps/app/package.json'), '{\n  "version": "2.0.0"\n}\n');
  await writeFile(join(root, 'arcantry.toml'), `config_version = 1

[sources.intent]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.core_history]
kind = "changelog"
path = "packages/core/CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["intent"]

[sources.app_history]
kind = "changelog"
path = "apps/app/CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["intent"]

[release]
adapter = "openspec-release@2"
topology = "${topology}"

[release.units.core]
manifests_path = "releases/core"
changelog_source = "core_history"
tag_prefix = "core/v"

[[release.units.core.version_sources]]
path = "packages/core/package.json"
adapter = "json-package@1"

[[release.units.core.selectors]]
source = "intent"
components = ["product:core"]

[release.units.app]
manifests_path = "releases/app"
changelog_source = "app_history"
tag_prefix = "app/v"
${topology === 'composed' ? 'dependencies = ["core"]' : ''}

[[release.units.app.version_sources]]
path = "apps/app/package.json"
adapter = "json-package@1"

[[release.units.app.selectors]]
source = "intent"
components = ["product:app"]
`);
  return root;
}

async function writeSchemaAwareChange(root: string, directory: string, release: string): Promise<void> {
  const change = join(root, 'openspec/changes/archive', directory);
  await mkdir(change, { recursive: true });
  await writeFile(join(change, '.openspec.yaml'), 'schema: arcantry\n');
  await writeFile(join(change, 'release.md'), release);
}

function releaseArtifact(title: string, impact: string, components: string[], extra = ''): string {
  return `---
category: changed
impact: ${impact}
${extra}visibility: public
components:
${components.map((component) => `  - ${component}`).join('\n')}
---

# ${title}

Delivered outcome for ${title.toLowerCase()}.
`;
}

async function createSingleV2Project(): Promise<string> {
  const root = await createFixtureDirectory('arcantry-release-v2-single-');
  await mkdir(join(root, 'openspec/changes/archive'), { recursive: true });
  await mkdir(join(root, 'openspec/schemas/arcantry'), { recursive: true });
  await writeFile(join(root, 'openspec/config.yaml'), 'schema: arcantry\n');
  await writeFile(join(root, 'openspec/schemas/arcantry/schema.yaml'), 'name: arcantry\nversion: 1\nartifacts:\n  - id: release\n    generates: release.md\n');
  await writeFile(join(root, 'package.json'), '{\n  "version": "1.0.0"\n}\n');
  await writeFile(join(root, 'arcantry.toml'), `config_version = 1

[sources.intent]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.history]
kind = "changelog"
path = "CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["intent"]

[release]
adapter = "openspec-release@2"
manifests_path = "releases"
changelog_source = "history"

[[release.version_sources]]
path = "package.json"
adapter = "json-package@1"
`);
  return root;
}

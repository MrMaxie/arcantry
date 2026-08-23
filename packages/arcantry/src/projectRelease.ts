import { isAbsolute, relative, resolve } from 'node:path';
import {
  checkChangelog,
  checkRelease,
  isReleaseDate,
  planRelease,
  readManifests,
  readReleaseVersion,
  releaseAdapter,
  releaseAdapterV2,
  readSchemaAwareChanges,
  renderReleaseChangelog,
  renderReleaseManifest,
  updateReleaseVersion,
  validateReleaseState,
  type ReleaseAdapterOptions,
  type ReleaseManifest,
  type ReleaseManifestV2,
  type ReleasePlan,
} from './release.js';
import { createProjectPlan, createWriteOperation, type PlanOperation, type ProjectPlan } from './projectPlan.js';
import {
  isPrivateProjectPath,
  type ProjectReleaseConfigV2Multi,
  type ReleaseTopology,
  type ReleaseUnitConfig,
  type ResolvedProject,
  type Visibility,
} from './projectConfig.js';

export type ResolvedReleaseConfiguration = {
  root: string;
  options: ReleaseAdapterOptions;
  manifestsPath: string;
  changelogPath: string;
  changelogVisibility: Visibility;
  unit?: string;
  topology: ReleaseTopology;
  adapter: typeof releaseAdapter | typeof releaseAdapterV2;
};

export function resolveReleaseConfiguration(project: ResolvedProject, unit?: string): ResolvedReleaseConfiguration {
  const release = project.config?.release;
  if (release === undefined) throw new Error('Project has no [release] configuration.');
  if ('units' in release) {
    if (unit === undefined) throw new Error(`Release topology ${release.topology} requires --unit.`);
    const selected = release.units[unit];
    if (selected === undefined) throw new Error(`Unknown release unit: ${unit}.`);
    validateMultiUnitCoverage(project, release);
    return resolveReleaseUnit(project, release, unit, selected);
  }
  const changelog = project.config?.sources[release.changelogSource];
  if (changelog === undefined || changelog.kind !== 'changelog' || changelog.management !== 'manage') {
    throw new Error(`Release changelog source is not managed: ${release.changelogSource}.`);
  }
  const openSpecPaths = changelog.from.map((id) => {
    const source = project.config?.sources[id];
    if (source === undefined || source.kind !== 'openspec') {
      throw new Error(`Release changelog dependency is not OpenSpec: ${id}.`);
    }
    return source.path;
  });
  const openSpecSources = openSpecPaths.map((path, index) => ({ id: changelog.from[index]!, path }));
  if (release.adapter === releaseAdapterV2) validateSingleV2Coverage(project, openSpecSources);
  return {
    root: project.root,
    manifestsPath: release.manifestsPath,
    changelogPath: changelog.path,
    changelogVisibility: changelog.visibility,
    unit: release.adapter === releaseAdapterV2 ? 'root' : undefined,
    topology: 'single',
    adapter: release.adapter,
    options: {
      adapter: release.adapter,
      topology: 'single',
      ...(release.adapter === releaseAdapterV2 ? { unit: 'root' } : {}),
      openSpecPaths,
      ...(release.adapter === releaseAdapterV2 ? {
        openSpecSources,
        selectors: changelog.from.map((source) => ({ source })),
      } : {}),
      releasesPath: release.manifestsPath,
      changelogPath: changelog.path,
      repositoryUrl: release.repositoryUrl,
      tagPrefix: release.tagPrefix,
      versionSources: release.versionSources,
    },
  };
}

function resolveReleaseUnit(
  project: ResolvedProject,
  release: ProjectReleaseConfigV2Multi,
  unitId: string,
  unit: ReleaseUnitConfig,
): ResolvedReleaseConfiguration {
  const changelog = project.config?.sources[unit.changelogSource];
  if (changelog === undefined || changelog.kind !== 'changelog' || changelog.management !== 'manage') {
    throw new Error(`Release changelog source is not managed: ${unit.changelogSource}.`);
  }
  const openSpecSources = [...new Set(unit.selectors.map((selector) => selector.source))].map((id) => {
    const source = project.config?.sources[id];
    if (source === undefined || source.kind !== 'openspec') throw new Error(`Release selector source is not OpenSpec: ${id}.`);
    return { id, path: source.path };
  });
  return {
    root: project.root,
    manifestsPath: unit.manifestsPath,
    changelogPath: changelog.path,
    changelogVisibility: changelog.visibility,
    unit: unitId,
    topology: release.topology,
    adapter: releaseAdapterV2,
    options: {
      adapter: releaseAdapterV2,
      topology: release.topology,
      unit: unitId,
      openSpecSources,
      selectors: unit.selectors,
      releasesPath: unit.manifestsPath,
      changelogPath: changelog.path,
      repositoryUrl: release.repositoryUrl,
      tagPrefix: unit.tagPrefix,
      versionSources: unit.versionSources,
      dependencies: Object.fromEntries(unit.dependencies.map((dependency) => {
        const dependencyUnit = release.units[dependency]!;
        return [dependency, { releasesPath: dependencyUnit.manifestsPath, versionSources: dependencyUnit.versionSources }];
      })),
    },
  };
}

export async function planReleaseBaseline(
  project: ResolvedProject,
  version: string,
  date: string,
  toolVersion: string,
  unit?: string,
): Promise<ProjectPlan> {
  return createReleasePlan(project, toolVersion, unit, async (configuration) => {
    validateStableVersion(version);
    validateReleaseDate(date);
    if (readManifests(configuration.root, configuration.manifestsPath, configuration.unit).length > 0) {
      throw new Error('Release baseline requires a project without release manifests.');
    }
    validateVersionSources(configuration, version);
    const state = validateReleaseState(configuration.root, configuration.options);
    const manifest: ReleaseManifest | ReleaseManifestV2 = configuration.adapter === releaseAdapterV2
      ? {
          format: 2,
          unit: configuration.unit!,
          version,
          date,
          changes: [],
          baseline: true,
          ...dependencyPins(configuration),
        } satisfies ReleaseManifestV2
      : { version, date, changes: [], baseline: true };
    return [
      await createWriteOperation(
        configuration.root,
        `${configuration.manifestsPath.replace(/[\\/]$/, '')}/${version}.yaml`,
        renderReleaseManifest(manifest),
        pathVisibility(configuration.root, configuration.manifestsPath),
      ),
      await createWriteOperation(
        configuration.root,
        configuration.changelogPath,
        renderReleaseChangelog({ archived: state.archived, manifests: [manifest] }, configuration.options),
        configuration.changelogVisibility,
      ),
    ];
  });
}

export async function planReleaseCut(
  project: ResolvedProject,
  date: string,
  toolVersion: string,
  unit?: string,
): Promise<ProjectPlan> {
  return createReleasePlan(project, toolVersion, unit, async (configuration) => {
    validateReleaseDate(date);
    const state = validateReleaseState(configuration.root, configuration.options);
    const release = planRelease(configuration.root, configuration.options);
    if (release.changes.length === 0) throw new Error('No unassigned archived changes to release.');
    if (release.ready === false) throw new Error(`Release unit ${configuration.unit} has unacknowledged pending dependencies: ${Object.keys(release.pendingDependencies ?? {}).join(', ')}.`);
    validateVersionSources(configuration, release.current);
    const manifest: ReleaseManifest | ReleaseManifestV2 = configuration.adapter === releaseAdapterV2
      ? {
          format: 2,
          unit: configuration.unit!,
          version: release.next,
          date,
          changes: release.changes,
          ...dependencyPins(configuration),
        } satisfies ReleaseManifestV2
      : { version: release.next, date, changes: release.changes };
    const operations: PlanOperation[] = [
      await createWriteOperation(
        configuration.root,
        `${configuration.manifestsPath.replace(/[\\/]$/, '')}/${manifest.version}.yaml`,
        renderReleaseManifest(manifest),
        pathVisibility(configuration.root, configuration.manifestsPath),
      ),
    ];
    for (const source of configuration.options.versionSources ?? []) {
      operations.push(await createWriteOperation(
        configuration.root,
        source.path,
        updateReleaseVersion(configuration.root, source, manifest.version),
        pathVisibility(configuration.root, source.path),
      ));
    }
    operations.push(await createWriteOperation(
      configuration.root,
      configuration.changelogPath,
      renderReleaseChangelog(
        { archived: state.archived, manifests: [...state.manifests, manifest] },
        configuration.options,
      ),
      configuration.changelogVisibility,
    ));
    return operations;
  });
}

export async function planReleaseRender(project: ResolvedProject, toolVersion: string, unit?: string): Promise<ProjectPlan> {
  return createReleasePlan(project, toolVersion, unit, async (configuration) => [
    await createWriteOperation(
      configuration.root,
      configuration.changelogPath,
      renderReleaseChangelog(validateReleaseState(configuration.root, configuration.options), configuration.options),
      configuration.changelogVisibility,
    ),
  ]);
}

export function inspectReleasePlan(project: ResolvedProject, unit?: string): ReleasePlan {
  const configuration = resolveReleaseConfiguration(project, unit);
  return planRelease(configuration.root, configuration.options);
}

export function checkProjectRelease(project: ResolvedProject, sealed: boolean, unit?: string): void {
  const release = project.config?.release;
  if (release === undefined) throw new Error('Project has no [release] configuration.');
  if (!('units' in release)) {
    const configuration = resolveReleaseConfiguration(project);
    if (sealed) checkRelease(configuration.root, configuration.options);
    else checkChangelog(configuration.root, configuration.options);
    return;
  }
  if (sealed && unit === undefined) throw new Error(`Release topology ${release.topology} requires --unit for sealed checking.`);
  validateMultiUnitCoverage(project, release);
  const units = unit === undefined ? Object.keys(release.units).sort() : [unit];
  for (const unitId of units) {
    const configuration = resolveReleaseConfiguration(project, unitId);
    if (sealed) checkRelease(configuration.root, configuration.options);
    else checkChangelog(configuration.root, configuration.options);
  }
}

async function createReleasePlan(
  project: ResolvedProject,
  toolVersion: string,
  unit: string | undefined,
  build: (configuration: ResolvedReleaseConfiguration) => Promise<PlanOperation[]>,
): Promise<ProjectPlan> {
  try {
    const configuration = resolveReleaseConfiguration(project, unit);
    return createProjectPlan({
      toolVersion,
      root: configuration.root,
      sourceId: 'release',
      transition: 'adopt',
      adapter: configuration.adapter,
      operations: (await build(configuration)).filter(
        (operation) => operation.action !== 'write' || operation.expectedHash !== operation.contentHash,
      ),
    });
  } catch (error) {
    return createProjectPlan({
      toolVersion,
      root: project.root,
      sourceId: 'release',
      transition: 'adopt',
      adapter: project.config?.release?.adapter ?? releaseAdapter,
      conflicts: [error instanceof Error ? error.message : String(error)],
    });
  }
}

function dependencyPins(configuration: ResolvedReleaseConfiguration): { dependencies?: Record<string, string> } {
  const dependencies = Object.fromEntries(Object.entries(configuration.options.dependencies ?? {}).map(([id, dependency]) => {
    const latest = readManifests(configuration.root, dependency.releasesPath, id).at(-1);
    if (latest === undefined) throw new Error(`Release unit dependency has no manifest: ${id}.`);
    for (const source of dependency.versionSources) {
      const actual = readReleaseVersion(configuration.root, source);
      if (actual !== latest.version) throw new Error(`Dependency version source must match ${latest.version}: ${source.path} contains ${actual}.`);
    }
    return [id, latest.version];
  }));
  return Object.keys(dependencies).length === 0 ? {} : { dependencies };
}

function validateMultiUnitCoverage(project: ResolvedProject, release: ProjectReleaseConfigV2Multi): void {
  const sourceIds = [...new Set(Object.values(release.units).flatMap((unit) => unit.selectors.map((selector) => selector.source)))];
  const sources = sourceIds.map((id) => ({ id, path: project.config!.sources[id]!.path }));
  const archived = readSchemaAwareChanges(project.root, sources, 'archive');
  const active = readSchemaAwareChanges(project.root, sources, 'active');
  const unmatched = [...archived].filter(([, artifact]) => !Object.values(release.units).some((unit) =>
    unit.selectors.some((selector) => selector.source === artifact.sourceId
      && (selector.components === undefined || selector.components.some((component) => artifact.components.includes(component)))))).map(([id]) => id);
  if (unmatched.length > 0) throw new Error(`Archived release-bearing OpenSpec changes match no release unit: ${unmatched.sort().join(', ')}.`);
  for (const [changeId, artifact] of [...archived, ...active]) {
    for (const unit of Object.keys(artifact.unitImpacts ?? {})) {
      if (!(unit in release.units)) throw new Error(`OpenSpec change ${changeId} declares unit_impacts for unknown unit: ${unit}.`);
      if (!release.units[unit]!.selectors.some((selector) => selector.source === artifact.sourceId && (selector.components === undefined || selector.components.some((component) => artifact.components.includes(component))))) {
        throw new Error(`OpenSpec change ${changeId} declares unit_impacts for unmatched unit: ${unit}.`);
      }
    }
    for (const [unit, dependencies] of Object.entries(artifact.dependencyUpdates ?? {})) {
      const configured = release.units[unit];
      if (configured === undefined) throw new Error(`OpenSpec change ${changeId} declares dependency_updates for unknown unit: ${unit}.`);
      if (!configured.selectors.some((selector) => selector.source === artifact.sourceId && (selector.components === undefined || selector.components.some((component) => artifact.components.includes(component))))) {
        throw new Error(`OpenSpec change ${changeId} declares dependency_updates for unmatched unit: ${unit}.`);
      }
      for (const dependency of dependencies) if (!configured.dependencies.includes(dependency)) throw new Error(`OpenSpec change ${changeId} acknowledges non-direct dependency ${unit}: ${dependency}.`);
    }
  }
}

function validateSingleV2Coverage(project: ResolvedProject, sources: Array<{ id: string; path: string }>): void {
  const archived = readSchemaAwareChanges(project.root, sources, 'archive');
  const active = readSchemaAwareChanges(project.root, sources, 'active');
  for (const [changeId, artifact] of [...archived, ...active]) {
    for (const unit of Object.keys(artifact.unitImpacts ?? {})) if (unit !== 'root') throw new Error(`OpenSpec change ${changeId} declares unit_impacts for unknown unit: ${unit}.`);
    if (Object.keys(artifact.dependencyUpdates ?? {}).length > 0) throw new Error(`OpenSpec change ${changeId} declares dependency_updates in a single release topology.`);
  }
}

function validateVersionSources(configuration: ResolvedReleaseConfiguration, expected: string): void {
  for (const source of configuration.options.versionSources ?? []) {
    const actual = readReleaseVersion(configuration.root, source);
    if (actual !== expected) throw new Error(`Version source must match ${expected}: ${source.path} contains ${actual}.`);
  }
}

function validateStableVersion(version: string): void {
  if (!/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/.test(version)) {
    throw new Error(`Release version must be full stable SemVer: ${version}.`);
  }
}

function validateReleaseDate(date: string): void {
  if (!isReleaseDate(date)) throw new Error(`Invalid release date: ${date}.`);
}

function pathVisibility(root: string, path: string): Visibility {
  if (isAbsolute(path)) {
    const projectPath = relative(resolve(root), resolve(path));
    if (projectPath.startsWith('..') || isAbsolute(projectPath)) return 'private';
  }
  return isPrivateProjectPath(path) ? 'private' : 'shared';
}

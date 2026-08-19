import { isAbsolute, relative, resolve } from 'node:path';
import {
  checkChangelog,
  checkRelease,
  isReleaseDate,
  planRelease,
  readManifests,
  readReleaseVersion,
  releaseAdapter,
  renderReleaseChangelog,
  renderReleaseManifest,
  updateReleaseVersion,
  validateReleaseState,
  type ReleaseAdapterOptions,
  type ReleaseManifest,
} from './release.js';
import { createProjectPlan, createWriteOperation, type PlanOperation, type ProjectPlan } from './projectPlan.js';
import type { ResolvedProject, Visibility } from './projectConfig.js';

export type ResolvedReleaseConfiguration = {
  root: string;
  options: ReleaseAdapterOptions;
  manifestsPath: string;
  changelogPath: string;
  changelogVisibility: Visibility;
};

export function resolveReleaseConfiguration(project: ResolvedProject): ResolvedReleaseConfiguration {
  const release = project.config?.release;
  if (release === undefined) throw new Error('Project has no [release] configuration.');
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
  return {
    root: project.root,
    manifestsPath: release.manifestsPath,
    changelogPath: changelog.path,
    changelogVisibility: changelog.visibility,
    options: {
      openSpecPaths,
      releasesPath: release.manifestsPath,
      changelogPath: changelog.path,
      repositoryUrl: release.repositoryUrl,
      tagPrefix: release.tagPrefix,
      versionSources: release.versionSources,
    },
  };
}

export async function planReleaseBaseline(
  project: ResolvedProject,
  version: string,
  date: string,
  toolVersion: string,
): Promise<ProjectPlan> {
  return createReleasePlan(project, toolVersion, async (configuration) => {
    validateStableVersion(version);
    validateReleaseDate(date);
    if (readManifests(configuration.root, configuration.manifestsPath).length > 0) {
      throw new Error('Release baseline requires a project without release manifests.');
    }
    validateVersionSources(configuration, version);
    const state = validateReleaseState(configuration.root, configuration.options);
    const manifest: ReleaseManifest = { version, date, changes: [], baseline: true };
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
): Promise<ProjectPlan> {
  return createReleasePlan(project, toolVersion, async (configuration) => {
    validateReleaseDate(date);
    const state = validateReleaseState(configuration.root, configuration.options);
    const release = planRelease(configuration.root, configuration.options);
    if (release.changes.length === 0) throw new Error('No unassigned archived changes to release.');
    validateVersionSources(configuration, release.current);
    const manifest: ReleaseManifest = { version: release.next, date, changes: release.changes };
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

export async function planReleaseRender(project: ResolvedProject, toolVersion: string): Promise<ProjectPlan> {
  return createReleasePlan(project, toolVersion, async (configuration) => [
    await createWriteOperation(
      configuration.root,
      configuration.changelogPath,
      renderReleaseChangelog(validateReleaseState(configuration.root, configuration.options), configuration.options),
      configuration.changelogVisibility,
    ),
  ]);
}

export function inspectReleasePlan(project: ResolvedProject): ReturnType<typeof planRelease> {
  const configuration = resolveReleaseConfiguration(project);
  return planRelease(configuration.root, configuration.options);
}

export function checkProjectRelease(project: ResolvedProject, sealed: boolean): void {
  const configuration = resolveReleaseConfiguration(project);
  if (sealed) checkRelease(configuration.root, configuration.options);
  else checkChangelog(configuration.root, configuration.options);
}

async function createReleasePlan(
  project: ResolvedProject,
  toolVersion: string,
  build: (configuration: ResolvedReleaseConfiguration) => Promise<PlanOperation[]>,
): Promise<ProjectPlan> {
  try {
    const configuration = resolveReleaseConfiguration(project);
    return createProjectPlan({
      toolVersion,
      root: configuration.root,
      sourceId: 'release',
      transition: 'adopt',
      adapter: releaseAdapter,
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
      adapter: releaseAdapter,
      conflicts: [error instanceof Error ? error.message : String(error)],
    });
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
  const normalized = path.replaceAll('\\', '/').replace(/^\.\//, '');
  return normalized === '.local' || normalized.startsWith('.local/') ? 'private' : 'shared';
}

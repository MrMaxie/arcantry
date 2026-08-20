import { type ReleaseArtifact, type ReleaseManifest, readArchivedChanges, readManifests } from './release.js';

export interface VersionModel {
  currentRelease: string;
  skillReleases: Record<string, string>;
}

export function buildVersionModel(root = process.cwd()): VersionModel {
  return createVersionModel(readManifests(root), readArchivedChanges(root));
}

export function createVersionModel(
  manifests: ReleaseManifest[],
  archivedChanges: Map<string, ReleaseArtifact>,
): VersionModel {
  const skillReleases: Record<string, string> = {};

  for (const manifest of manifests) {
    for (const changeId of manifest.changes) {
      const artifact = archivedChanges.get(changeId);
      if (!artifact) continue;
      for (const component of artifact.components) {
        if (!component.startsWith('skill:')) continue;
        const skill = component.slice('skill:'.length);
        skillReleases[skill] ??= manifest.version;
      }
    }
  }

  return {
    currentRelease: manifests.at(-1)?.version ?? '0.0.0',
    skillReleases,
  };
}

import { describe, expect, it } from 'vitest';
import type { ReleaseArtifact, ReleaseManifest } from './release.js';
import { createVersionModel } from './version.js';

const artifact = (components: string[]): ReleaseArtifact => ({
  category: 'added',
  impact: 'minor',
  visibility: 'public',
  components,
  title: 'Test release',
  body: 'A release used by the version model test.',
});

describe('documentation version model', () => {
  it('uses the latest product release and the first release containing a skill', () => {
    const manifests: ReleaseManifest[] = [
      { version: '0.2.0', date: '2026-08-01', changes: ['first'] },
      { version: '0.3.0', date: '2026-08-17', changes: ['second'] },
    ];
    const archived = new Map([
      ['first', artifact(['skill:adopt-arcantry'])],
      ['second', artifact(['skill:adopt-arcantry', 'skill:select-task-skills'])],
    ]);

    expect(createVersionModel(manifests, archived)).toEqual({
      currentRelease: '0.3.0',
      skillReleases: {
        'adopt-arcantry': '0.2.0',
        'select-task-skills': '0.3.0',
      },
    });
  });

  it('does not assign a release to an unlisted skill', () => {
    const model = createVersionModel(
      [{ version: '0.2.3', date: '2026-08-16', changes: [] }],
      new Map(),
    );

    expect(model.currentRelease).toBe('0.2.3');
    expect(model.skillReleases['unreleased-skill']).toBeUndefined();
  });
});


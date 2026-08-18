import { describe, expect, it } from 'vitest';
import { cutoverChangelog, inspectChangelog, migrateChangelogToV2, renderKeepAChangelogV2 } from './changelog.js';

describe('Keep a Changelog adapter', () => {
  it('renders deterministic 2.0 sections and optional comparison links', () => {
    const changelog = renderKeepAChangelogV2({
      repositoryUrl: 'https://github.com/example/project/',
      unreleased: { Changed: ['Improve planning'] },
      releases: [
        { version: '1.0.0', date: '2026-08-01', categories: { Added: ['Initial release'] } },
        { version: '1.1.0', date: '2026-08-18', categories: { Fixed: ['Preserve history'] } },
      ],
    });

    expect(changelog).toContain('https://keepachangelog.com/en/2.0.0/');
    expect(changelog.indexOf('## [1.1.0]')).toBeLessThan(changelog.indexOf('## [1.0.0]'));
    expect(changelog).toContain('[Unreleased]: https://github.com/example/project/compare/v1.1.0...HEAD');
    expect(inspectChangelog(changelog)).toMatchObject({ format: 'keep-a-changelog-2', releases: ['1.1.0', '1.0.0'] });
  });

  it('cuts over while preserving the complete legacy release tail byte-for-byte', () => {
    const legacyTail = '## [0.9.0] - 2026-07-01\r\n\r\n### Added\r\n\r\n- Legacy behavior\r\n';
    const legacy = `# Changelog\r\n\r\nBased on https://keepachangelog.com/en/1.1.0/\r\n\r\n## [Unreleased]\r\n\r\n${legacyTail}`;

    const cutover = cutoverChangelog(legacy, '1.0.0');

    expect(cutover.endsWith(legacyTail)).toBe(true);
    expect(cutover).toContain('## [Unreleased]');
    expect(cutover).toContain('Arcantry manages releases from 1.0.0');
  });

  it('blocks cutover when Unreleased contains meaning not represented in OpenSpec', () => {
    const changelog = '# Changelog\n\n## [Unreleased]\n\n### Added\n\n- Unknown work\n\n## [0.9.0] - 2026-07-01\n';
    expect(() => cutoverChangelog(changelog, '1.0.0')).toThrow('non-empty Unreleased');
  });

  it('migrates an identified 1.x preamble without rewriting release prose', () => {
    const changelog = '# Changelog\n\nBased on https://keepachangelog.com/en/1.1.0/\n\n## [Unreleased]\n\n## [1.0.0] - 2026-08-18\n\n### Added\n\n- Exact prose\n';
    const migrated = migrateChangelogToV2(changelog);

    expect(migrated).toContain('https://keepachangelog.com/en/2.0.0/');
    expect(migrated).toContain('- Exact prose\n');
  });
});

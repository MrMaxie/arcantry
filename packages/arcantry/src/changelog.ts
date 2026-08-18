import { rcompare, valid } from 'semver';

export const changelogCategories = ['Added', 'Changed', 'Deprecated', 'Removed', 'Fixed', 'Security'] as const;
export type ChangelogCategory = (typeof changelogCategories)[number];

export type ChangelogRelease = {
  version: string;
  date: string;
  summary?: string;
  categories: Partial<Record<ChangelogCategory, string[]>>;
};

export type ChangelogInspection = {
  format: 'keep-a-changelog-2' | 'keep-a-changelog-1' | 'compatible' | 'unknown';
  releases: string[];
  hasUnreleased: boolean;
  diagnostics: string[];
};

const versionHeading = /^## \[(\d+\.\d+\.\d+)\] - (\d{4}-\d{2}-\d{2})\s*$/gm;
const anyReleaseHeading = /^## \[\d+\.\d+\.\d+\] - \d{4}-\d{2}-\d{2}\s*$/m;
const unreleasedHeading = /^## \[Unreleased\]\s*$/m;

export const keepAChangelogPreamble = [
  '# Changelog',
  '',
  'All notable changes to this project will be documented in this file.',
  '',
  'The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),',
  'and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).',
  '',
].join('\n');

export const inspectChangelog = (content: string): ChangelogInspection => {
  const plain = content.replace(/^\uFEFF/, '');
  const releases = [...plain.matchAll(versionHeading)].map((match) => match[1]!);
  const diagnostics: string[] = [];
  if (!plain.startsWith('# Changelog')) diagnostics.push('Changelog must start with # Changelog.');
  if (!unreleasedHeading.test(plain)) diagnostics.push('Changelog has no [Unreleased] section.');

  for (const heading of plain.matchAll(/^### (.+)$/gm)) {
    if (!changelogCategories.includes(heading[1] as ChangelogCategory)) {
      diagnostics.push(`Unknown Keep a Changelog category: ${heading[1]}.`);
    }
  }

  const format = plain.includes('keepachangelog.com/en/2.0.0')
    ? 'keep-a-changelog-2'
    : /keepachangelog\.com\/en\/1\./.test(plain)
      ? 'keep-a-changelog-1'
      : releases.length > 0 || unreleasedHeading.test(plain)
        ? 'compatible'
        : 'unknown';
  return { format, releases, hasUnreleased: unreleasedHeading.test(plain), diagnostics };
};

export const renderKeepAChangelogV2 = (input: {
  releases: ChangelogRelease[];
  unreleased?: Partial<Record<ChangelogCategory, string[]>>;
  repositoryUrl?: string;
  tagPrefix?: string;
}): string => {
  const tagPrefix = input.tagPrefix ?? 'v';
  const releases = [...input.releases].sort((left, right) => rcompare(left.version, right.version));
  for (const release of releases) {
    if (valid(release.version) === null) throw new Error(`Invalid changelog version: ${release.version}`);
    if (!/^\d{4}-\d{2}-\d{2}$/.test(release.date)) throw new Error(`Invalid changelog date: ${release.date}`);
  }

  const sections = [keepAChangelogPreamble.trimEnd(), '## [Unreleased]', renderCategories(input.unreleased ?? {})];
  for (const release of releases) {
    sections.push(
      `## [${release.version}] - ${release.date}`,
      release.summary?.trim() ?? '',
      renderCategories(release.categories),
    );
  }

  const body = `${sections.filter((section) => section.length > 0).join('\n\n')}\n`;
  if (input.repositoryUrl === undefined || releases.length === 0) return body;
  const repositoryUrl = input.repositoryUrl.replace(/\/$/, '');
  const links = [`[Unreleased]: ${repositoryUrl}/compare/${tagPrefix}${releases[0]!.version}...HEAD`];
  for (const [index, release] of releases.entries()) {
    const previous = releases[index + 1];
    links.push(
      previous === undefined
        ? `[${release.version}]: ${repositoryUrl}/releases/tag/${tagPrefix}${release.version}`
        : `[${release.version}]: ${repositoryUrl}/compare/${tagPrefix}${previous.version}...${tagPrefix}${release.version}`,
    );
  }
  return `${body}\n${links.join('\n')}\n`;
};

export const cutoverChangelog = (content: string, managedFrom: string): string => {
  if (valid(managedFrom) === null) throw new Error('managed_from must be a full SemVer version for changelog cutover.');
  if (content.includes(`Arcantry manages releases from ${managedFrom}.`)) return content;
  const plain = content.replace(/^\uFEFF/, '');
  const firstRelease = plain.search(anyReleaseHeading);
  if (firstRelease < 0) throw new Error('Changelog cutover requires at least one dated version section.');

  const unreleased = plain.match(unreleasedHeading);
  if (unreleased !== null && unreleased.index !== undefined && unreleased.index < firstRelease) {
    const bodyStart = unreleased.index + unreleased[0].length;
    const unreleasedBody = plain.slice(bodyStart, firstRelease).replace(/^###\s+.*$/gm, '').trim();
    if (unreleasedBody.length > 0) {
      throw new Error('Changelog cutover cannot preserve a non-empty Unreleased section without explicit OpenSpec meaning.');
    }
  }

  const bom = content.startsWith('\uFEFF') ? '\uFEFF' : '';
  const legacy = plain.slice(firstRelease);
  const marker = `<!-- Arcantry manages releases from ${managedFrom}. Earlier history remains project-owned. -->`;
  return `${bom}${keepAChangelogPreamble}\n## [Unreleased]\n\n${marker}\n\n${legacy}`;
};

export const migrateChangelogToV2 = (content: string): string => {
  const inspection = inspectChangelog(content);
  if (inspection.format === 'keep-a-changelog-2') return content;
  if (inspection.format !== 'keep-a-changelog-1') {
    throw new Error('Full changelog migration requires an identified Keep a Changelog 1.x source.');
  }
  return content.replace(/https:\/\/keepachangelog\.com\/en\/1(?:\.\d+){1,2}\/?/g, 'https://keepachangelog.com/en/2.0.0/');
};

const renderCategories = (categories: Partial<Record<ChangelogCategory, string[]>>): string =>
  changelogCategories
    .flatMap((category) => {
      const entries = categories[category]?.map((entry) => entry.trim()).filter(Boolean) ?? [];
      return entries.length === 0 ? [] : [`### ${category}`, '', ...entries.map((entry) => `- ${entry}`), ''];
    })
    .join('\n')
    .trimEnd();

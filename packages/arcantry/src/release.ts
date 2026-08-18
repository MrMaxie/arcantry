import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { basename, join, resolve } from 'node:path';
import { parse as parseYaml, stringify as stringifyYaml } from 'yaml';
import { changelogCategories, keepAChangelogPreamble, type ChangelogCategory } from './changelog.js';

export const releaseAdapter = 'openspec-release@1' as const;
export type Impact = 'none' | 'patch' | 'minor' | 'major';
export type Category = 'added' | 'changed' | 'fixed' | 'deprecated' | 'removed' | 'security';
export type ReleaseVisibility = 'public' | 'internal';

export interface ReleaseArtifact {
  category: Category;
  impact: Impact;
  visibility: ReleaseVisibility;
  components: string[];
  title: string;
  body: string;
}

export interface ReleaseManifest {
  version: string;
  date: string;
  changes: string[];
}

export interface ReleaseState {
  archived: Map<string, ReleaseArtifact>;
  manifests: ReleaseManifest[];
  assigned: Set<string>;
}

export interface ReleaseAdapterOptions {
  openSpecPaths?: string[];
  releasesPath?: string;
  changelogPath?: string;
  distributionManifestPaths?: string[];
}

export interface ReleaseSealValidationOptions {
  pullRequestHead?: string;
}

export interface NpmPublicationIdentity {
  packageName: string;
  version: string;
  tag: string;
  repositoryUrl: string;
}

const impactRank: Record<Impact, number> = { none: 0, patch: 1, minor: 2, major: 3 };
const categories: Category[] = ['added', 'changed', 'fixed', 'deprecated', 'removed', 'security'];
const categoryHeadings: Record<Category, ChangelogCategory> = {
  added: 'Added',
  changed: 'Changed',
  deprecated: 'Deprecated',
  removed: 'Removed',
  fixed: 'Fixed',
  security: 'Security',
};
const impacts: Impact[] = ['none', 'patch', 'minor', 'major'];
const visibilities: ReleaseVisibility[] = ['public', 'internal'];
const componentPattern = /^[a-z0-9]+(?:-[a-z0-9]+)*(?::[a-z0-9]+(?:-[a-z0-9]+)*)?$/;
const semVerPattern = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/;
const datePattern = /^\d{4}-\d{2}-\d{2}$/;

export function parseReleaseArtifact(source: string): ReleaseArtifact {
  const match = source.match(/^---\r?\n([\s\S]*?)\r?\n---\r?\n([\s\S]*)$/);
  if (!match) throw new Error('release.md must start with YAML frontmatter');

  const metadata = parseYaml(match[1]) as Record<string, unknown>;
  const category = metadata.category as Category;
  const impact = metadata.impact as Impact;
  const visibility = metadata.visibility as ReleaseVisibility;
  const components = metadata.components;

  if (!categories.includes(category)) throw new Error(`invalid release category: ${String(category)}`);
  if (!impacts.includes(impact)) throw new Error(`invalid release impact: ${String(impact)}`);
  if (!visibilities.includes(visibility)) throw new Error(`invalid release visibility: ${String(visibility)}`);
  if (!Array.isArray(components) || components.length === 0 || !components.every((component) => typeof component === 'string' && componentPattern.test(component))) {
    throw new Error('release components must be a non-empty array of component identifiers');
  }
  if (new Set(components).size !== components.length) throw new Error('release components must be unique');

  const content = match[2].trim();
  const firstLineEnd = content.indexOf('\n');
  const firstLine = firstLineEnd === -1 ? content : content.slice(0, firstLineEnd);
  let titleStart = 1;
  while (firstLine[titleStart] === ' ' || firstLine[titleStart] === '\t') titleStart += 1;
  const title = firstLine.startsWith('#') && titleStart > 1 ? firstLine.slice(titleStart).trim() : '';
  if (!title) throw new Error('release.md must contain a level-one title');
  const body = firstLineEnd === -1 ? '' : content.slice(firstLineEnd + 1).trim();
  if (!body) throw new Error('release.md must describe the delivered outcome');
  return { category, impact, visibility, components, title, body };
}

export function highestImpact(values: Impact[]): Impact {
  return values.reduce<Impact>((highest, current) => impactRank[current] > impactRank[highest] ? current : highest, 'none');
}

export function bumpVersion(version: string, impact: Impact): string {
  const match = version.match(semVerPattern);
  if (!match) throw new Error(`invalid SemVer version: ${version}`);
  const major = Number(match[1]);
  const minor = Number(match[2]);
  const patch = Number(match[3]);
  if (impact === 'major') return `${major + 1}.0.0`;
  if (impact === 'minor') return `${major}.${minor + 1}.0`;
  if (impact === 'patch') return `${major}.${minor}.${patch + 1}`;
  return version;
}

function changeIdFromArchiveDirectory(directory: string): string {
  const match = directory.match(/^\d{4}-\d{2}-\d{2}-(.+)$/);
  if (!match) throw new Error(`invalid OpenSpec archive directory: ${directory}`);
  return match[1];
}

export function readArchivedChanges(root = process.cwd(), openSpecPaths = ['openspec']): Map<string, ReleaseArtifact> {
  const result = new Map<string, ReleaseArtifact>();
  for (const openSpecPath of openSpecPaths) {
    const archive = join(resolve(root, openSpecPath), 'changes', 'archive');
    if (!existsSync(archive)) continue;
    for (const directory of readdirSync(archive, { withFileTypes: true })) {
      if (!directory.isDirectory()) continue;
      const changeId = changeIdFromArchiveDirectory(directory.name);
      if (result.has(changeId)) throw new Error(`duplicate archived change id: ${changeId}`);
      const path = join(archive, directory.name, 'release.md');
      if (!existsSync(path)) throw new Error(`archived change ${changeId} has no release.md`);
      result.set(changeId, parseReleaseArtifact(readFileSync(path, 'utf8')));
    }
  }
  return result;
}

export function readManifests(root = process.cwd(), releasesPath = 'releases'): ReleaseManifest[] {
  const directory = resolve(root, releasesPath);
  if (!existsSync(directory)) return [];
  return readdirSync(directory).filter((file) => file.endsWith('.yaml')).map((file) => {
    const data = parseYaml(readFileSync(join(directory, file), 'utf8')) as Partial<ReleaseManifest>;
    if (!data || typeof data.version !== 'string' || !semVerPattern.test(data.version)) throw new Error(`invalid release version: ${file}`);
    if (basename(file, '.yaml') !== data.version) throw new Error(`release manifest filename must match version: ${file}`);
    if (typeof data.date !== 'string' || !datePattern.test(data.date)) throw new Error(`invalid release date: ${file}`);
    if (!Array.isArray(data.changes) || !data.changes.every((change) => typeof change === 'string' && change.length > 0)) throw new Error(`invalid release changes: ${file}`);
    if (new Set(data.changes).size !== data.changes.length) throw new Error(`duplicate change in release manifest: ${file}`);
    return data as ReleaseManifest;
  }).sort((left, right) => compareSemVer(left.version, right.version));
}

function compareSemVer(left: string, right: string): number {
  if (!semVerPattern.test(left)) throw new Error(`invalid SemVer version: ${left}`);
  if (!semVerPattern.test(right)) throw new Error(`invalid SemVer version: ${right}`);
  const a = left.split('.').map(Number);
  const b = right.split('.').map(Number);
  for (let index = 0; index < 3; index += 1) if (a[index] !== b[index]) return a[index]! - b[index]!;
  return 0;
}

export function validateReleaseState(root = process.cwd(), options: ReleaseAdapterOptions = {}): ReleaseState {
  const archived = readArchivedChanges(root, options.openSpecPaths);
  const manifests = readManifests(root, options.releasesPath);
  const assigned = new Set<string>();
  for (const manifest of manifests) for (const changeId of manifest.changes) {
    if (!archived.has(changeId)) throw new Error(`release ${manifest.version} references unknown archived change: ${changeId}`);
    if (assigned.has(changeId)) throw new Error(`archived change assigned more than once: ${changeId}`);
    assigned.add(changeId);
  }
  return { archived, manifests, assigned };
}

export function planRelease(root = process.cwd(), options: ReleaseAdapterOptions = {}): { current: string; next: string; impact: Impact; changes: string[] } {
  const { archived, manifests, assigned } = validateReleaseState(root, options);
  const changes = [...archived.keys()].filter((id) => !assigned.has(id)).sort();
  const unversioned = changes.filter((id) => archived.get(id)!.impact === 'none');
  if (unversioned.length > 0) throw new Error(`completed changes must declare a SemVer impact: ${unversioned.join(', ')}`);
  const impact = highestImpact(changes.map((id) => archived.get(id)!.impact));
  const current = manifests.at(-1)?.version ?? '0.0.0';
  return { current, next: bumpVersion(current, impact), impact, changes };
}

export function cutRelease(
  root = process.cwd(),
  date = new Date().toISOString().slice(0, 10),
  options: ReleaseAdapterOptions = {},
): ReleaseManifest {
  if (!datePattern.test(date)) throw new Error(`invalid release date: ${date}`);
  const plan = planRelease(root, options);
  if (plan.changes.length === 0) throw new Error('no unassigned archived changes to release');
  if (plan.impact === 'none') throw new Error('unassigned changes do not require a SemVer release');
  const manifest: ReleaseManifest = { version: plan.next, date, changes: plan.changes };
  const directory = resolve(root, options.releasesPath ?? 'releases');
  mkdirSync(directory, { recursive: true });
  const path = join(directory, `${manifest.version}.yaml`);
  if (existsSync(path)) throw new Error(`release manifest already exists: ${manifest.version}`);
  writeFileSync(path, stringifyYaml(manifest, { lineWidth: 0 }), 'utf8');
  return manifest;
}

export function renderChangelog(root = process.cwd(), options: ReleaseAdapterOptions = {}): string {
  const { archived, manifests } = validateReleaseState(root, options);
  const ordered = [...manifests].sort((left, right) => compareSemVer(right.version, left.version));
  const lines = [keepAChangelogPreamble.trimEnd(), '', '## [Unreleased]', ''];
  for (const manifest of ordered) {
    lines.push(`## [${manifest.version}] - ${manifest.date}`, '');
    const grouped = new Map<Category, Array<{ id: string; artifact: ReleaseArtifact }>>();
    for (const changeId of manifest.changes) {
      const artifact = archived.get(changeId)!;
      if (artifact.visibility === 'internal') continue;
      grouped.set(artifact.category, [...(grouped.get(artifact.category) ?? []), { id: changeId, artifact }]);
    }
    for (const category of categories) {
      const entries = grouped.get(category);
      if (!entries?.length) continue;
      lines.push(`### ${categoryHeadings[category]}`, '');
      for (const { id, artifact } of entries) lines.push(`<!-- openspec: ${id} -->`, `#### ${artifact.title}`, '', artifact.body, '');
    }
  }
  const invalidHeadings = Object.values(categoryHeadings).filter((heading) => !changelogCategories.includes(heading));
  if (invalidHeadings.length > 0) throw new Error(`invalid changelog categories: ${invalidHeadings.join(', ')}`);
  return `${lines.join('\n').trimEnd()}\n`;
}

export function checkChangelog(root = process.cwd(), options: ReleaseAdapterOptions = {}): void {
  const path = resolve(root, options.changelogPath ?? 'CHANGELOG.md');
  if (!existsSync(path)) throw new Error('CHANGELOG.md is missing');
  validateDistributionVersions(root, options.distributionManifestPaths, options.releasesPath);
  if (readFileSync(path, 'utf8') !== renderChangelog(root, options)) throw new Error('CHANGELOG.md is stale; run `just release-render`');
}

export function readActiveChangeIds(root = process.cwd(), openSpecPaths = ['openspec']): string[] {
  return openSpecPaths.flatMap((openSpecPath) => {
    const directory = join(resolve(root, openSpecPath), 'changes');
    if (!existsSync(directory)) return [];
    return readdirSync(directory, { withFileTypes: true })
      .filter((entry) => entry.isDirectory() && entry.name !== 'archive')
      .map((entry) => entry.name);
  }).sort();
}

function gitOutput(root: string, args: string[]): string {
  try {
    return execFileSync('git', args, { cwd: root, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }).trim();
  } catch {
    throw new Error('release sealing requires a readable Git repository');
  }
}

export function validateReleaseSeal(
  root = process.cwd(),
  options: ReleaseAdapterOptions = {},
  validation: ReleaseSealValidationOptions = {},
): void {
  const latest = readManifests(root, options.releasesPath).at(-1);
  if (!latest) throw new Error('release sealing requires at least one release manifest');
  if (gitOutput(root, ['status', '--porcelain=v1', '--untracked-files=all'])) throw new Error('repository contains unsealed working tree changes');
  const manifestPath = `${options.releasesPath ?? 'releases'}/${latest.version}.yaml`.replaceAll('\\', '/');
  const manifestCommit = gitOutput(root, ['log', '--diff-filter=A', '-1', '--format=%H', '--', manifestPath]);
  if (!manifestCommit) throw new Error(`latest release manifest is not committed: ${manifestPath}`);
  const repositoryHead = gitOutput(root, ['rev-parse', 'HEAD']);
  const releaseHead = validation.pullRequestHead
    ? gitOutput(root, ['rev-parse', '--verify', `${validation.pullRequestHead}^{commit}`])
    : repositoryHead;
  if (releaseHead !== manifestCommit) throw new Error(`repository HEAD is not sealed by release ${latest.version}`);
  if (releaseHead !== repositoryHead) {
    const parents = gitOutput(root, ['show', '-s', '--format=%P', repositoryHead]).split(' ').filter(Boolean);
    if (parents.length !== 2 || !parents.includes(releaseHead)) {
      throw new Error('pull request release head is not a direct parent of the checked-out merge commit');
    }
  }
}

export function validateNpmPublication(
  tag: string,
  root = process.cwd(),
  options: ReleaseAdapterOptions = {},
): NpmPublicationIdentity {
  const version = tag.startsWith('v') ? tag.slice(1) : '';
  if (!semVerPattern.test(version) || tag !== `v${version}`) throw new Error(`invalid npm release tag: ${tag}`);

  const latest = readManifests(root, options.releasesPath).at(-1);
  if (!latest) throw new Error('npm publication requires a release manifest');
  if (latest.version !== version) throw new Error(`npm release tag ${tag} does not match release ${latest.version}`);

  const packagePath = join(root, 'packages', 'arcantry', 'package.json');
  const packageManifest = JSON.parse(readFileSync(packagePath, 'utf8')) as {
    name?: unknown;
    version?: unknown;
    repository?: { url?: unknown } | string;
  };
  const repositoryUrl = typeof packageManifest.repository === 'string'
    ? packageManifest.repository
    : packageManifest.repository?.url;
  if (packageManifest.name !== 'arcantry') throw new Error('npm package name must be arcantry');
  if (packageManifest.version !== version) throw new Error(`npm package version does not match tag ${tag}`);
  if (repositoryUrl !== 'https://github.com/MrMaxie/arcantry.git') {
    throw new Error('npm package repository must match https://github.com/MrMaxie/arcantry.git');
  }

  validateDistributionVersions(root, options.distributionManifestPaths, options.releasesPath);
  validateReleaseSeal(root, options);
  const head = gitOutput(root, ['rev-parse', 'HEAD']);
  const tagCommit = gitOutput(root, ['rev-list', '-n', '1', tag]);
  if (tagCommit !== head) throw new Error(`npm release tag ${tag} does not point to repository HEAD`);

  return { packageName: packageManifest.name, version, tag, repositoryUrl };
}

export function checkRelease(
  root = process.cwd(),
  options: ReleaseAdapterOptions = {},
  validation: ReleaseSealValidationOptions = {},
): void {
  const { archived, assigned } = validateReleaseState(root, options);
  const active = readActiveChangeIds(root, options.openSpecPaths);
  if (active.length > 0) throw new Error(`active OpenSpec changes are not release-complete: ${active.join(', ')}`);
  const unassigned = [...archived.keys()].filter((id) => !assigned.has(id)).sort();
  if (unassigned.length > 0) throw new Error(`archived OpenSpec changes are not assigned to a release: ${unassigned.join(', ')}`);
  checkChangelog(root, options);
  validateReleaseSeal(root, options, validation);
}

export function validateDistributionVersions(
  root = process.cwd(),
  distributions = [
    'packages/arcantry/package.json',
    '.codex-plugin/plugin.json',
    '.claude-plugin/plugin.json',
    'gemini-extension.json',
  ],
  releasesPath = 'releases',
): void {
  const expected = readManifests(root, releasesPath).at(-1)?.version ?? '0.0.0';
  for (const relativePath of distributions) {
    const path = join(root, relativePath);
    if (!existsSync(path)) continue;
    const manifest = JSON.parse(readFileSync(path, 'utf8')) as { version?: unknown };
    if (manifest.version !== expected) throw new Error(`distribution version must match release ${expected}: ${path}`);
  }
}

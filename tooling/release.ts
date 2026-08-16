import { existsSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { basename, join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { parse as parseYaml } from 'yaml';

export type Impact = 'none' | 'patch' | 'minor' | 'major';
export type Category = 'added' | 'changed' | 'fixed' | 'deprecated' | 'removed' | 'security';
export type Visibility = 'public' | 'internal';

export interface ReleaseArtifact {
  category: Category;
  impact: Impact;
  visibility: Visibility;
  title: string;
  body: string;
}

export interface ReleaseManifest {
  version: string;
  changes: string[];
}

const impactRank: Record<Impact, number> = { none: 0, patch: 1, minor: 2, major: 3 };
const categories: Category[] = ['added', 'changed', 'fixed', 'deprecated', 'removed', 'security'];
const impacts: Impact[] = ['none', 'patch', 'minor', 'major'];
const visibilities: Visibility[] = ['public', 'internal'];

export function parseReleaseArtifact(source: string): ReleaseArtifact {
  const match = source.match(/^---\r?\n([\s\S]*?)\r?\n---\r?\n([\s\S]*)$/);
  if (!match) throw new Error('release.md must start with YAML frontmatter');

  const metadata = parseYaml(match[1]) as Record<string, unknown>;
  const category = metadata.category as Category;
  const impact = metadata.impact as Impact;
  const visibility = metadata.visibility as Visibility;

  if (!categories.includes(category)) throw new Error(`invalid release category: ${String(category)}`);
  if (!impacts.includes(impact)) throw new Error(`invalid release impact: ${String(impact)}`);
  if (!visibilities.includes(visibility)) throw new Error(`invalid release visibility: ${String(visibility)}`);

  const content = match[2].trim();
  const heading = content.match(/^#\s+(.+)$/m);
  if (!heading) throw new Error('release.md must contain a level-one title');

  const title = heading[1].trim();
  const body = content.replace(/^#\s+.+\r?\n?/, '').trim();
  if (!body) throw new Error('release.md must describe the delivered outcome');

  return { category, impact, visibility, title, body };
}

export function highestImpact(values: Impact[]): Impact {
  return values.reduce<Impact>((highest, current) =>
    impactRank[current] > impactRank[highest] ? current : highest,
  'none');
}

export function bumpVersion(version: string, impact: Impact): string {
  const match = version.match(/^(\d+)\.(\d+)\.(\d+)$/);
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

export function readArchivedChanges(root = process.cwd()): Map<string, ReleaseArtifact> {
  const archive = join(root, 'openspec', 'changes', 'archive');
  const result = new Map<string, ReleaseArtifact>();
  if (!existsSync(archive)) return result;

  for (const directory of readdirSync(archive, { withFileTypes: true })) {
    if (!directory.isDirectory()) continue;
    const changeId = changeIdFromArchiveDirectory(directory.name);
    const path = join(archive, directory.name, 'release.md');
    if (!existsSync(path)) throw new Error(`archived change ${changeId} has no release.md`);
    result.set(changeId, parseReleaseArtifact(readFileSync(path, 'utf8')));
  }

  return result;
}

export function readManifests(root = process.cwd()): ReleaseManifest[] {
  const directory = join(root, 'releases');
  if (!existsSync(directory)) return [];

  return readdirSync(directory)
    .filter((file) => file.endsWith('.yaml'))
    .map((file) => {
      const data = parseYaml(readFileSync(join(directory, file), 'utf8')) as ReleaseManifest;
      if (!data?.version || !Array.isArray(data.changes)) throw new Error(`invalid release manifest: ${file}`);
      if (basename(file, '.yaml') !== data.version) throw new Error(`release manifest filename must match version: ${file}`);
      return data;
    })
    .sort((a, b) => compareSemVer(a.version, b.version));
}

function compareSemVer(a: string, b: string): number {
  const aa = a.split('.').map(Number);
  const bb = b.split('.').map(Number);
  for (let index = 0; index < 3; index += 1) {
    if (aa[index] !== bb[index]) return aa[index] - bb[index];
  }
  return 0;
}

export function planRelease(root = process.cwd()): { current: string; next: string; impact: Impact; changes: string[] } {
  const archived = readArchivedChanges(root);
  const manifests = readManifests(root);
  const assigned = new Set(manifests.flatMap((manifest) => manifest.changes));
  const changes = [...archived.keys()].filter((id) => !assigned.has(id)).sort();
  const impact = highestImpact(changes.map((id) => archived.get(id)!.impact));
  const current = manifests.at(-1)?.version ?? '0.0.0';
  return { current, next: bumpVersion(current, impact), impact, changes };
}

export function renderChangelog(root = process.cwd()): string {
  const archived = readArchivedChanges(root);
  const manifests = readManifests(root).sort((a, b) => compareSemVer(b.version, a.version));
  const lines = ['# Changelog', ''];

  for (const manifest of manifests) {
    lines.push(`## ${manifest.version}`, '');
    const grouped = new Map<Category, ReleaseArtifact[]>();

    for (const changeId of manifest.changes) {
      const artifact = archived.get(changeId);
      if (!artifact) throw new Error(`release ${manifest.version} references unknown archived change: ${changeId}`);
      if (artifact.visibility === 'internal') continue;
      grouped.set(artifact.category, [...(grouped.get(artifact.category) ?? []), artifact]);
    }

    for (const category of categories) {
      const entries = grouped.get(category);
      if (!entries?.length) continue;
      lines.push(`### ${category[0].toUpperCase()}${category.slice(1)}`, '');
      for (const entry of entries) lines.push(`#### ${entry.title}`, '', entry.body, '');
    }
  }

  return `${lines.join('\n').trimEnd()}\n`;
}

function main(): void {
  const command = process.argv[2];
  if (command === 'plan') {
    process.stdout.write(`${JSON.stringify(planRelease(), null, 2)}\n`);
    return;
  }
  if (command === 'render') {
    writeFileSync('CHANGELOG.md', renderChangelog(), 'utf8');
    return;
  }
  throw new Error('usage: release.ts <plan|render>');
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) main();

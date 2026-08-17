import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { basename, join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { parse as parseYaml, stringify as stringifyYaml } from 'yaml';

export type Impact = 'none' | 'patch' | 'minor' | 'major';
export type Category = 'added' | 'changed' | 'fixed' | 'deprecated' | 'removed' | 'security';
export type Visibility = 'public' | 'internal';

export interface ReleaseArtifact {
  category: Category;
  impact: Impact;
  visibility: Visibility;
  components: string[];
  title: string;
  body: string;
}

export interface ReleaseManifest {
  version: string;
  date: string;
  changes: string[];
}

interface ReleaseState {
  archived: Map<string, ReleaseArtifact>;
  manifests: ReleaseManifest[];
  assigned: Set<string>;
}

const impactRank: Record<Impact, number> = { none: 0, patch: 1, minor: 2, major: 3 };
const categories: Category[] = ['added', 'changed', 'fixed', 'deprecated', 'removed', 'security'];
const impacts: Impact[] = ['none', 'patch', 'minor', 'major'];
const visibilities: Visibility[] = ['public', 'internal'];
const componentPattern = /^[a-z0-9]+(?:-[a-z0-9]+)*(?::[a-z0-9]+(?:-[a-z0-9]+)*)?$/;
const semVerPattern = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/;
const datePattern = /^\d{4}-\d{2}-\d{2}$/;

export function parseReleaseArtifact(source: string): ReleaseArtifact {
  const match = source.match(/^---\r?\n([\s\S]*?)\r?\n---\r?\n([\s\S]*)$/);
  if (!match) throw new Error('release.md must start with YAML frontmatter');

  const metadata = parseYaml(match[1]) as Record<string, unknown>;
  const category = metadata.category as Category;
  const impact = metadata.impact as Impact;
  const visibility = metadata.visibility as Visibility;
  const components = metadata.components ?? [];

  if (!categories.includes(category)) throw new Error(`invalid release category: ${String(category)}`);
  if (!impacts.includes(impact)) throw new Error(`invalid release impact: ${String(impact)}`);
  if (!visibilities.includes(visibility)) throw new Error(`invalid release visibility: ${String(visibility)}`);
  if (!Array.isArray(components) || !components.every((component) => typeof component === 'string' && componentPattern.test(component))) {
    throw new Error('release components must be an array of component identifiers');
  }
  if (new Set(components).size !== components.length) throw new Error('release components must be unique');

  const content = match[2].trim();
  const heading = content.match(/^#\s+(.+)$/m);
  if (!heading) throw new Error('release.md must contain a level-one title');

  const title = heading[1].trim();
  const body = content.replace(/^#\s+.+\r?\n?/, '').trim();
  if (!body) throw new Error('release.md must describe the delivered outcome');

  return { category, impact, visibility, components, title, body };
}

export function highestImpact(values: Impact[]): Impact {
  return values.reduce<Impact>(
    (highest, current) => impactRank[current] > impactRank[highest] ? current : highest,
    'none',
  );
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

export function readArchivedChanges(root = process.cwd()): Map<string, ReleaseArtifact> {
  const archive = join(root, 'openspec', 'changes', 'archive');
  const result = new Map<string, ReleaseArtifact>();
  if (!existsSync(archive)) return result;

  for (const directory of readdirSync(archive, { withFileTypes: true })) {
    if (!directory.isDirectory()) continue;
    const changeId = changeIdFromArchiveDirectory(directory.name);
    if (result.has(changeId)) throw new Error(`duplicate archived change id: ${changeId}`);

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
      const data = parseYaml(readFileSync(join(directory, file), 'utf8')) as Partial<ReleaseManifest>;
      if (!data || typeof data.version !== 'string' || !semVerPattern.test(data.version)) {
        throw new Error(`invalid release version: ${file}`);
      }
      if (basename(file, '.yaml') !== data.version) {
        throw new Error(`release manifest filename must match version: ${file}`);
      }
      if (typeof data.date !== 'string' || !datePattern.test(data.date)) {
        throw new Error(`invalid release date: ${file}`);
      }
      if (!Array.isArray(data.changes) || !data.changes.every((change) => typeof change === 'string' && change.length > 0)) {
        throw new Error(`invalid release changes: ${file}`);
      }
      if (new Set(data.changes).size !== data.changes.length) {
        throw new Error(`duplicate change in release manifest: ${file}`);
      }
      return data as ReleaseManifest;
    })
    .sort((a, b) => compareSemVer(a.version, b.version));
}

function compareSemVer(a: string, b: string): number {
  if (!semVerPattern.test(a)) throw new Error(`invalid SemVer version: ${a}`);
  if (!semVerPattern.test(b)) throw new Error(`invalid SemVer version: ${b}`);

  const aa = a.split('.').map(Number);
  const bb = b.split('.').map(Number);
  for (let index = 0; index < 3; index += 1) {
    if (aa[index] !== bb[index]) return aa[index] - bb[index];
  }
  return 0;
}

export function validateReleaseState(root = process.cwd()): ReleaseState {
  const archived = readArchivedChanges(root);
  const manifests = readManifests(root);
  const assigned = new Set<string>();

  for (const manifest of manifests) {
    for (const changeId of manifest.changes) {
      if (!archived.has(changeId)) {
        throw new Error(`release ${manifest.version} references unknown archived change: ${changeId}`);
      }
      if (assigned.has(changeId)) throw new Error(`archived change assigned more than once: ${changeId}`);
      assigned.add(changeId);
    }
  }

  return { archived, manifests, assigned };
}

export function planRelease(root = process.cwd()): { current: string; next: string; impact: Impact; changes: string[] } {
  const { archived, manifests, assigned } = validateReleaseState(root);
  const changes = [...archived.keys()].filter((id) => !assigned.has(id)).sort();
  const impact = highestImpact(changes.map((id) => archived.get(id)!.impact));
  const current = manifests.at(-1)?.version ?? '0.0.0';
  return { current, next: bumpVersion(current, impact), impact, changes };
}

export function cutRelease(root = process.cwd(), date = new Date().toISOString().slice(0, 10)): ReleaseManifest {
  if (!datePattern.test(date)) throw new Error(`invalid release date: ${date}`);

  const plan = planRelease(root);
  if (plan.changes.length === 0) throw new Error('no unassigned archived changes to release');
  if (plan.impact === 'none') throw new Error('unassigned changes do not require a SemVer release');

  const manifest: ReleaseManifest = { version: plan.next, date, changes: plan.changes };
  const directory = join(root, 'releases');
  mkdirSync(directory, { recursive: true });
  const path = join(directory, `${manifest.version}.yaml`);
  if (existsSync(path)) throw new Error(`release manifest already exists: ${manifest.version}`);
  writeFileSync(path, stringifyYaml(manifest, { lineWidth: 0 }), 'utf8');
  return manifest;
}

export function renderChangelog(root = process.cwd()): string {
  const { archived, manifests } = validateReleaseState(root);
  const ordered = [...manifests].sort((a, b) => compareSemVer(b.version, a.version));
  const lines = ['# Changelog', ''];

  for (const manifest of ordered) {
    lines.push(`## ${manifest.version} - ${manifest.date}`, '');
    const grouped = new Map<Category, Array<{ id: string; artifact: ReleaseArtifact }>>();

    for (const changeId of manifest.changes) {
      const artifact = archived.get(changeId)!;
      if (artifact.visibility === 'internal') continue;
      grouped.set(artifact.category, [...(grouped.get(artifact.category) ?? []), { id: changeId, artifact }]);
    }

    for (const category of categories) {
      const entries = grouped.get(category);
      if (!entries?.length) continue;
      lines.push(`### ${category[0].toUpperCase()}${category.slice(1)}`, '');
      for (const { id, artifact } of entries) {
        lines.push(`<!-- openspec: ${id} -->`, `#### ${artifact.title}`, '', artifact.body, '');
      }
    }
  }

  return `${lines.join('\n').trimEnd()}\n`;
}

export function checkChangelog(root = process.cwd()): void {
  const path = join(root, 'CHANGELOG.md');
  if (!existsSync(path)) throw new Error('CHANGELOG.md is missing');
  validateDistributionVersions(root);
  const expected = renderChangelog(root);
  const actual = readFileSync(path, 'utf8');
  if (actual !== expected) throw new Error('CHANGELOG.md is stale; run `just release-render`');
}

export function validateDistributionVersions(root = process.cwd()): void {
  const manifests = readManifests(root);
  const expected = manifests.at(-1)?.version ?? '0.0.0';
  const distributions = [
    join(root, 'packages', 'arcantry', 'package.json'),
    join(root, '.codex-plugin', 'plugin.json'),
  ];

  for (const path of distributions) {
    if (!existsSync(path)) continue;
    const manifest = JSON.parse(readFileSync(path, 'utf8')) as { version?: unknown };
    if (manifest.version !== expected) {
      throw new Error(`distribution version must match release ${expected}: ${path}`);
    }
  }
}

function main(): void {
  const command = process.argv[2];
  if (command === 'plan') {
    process.stdout.write(`${JSON.stringify(planRelease(), null, 2)}\n`);
    return;
  }
  if (command === 'cut') {
    process.stdout.write(`${JSON.stringify(cutRelease(), null, 2)}\n`);
    return;
  }
  if (command === 'render') {
    writeFileSync('CHANGELOG.md', renderChangelog(), 'utf8');
    return;
  }
  if (command === 'check') {
    checkChangelog();
    return;
  }
  throw new Error('usage: release.ts <plan|cut|render|check>');
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) main();

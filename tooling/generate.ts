import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { readCatalog, readSkillAgent, readSkillFrontmatter, readSkillMetadata, validateCatalog } from './catalog.js';
import { readArchivedChanges, readManifests } from './release.js';
import { buildVersionModel } from './version.js';

const root = process.cwd();
const check = process.argv.includes('--check');
let stale = false;

function project(path: string, content: string): void {
  const current = existsSync(path) ? readFileSync(path, 'utf8') : undefined;
  if (current === content) return;
  stale = true;
  if (!check) {
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, content, 'utf8');
  }
}

project(
  join(root, 'public', 'schemas', 'arcantry-config-v1.tosd'),
  readFileSync(join(root, 'schemas', 'arcantry-config-v1.tosd'), 'utf8'),
);

function escapeHtml(value: string): string {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

const validationErrors = validateCatalog(root);
if (validationErrors.length > 0) throw new Error(validationErrors.join('\n'));

const catalog = readCatalog(root);
const versionModel = buildVersionModel(root);
const version = versionModel.currentRelease;
const archivedChanges = readArchivedChanges(root);
const releaseManifests = readManifests(root);

function skillReleaseVersion(name: string): string | undefined {
  return versionModel.skillReleases[name];
}

const plugin = {
  name: 'arcantry',
  version,
  description: 'Repository lifecycle, adoption tooling, and focused agent skills.',
  author: { name: 'Maxie', url: 'https://github.com/MrMaxie' },
  homepage: 'https://maxie.dev/arcantry/',
  repository: 'https://github.com/MrMaxie/arcantry',
  license: 'Apache-2.0',
  keywords: ['agent-skills', 'codex', 'repository-lifecycle'],
  skills: './skills/',
  interface: {
    displayName: 'Arcantry',
    shortDescription: 'Intent-led repositories and focused agent skills',
    longDescription: 'Adopt a reproducible repository contract and install focused skills from one versioned Arcantry catalog.',
    developerName: 'Maxie',
    category: 'Developer Tools',
    capabilities: ['Skills', 'Read', 'Write'],
    websiteURL: 'https://maxie.dev/arcantry/',
    brandColor: '#FB255F',
    defaultPrompt: ['Use Arcantry to adopt this repository or choose the smallest focused skill for the task.'],
  },
};
project(join(root, '.codex-plugin', 'plugin.json'), `${JSON.stringify(plugin, null, 2)}\n`);

const detailsRoot = join(root, 'src', 'content', 'docs', 'skills');
if (!check) mkdirSync(detailsRoot, { recursive: true });
const expected = new Set<string>();
const catalogCards = new Map<string, string>();
const catalogSections = [
  {
    title: 'Start with your repository',
    skills: ['adopt-arcantry'],
  },
  {
    title: 'Build and improve skills',
    skills: [
      'agent-self-improve',
      'audit-skill-portfolio',
      'capture-repeatable-work',
      'evaluate-skill-change',
      'forge-skill-from-conversations',
      'productize-repeatable-work',
      'select-task-skills',
    ],
  },
  {
    title: 'Design for people',
    skills: ['audience-scope-discipline', 'design-terminal-ux'],
  },
  {
    title: 'Coordinate delivery',
    skills: [
      'intake-linear-work',
      'intake-repository-work',
      'promote-meeting-notes',
      'stage-code-review-findings',
    ],
  },
] as const;

for (const entry of catalog.skills) {
  const metadata = readSkillMetadata(root, entry.name);
  const frontmatter = readSkillFrontmatter(root, entry.name);
  const agent = readSkillAgent(root, entry.name);
  const filename = `${entry.name}.md`;
  expected.add(filename);
  catalogCards.set(
    entry.name,
    [
      `<a class="skill-catalog-card" href="/arcantry/skills/${entry.name}/">`,
      `  <p class="skill-catalog-version">${skillReleaseVersion(entry.name) ?? 'Unreleased'}</p>`,
      `  <h3>${escapeHtml(agent.displayName)}</h3>`,
      `  <p class="skill-catalog-summary">${escapeHtml(agent.shortDescription)}</p>`,
      `  <p class="skill-catalog-command"><code>$${escapeHtml(entry.name)}</code></p>`,
      '</a>',
    ].join('\n'),
  );
  const scenarios = metadata.scenarios
    .map(
      (scenario) =>
        `### ${scenario.title}\n\n**Prompt**\n\n> ${scenario.prompt}\n\n**Expected outcome**\n\n${scenario.outcome}`,
    )
    .join('\n\n');
  const compatibility = metadata.compatibility === undefined
    ? ''
    : `## Compatibility\n\nSource kinds: ${metadata.compatibility.sourceKinds.map((kind) => `\`${kind}\``).join(', ')}.${
        metadata.compatibility.adapters === undefined
          ? ''
          : `\n\nAdapters:\n\n${metadata.compatibility.adapters.map((adapter) => `- \`${adapter.name}\`: \`${adapter.versions}\``).join('\n')}`
      }\n\n`;
  const learning = metadata.learning === undefined
    ? ''
    : `## Learning outcomes\n\n${metadata.learning.outcomes.map((outcome) => `- ${outcome}`).join('\n')}\n\n`;
  const history = releaseManifests
    .flatMap((manifest) =>
      manifest.changes.flatMap((changeId) => {
        const artifact = archivedChanges.get(changeId);
        return artifact?.components.includes(`skill:${entry.name}`)
          ? [`- **${manifest.version}** — ${artifact.title}`]
          : [];
      }),
    )
    .reverse();
  const firstRelease = skillReleaseVersion(entry.name);
  const releaseState = firstRelease ? `Included since Arcantry ${firstRelease}` : 'Unreleased';
  const historySection = history.length > 0 ? history.join('\n') : 'This skill is not part of a released Arcantry version yet.';
  const content = `---\ntitle: ${agent.displayName}\ndescription: ${JSON.stringify(metadata.summary)}\n---\n\n<!-- Generated by tooling/generate.ts. Do not edit directly. -->\n\n<p class="skill-reference-meta"><span>${releaseState}</span><code>$${entry.name}</code></p>\n\n${frontmatter.description}\n\n## Link this skill\n\n\`\`\`text\narcantry skills link ${entry.name}\n\`\`\`\n\n${compatibility}${learning}## Examples\n\n${scenarios}\n\n## Release history\n\n${historySection}\n`;
  project(join(detailsRoot, filename), content);
}

const catalogSectionNames = catalogSections.flatMap((section) => [...section.skills]);
const duplicateSectionNames = catalogSectionNames.filter(
  (name, index) => catalogSectionNames.indexOf(name) !== index,
);
const missingSectionNames = catalog.skills
  .map((entry) => entry.name)
  .filter((name) => !catalogSectionNames.includes(name as (typeof catalogSectionNames)[number]));
const unknownSectionNames = catalogSectionNames.filter((name) => !catalogCards.has(name));

if (duplicateSectionNames.length > 0 || missingSectionNames.length > 0 || unknownSectionNames.length > 0) {
  throw new Error(
    [
      duplicateSectionNames.length > 0 ? `Duplicate catalog section skills: ${duplicateSectionNames.join(', ')}` : '',
      missingSectionNames.length > 0 ? `Skills missing from catalog sections: ${missingSectionNames.join(', ')}` : '',
      unknownSectionNames.length > 0 ? `Unknown catalog section skills: ${unknownSectionNames.join(', ')}` : '',
    ]
      .filter(Boolean)
      .join('\n'),
  );
}

const catalogSectionsContent = catalogSections
  .map((section) => {
    const cards = section.skills.map((name) => catalogCards.get(name)).join('\n');
    return `## ${section.title}\n\n<div class="skill-catalog-grid">\n${cards}\n</div>`;
  })
  .join('\n\n');

expected.add('catalog.md');
project(
  join(detailsRoot, 'catalog.md'),
  `---\ntitle: Skill catalog\ndescription: Choose a focused Arcantry skill for the work at hand.\n---\n\n<!-- Generated by tooling/generate.ts. Do not edit directly. -->\n\n<p class="skill-catalog-lead">Choose the job. Each page shows when the skill shipped and how to link it.</p>\n\n${catalogSectionsContent}\n`,
);

if (existsSync(detailsRoot)) {
  for (const file of readdirSync(detailsRoot)) {
    if (file === 'index.md' || expected.has(file) || !file.endsWith('.md')) continue;
    stale = true;
    if (!check) rmSync(join(detailsRoot, file));
  }
}

if (stale && check) {
  process.stderr.write('Generated projections are stale. Run `pnpm run generate`.\n');
  process.exitCode = 1;
} else if (!check) {
  process.stdout.write('Generated Arcantry projections.\n');
}

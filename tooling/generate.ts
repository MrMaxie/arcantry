import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { readCatalog, readSkillAgent, readSkillFrontmatter, readSkillMetadata, validateCatalog } from './catalog.js';
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
const plugin = {
  name: 'arcantry',
  version,
  description: 'Local-first project knowledge and focused agent skills.',
  author: { name: 'Maxie', url: 'https://github.com/MrMaxie' },
  homepage: 'https://maxie.dev/arcantry/',
  repository: 'https://github.com/MrMaxie/arcantry',
  license: 'Apache-2.0',
  keywords: ['agent-skills', 'codex', 'repository-lifecycle'],
  skills: './skills/',
  interface: {
    displayName: 'Arcantry',
    shortDescription: 'Intent-led repositories and focused agent skills',
    longDescription: 'Coordinate project knowledge and install focused skills for self-improvement, repository safety, and content safety.',
    developerName: 'Maxie',
    category: 'Developer Tools',
    capabilities: ['Skills', 'Read', 'Write'],
    websiteURL: 'https://maxie.dev/arcantry/',
    brandColor: '#FB255F',
    defaultPrompt: ['Use Arcantry to adopt this repository or choose the smallest focused skill for the task.'],
  },
};
project(join(root, '.codex-plugin', 'plugin.json'), `${JSON.stringify(plugin, null, 2)}\n`);
project(
  join(root, '.claude-plugin', 'plugin.json'),
  `${JSON.stringify({
    name: plugin.name,
    version: plugin.version,
    description: plugin.description,
    author: plugin.author,
    homepage: plugin.homepage,
    repository: plugin.repository,
    license: plugin.license,
    keywords: ['agent-skills', 'claude-code', 'repository-lifecycle'],
  }, null, 2)}\n`,
);
project(
  join(root, 'gemini-extension.json'),
  `${JSON.stringify({
    name: plugin.name,
    version: plugin.version,
    description: plugin.description,
  }, null, 2)}\n`,
);

const detailsRoot = join(root, 'src', 'content', 'docs', 'skills');
if (!check) mkdirSync(detailsRoot, { recursive: true });
const expected = new Set<string>();
const catalogCards = new Map<string, string>();
const catalogSections = [
  { family: 'self-improvement', title: 'Self improvement' },
  { family: 'repo-safely', title: 'Repo safely' },
  { family: 'content-safely', title: 'Content safely' },
].map((section) => ({
  ...section,
  skills: catalog.skills.filter((entry) => entry.family === section.family).map((entry) => entry.name),
}));

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
      `  <p class="skill-catalog-version">Arcantry ${version}</p>`,
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
  const content = `---\ntitle: ${agent.displayName}\ndescription: ${JSON.stringify(metadata.summary)}\n---\n\n<!-- Generated by tooling/generate.ts. Do not edit directly. -->\n\n<p class="skill-reference-meta"><span>Arcantry ${version}</span><code>$${entry.name}</code></p>\n\n${frontmatter.description}\n\n## Link this skill\n\n\`\`\`text\narcantry skills link ${entry.name} --scope user\narcantry skills link ${entry.name} --scope user --agent claude\narcantry skills link ${entry.name} --scope user --agent gemini\n\`\`\`\n\nThe default target is the Codex-compatible Agent Skills directory. Use the explicit agent profile for Claude Code or Gemini CLI.\n\n${compatibility}${learning}## Examples\n\n${scenarios}\n`;
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
  `---\ntitle: Skill catalog\ndescription: Choose a focused Arcantry skill for the work at hand.\n---\n\n<!-- Generated by tooling/generate.ts. Do not edit directly. -->\n\n<p class="skill-catalog-lead">Choose the job. Each page explains the outcome, compatibility, and link command.</p>\n\n${catalogSectionsContent}\n`,
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

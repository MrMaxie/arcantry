import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { Command } from 'commander';
import { readCatalog, readSkillAgent, readSkillFrontmatter, readSkillMetadata, validateCatalog } from './catalog.js';
import { buildVersionModel } from './version.js';

const root = process.cwd();
const options = new Command()
  .name('generate.ts')
  .description('Generate Arcantry package and documentation projections.')
  .option('--check', 'Verify tracked generated files without writing them.')
  .option('--docs-only', 'Generate only documentation projections.')
  .showHelpAfterError()
  .parse()
  .opts<{ check?: boolean; docsOnly?: boolean }>();
const check = options.check ?? false;
const docsOnly = options.docsOnly ?? false;
let stale = false;

const project = (path: string, content: string, tracked = true): void => {
  const current = existsSync(path) ? readFileSync(path, 'utf8') : undefined;
  if (current === content) return;
  if (tracked) stale = true;
  if (!check) {
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, content, 'utf8');
  }
};

const generatedDocsRoot = join(root, 'apps', 'docs', 'src', 'content', 'docs', '_generated');
const detailsRoot = join(generatedDocsRoot, 'skills');
const generatedSourceRoot = join(root, 'apps', 'docs', 'src', 'generated');
const generatedPublicRoot = join(root, 'apps', 'docs', 'public', 'schemas');

if (!check) {
  rmSync(generatedDocsRoot, { force: true, recursive: true });
  rmSync(generatedSourceRoot, { force: true, recursive: true });
  rmSync(generatedPublicRoot, { force: true, recursive: true });
}

project(
  join(generatedPublicRoot, 'arcantry-config-v1.tosd'),
  readFileSync(join(root, 'schemas', 'arcantry-config-v1.tosd'), 'utf8'),
  false,
);

const escapeHtml = (value: string): string => {
  return value
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
};

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
    longDescription:
      'Coordinate project knowledge and install focused skills for self-improvement, repository safety, and content safety.',
    developerName: 'Maxie',
    category: 'Developer Tools',
    capabilities: ['Skills', 'Read', 'Write'],
    websiteURL: 'https://maxie.dev/arcantry/',
    brandColor: '#FB255F',
    defaultPrompt: ['Use Arcantry to adopt this repository or choose the smallest focused skill for the task.'],
  },
};
if (!docsOnly) {
  project(join(root, '.codex-plugin', 'plugin.json'), `${JSON.stringify(plugin, null, 2)}\n`);
  project(
    join(root, '.claude-plugin', 'plugin.json'),
    `${JSON.stringify(
      {
        name: plugin.name,
        version: plugin.version,
        description: plugin.description,
        author: plugin.author,
        homepage: plugin.homepage,
        repository: plugin.repository,
        license: plugin.license,
        keywords: ['agent-skills', 'claude-code', 'repository-lifecycle'],
      },
      null,
      2,
    )}\n`,
  );
}
if (!check) mkdirSync(detailsRoot, { recursive: true });
const catalogCards = new Map<string, string>();
const skillDisplayNames = new Map<string, string>();
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
  skillDisplayNames.set(entry.name, agent.displayName);
  const filename = `${entry.name}.md`;
  catalogCards.set(
    entry.name,
    [
      `<a class="skill-catalog-card" href="/arcantry/skills/${entry.name}/">`,
      `  <p class="skill-catalog-version">Arcantry ${version}</p>`,
      `  <h3>${escapeHtml(agent.displayName)}</h3>`,
      `  <p class="skill-catalog-summary">${escapeHtml(metadata.summary)}</p>`,
      `  <p class="skill-catalog-command"><code>${escapeHtml(entry.name)}</code></p>`,
      '</a>',
    ].join('\n'),
  );
  const scenarios = metadata.scenarios
    .map(
      (scenario) =>
        `### ${scenario.title}\n\n**Prompt**\n\n> ${scenario.prompt}\n\n**Expected outcome**\n\n${scenario.outcome}`,
    )
    .join('\n\n');
  const compatibility =
    metadata.compatibility === undefined
      ? ''
      : `## Compatibility\n\nSource kinds: ${metadata.compatibility.sourceKinds.map((kind) => `\`${kind}\``).join(', ')}.${
          metadata.compatibility.adapters === undefined
            ? ''
            : `\n\nAdapters:\n\n${metadata.compatibility.adapters.map((adapter) => `- \`${adapter.name}\`: \`${adapter.versions}\``).join('\n')}`
        }\n\n`;
  const learning =
    metadata.learning === undefined
      ? ''
      : `## Learning outcomes\n\n${metadata.learning.outcomes.map((outcome) => `- ${outcome}`).join('\n')}\n\n`;
  const content = `---\ntitle: ${agent.displayName}\ndescription: ${JSON.stringify(metadata.summary)}\n---\n\n<!-- Generated by tooling/generate.ts. Do not edit directly. -->\n\n<p class="skill-reference-meta"><span>Arcantry ${version}</span><code>${entry.name}</code></p>\n\n${metadata.summary}\n\n## When to use\n\n${frontmatter.description}\n\n## Link this skill\n\n\`\`\`text\narcantry skills link ${entry.name} --scope user\n\`\`\`\n\nThe standard destination is \`~/.agents/skills\`. Codex reads this universal Agent Skills location directly.\n\n### Claude compatibility\n\n\`\`\`text\narcantry skills link ${entry.name} --scope user --compat claude\n\`\`\`\n\nThis keeps the universal link and adds a Claude alias to the same canonical package.\n\n${compatibility}${learning}## Examples\n\n${scenarios}\n`;
  project(join(detailsRoot, filename), content, false);
}

const catalogSectionNames = catalogSections.flatMap((section) => [...section.skills]);
const duplicateSectionNames = catalogSectionNames.filter((name, index) => catalogSectionNames.indexOf(name) !== index);
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

project(
  join(generatedSourceRoot, 'skill-sidebar.ts'),
  `// Generated by tooling/generate.ts. Do not edit directly.\nexport default ${JSON.stringify(
    catalogSections.map((section) => ({
      label: section.title,
      collapsed: true,
      items: section.skills.map((name) => ({ label: skillDisplayNames.get(name), slug: `skills/${name}` })),
    })),
    null,
    2,
  )};\n`,
  false,
);

project(
  join(detailsRoot, 'catalog.md'),
  `---\ntitle: Skill catalog\ndescription: Choose a focused Arcantry skill for the work at hand.\n---\n\n<!-- Generated by tooling/generate.ts. Do not edit directly. -->\n\n<p class="skill-catalog-lead">Choose the job. Each page explains the outcome, compatibility, and link command.</p>\n\n${catalogSectionsContent}\n`,
  false,
);

if (stale && check) {
  process.stderr.write('Generated tracked projections are stale. Run `just generate`.\n');
  process.exitCode = 1;
} else if (!check) {
  process.stdout.write('Generated Arcantry projections.\n');
}

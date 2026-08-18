import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { parse as parseYaml } from 'yaml';

export interface CatalogEntry {
  name: string;
  family: 'self-improvement' | 'repo-safely' | 'content-safely';
  tags: string[];
}

export interface Catalog {
  $schema?: string;
  skills: CatalogEntry[];
}

export interface SkillScenario {
  title: string;
  prompt: string;
  outcome: string;
}

export interface SkillMetadata {
  $schema?: string;
  summary: string;
  scenarios: SkillScenario[];
  compatibility?: {
    sourceKinds: Array<'openspec' | 'changelog' | 'todo-txt'>;
    adapters?: Array<{ name: string; versions: string }>;
  };
  learning?: {
    prerequisites?: string[];
    outcomes: string[];
  };
}

export interface SkillAgent {
  displayName: string;
  shortDescription: string;
  defaultPrompt: string;
}

const skillNamePattern = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;
const skillFamilies = new Set(['self-improvement', 'repo-safely', 'content-safely']);
const catalogSchemaPath = './schemas/catalog.schema.json';
const skillMetadataSchemaPath = '../../schemas/skill-metadata.schema.json';

export function readCatalog(root = process.cwd()): Catalog {
  return JSON.parse(readFileSync(join(root, 'catalog.json'), 'utf8')) as Catalog;
}

export function readSkillMetadata(root: string, name: string): SkillMetadata {
  return JSON.parse(readFileSync(join(root, 'skills', name, 'arcantry.json'), 'utf8')) as SkillMetadata;
}

export function readSkillFrontmatter(root: string, name: string): { name: string; description: string } {
  const source = readFileSync(join(root, 'skills', name, 'SKILL.md'), 'utf8');
  const match = source.match(/^---\r?\n([\s\S]*?)\r?\n---/);
  if (!match) throw new Error(`skills/${name}/SKILL.md has no YAML frontmatter`);

  const data = parseYaml(match[1]) as Record<string, unknown>;
  if (typeof data.name !== 'string' || typeof data.description !== 'string') {
    throw new Error(`skills/${name}/SKILL.md must declare name and description`);
  }
  return { name: data.name, description: data.description };
}

export function readSkillAgent(root: string, name: string): SkillAgent {
  const source = parseYaml(readFileSync(join(root, 'skills', name, 'agents', 'openai.yaml'), 'utf8')) as {
    interface?: { display_name?: unknown; short_description?: unknown; default_prompt?: unknown };
  };
  const displayName = source.interface?.display_name;
  const shortDescription = source.interface?.short_description;
  const defaultPrompt = source.interface?.default_prompt;
  if (typeof displayName !== 'string' || typeof shortDescription !== 'string' || typeof defaultPrompt !== 'string') {
    throw new Error(`skills/${name} agents/openai.yaml must declare interface display_name, short_description, and default_prompt`);
  }
  return { displayName, shortDescription, defaultPrompt };
}

export function validateCatalog(root = process.cwd()): string[] {
  const errors: string[] = [];
  const catalog = readCatalog(root);
  if (!Array.isArray(catalog.skills)) return ['catalog.json must contain a skills array'];
  if (catalog.$schema !== catalogSchemaPath) errors.push(`catalog.json $schema must be ${catalogSchemaPath}`);
  if (Object.keys(catalog).some((key) => key !== '$schema' && key !== 'skills')) {
    errors.push('catalog.json must not contain unsupported fields');
  }

  const names = catalog.skills.map((entry) => entry.name);
  const sortedNames = [...names].sort((a, b) => a.localeCompare(b));
  if (JSON.stringify(names) !== JSON.stringify(sortedNames)) errors.push('catalog skills must be sorted by name');
  if (new Set(names).size !== names.length) errors.push('catalog skill names must be unique');

  const skillsRoot = join(root, 'skills');
  const directories = existsSync(skillsRoot)
    ? readdirSync(skillsRoot, { withFileTypes: true })
        .filter((entry) => entry.isDirectory())
        .map((entry) => entry.name)
        .sort((a, b) => a.localeCompare(b))
    : [];
  if (JSON.stringify(directories) !== JSON.stringify(sortedNames)) {
    errors.push('catalog membership must exactly match skills/ directories');
  }

  for (const entry of catalog.skills) {
    if (!skillNamePattern.test(entry.name) || entry.name.length > 63) errors.push(`invalid skill name: ${entry.name}`);
    if (!skillFamilies.has(entry.family)) errors.push(`skills/${entry.name} family is invalid`);
    if (
      !Array.isArray(entry.tags) ||
      entry.tags.length === 0 ||
      new Set(entry.tags).size !== entry.tags.length ||
      !entry.tags.every((tag) => typeof tag === 'string' && skillNamePattern.test(tag))
    ) {
      errors.push(`skills/${entry.name} must have unique tags`);
    }
    if (Object.keys(entry).some((key) => key !== 'name' && key !== 'family' && key !== 'tags')) {
      errors.push(`skills/${entry.name} must not contain unsupported fields`);
    }

    const directory = join(skillsRoot, entry.name);
    for (const required of ['SKILL.md', 'arcantry.json', join('agents', 'openai.yaml')]) {
      if (!existsSync(join(directory, required))) errors.push(`skills/${entry.name} is missing ${required}`);
    }
    if (!existsSync(join(directory, 'SKILL.md')) || !existsSync(join(directory, 'arcantry.json'))) continue;

    try {
      const frontmatter = readSkillFrontmatter(root, entry.name);
      if (frontmatter.name !== entry.name) errors.push(`skills/${entry.name} frontmatter name must match its directory`);
      if (frontmatter.description.trim().length < 30) errors.push(`skills/${entry.name} description is too short`);
    } catch (error) {
      errors.push(error instanceof Error ? error.message : String(error));
    }

    const metadata = readSkillMetadata(root, entry.name);
    if (metadata.$schema !== skillMetadataSchemaPath) {
      errors.push(`skills/${entry.name} metadata $schema must be ${skillMetadataSchemaPath}`);
    }
    if (Object.keys(metadata).some((key) => !['$schema', 'summary', 'scenarios', 'compatibility', 'learning'].includes(key))) {
      errors.push(`skills/${entry.name} metadata must not contain unsupported fields`);
    }
    if (typeof metadata.summary !== 'string' || metadata.summary.length < 30 || metadata.summary.length > 180) {
      errors.push(`skills/${entry.name} summary must contain 30-180 characters`);
    }
    if (!Array.isArray(metadata.scenarios) || metadata.scenarios.length !== 2) {
      errors.push(`skills/${entry.name} must define exactly two scenarios`);
    } else {
      for (const [index, scenario] of metadata.scenarios.entries()) {
        if (
          !scenario.title ||
          !scenario.prompt ||
          !scenario.outcome ||
          scenario.title.trim().length < 5 ||
          scenario.title.trim().length > 60 ||
          scenario.prompt.trim().length < 15 ||
          scenario.prompt.trim().length > 280 ||
          scenario.outcome.trim().length < 15 ||
          scenario.outcome.trim().length > 220
        ) {
          errors.push(`skills/${entry.name} scenario ${index + 1} is incomplete or outside the supported length`);
        }
        if (Object.keys(scenario).some((key) => key !== 'title' && key !== 'prompt' && key !== 'outcome')) {
          errors.push(`skills/${entry.name} scenario ${index + 1} must not contain unsupported fields`);
        }
      }
    }
    if (metadata.compatibility !== undefined) {
      const allowedKinds = new Set(['openspec', 'changelog', 'todo-txt']);
      if (
        Object.keys(metadata.compatibility).some((key) => key !== 'sourceKinds' && key !== 'adapters') ||
        !Array.isArray(metadata.compatibility.sourceKinds) ||
        metadata.compatibility.sourceKinds.length === 0 ||
        new Set(metadata.compatibility.sourceKinds).size !== metadata.compatibility.sourceKinds.length ||
        !metadata.compatibility.sourceKinds.every((kind) => allowedKinds.has(kind))
      ) {
        errors.push(`skills/${entry.name} compatibility sourceKinds are invalid`);
      }
      if (metadata.compatibility.adapters !== undefined && (
        !Array.isArray(metadata.compatibility.adapters) ||
        !metadata.compatibility.adapters.every((adapter) =>
          Object.keys(adapter).every((key) => key === 'name' || key === 'versions') &&
          skillNamePattern.test(adapter.name) &&
          typeof adapter.versions === 'string' &&
          /\d/.test(adapter.versions)
        )
      )) {
        errors.push(`skills/${entry.name} compatibility adapters are invalid`);
      }
    }
    if (metadata.learning !== undefined) {
      const validStatements = (values: unknown, required: boolean): boolean =>
        (!required && values === undefined) ||
        (Array.isArray(values) && values.length > 0 && values.every((value) => typeof value === 'string' && value.trim().length >= 5 && value.trim().length <= 160));
      if (
        Object.keys(metadata.learning).some((key) => key !== 'prerequisites' && key !== 'outcomes') ||
        !validStatements(metadata.learning.prerequisites, false) ||
        !validStatements(metadata.learning.outcomes, true)
      ) {
        errors.push(`skills/${entry.name} learning metadata is invalid`);
      }
    }

    const agentPath = join(directory, 'agents', 'openai.yaml');
    if (existsSync(agentPath)) {
      try {
        const agent = readSkillAgent(root, entry.name);
        if (agent.displayName.trim().length < 3) {
          errors.push(`skills/${entry.name} display_name is too short`);
        }
        if (agent.shortDescription.trim().length < 15 || agent.shortDescription.length > 80) {
          errors.push(`skills/${entry.name} short_description must contain 15-80 characters`);
        }
        if (!agent.defaultPrompt.includes(`$${entry.name}`)) {
          errors.push(`skills/${entry.name} default_prompt must mention $${entry.name}`);
        }
      } catch (error) {
        errors.push(error instanceof Error ? error.message : String(error));
      }
    }

    for (const markdownPath of markdownFiles(directory)) {
      const markdown = readFileSync(markdownPath, 'utf8');
      for (const match of markdown.matchAll(/\[[^\]]*\]\(([^)]+)\)/g)) {
        const target = match[1].split('#', 1)[0];
        if (!target || /^(?:https?:|mailto:)/.test(target)) continue;
        const path = resolve(dirname(markdownPath), target);
        if (!existsSync(path)) errors.push(`${markdownPath.slice(root.length + 1)} references missing ${target}`);
      }
    }
  }

  return errors;
}

function markdownFiles(directory: string): string[] {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const path = join(directory, entry.name);
    if (entry.isDirectory()) return markdownFiles(path);
    return entry.isFile() && entry.name.endsWith('.md') ? [path] : [];
  });
}

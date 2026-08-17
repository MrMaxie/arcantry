import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { parse as parseYaml } from 'yaml';

export interface CatalogEntry {
  name: string;
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
}

export interface SkillAgent {
  defaultPrompt: string;
}

const skillNamePattern = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;

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
    interface?: { default_prompt?: unknown };
  };
  const defaultPrompt = source.interface?.default_prompt;
  if (typeof defaultPrompt !== 'string') {
    throw new Error(`skills/${name} agents/openai.yaml must declare interface.default_prompt`);
  }
  return { defaultPrompt };
}

export function validateCatalog(root = process.cwd()): string[] {
  const errors: string[] = [];
  const catalog = readCatalog(root);
  if (!Array.isArray(catalog.skills)) return ['catalog.json must contain a skills array'];

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
    if (!skillNamePattern.test(entry.name)) errors.push(`invalid skill name: ${entry.name}`);
    if (!Array.isArray(entry.tags) || entry.tags.length === 0 || new Set(entry.tags).size !== entry.tags.length) {
      errors.push(`skills/${entry.name} must have unique tags`);
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
    if (typeof metadata.summary !== 'string' || metadata.summary.length < 30 || metadata.summary.length > 180) {
      errors.push(`skills/${entry.name} summary must contain 30-180 characters`);
    }
    if (!Array.isArray(metadata.scenarios) || metadata.scenarios.length !== 2) {
      errors.push(`skills/${entry.name} must define exactly two scenarios`);
    } else {
      for (const [index, scenario] of metadata.scenarios.entries()) {
        if (!scenario.title || !scenario.prompt || !scenario.outcome) {
          errors.push(`skills/${entry.name} scenario ${index + 1} is incomplete`);
        }
      }
    }

    const agentPath = join(directory, 'agents', 'openai.yaml');
    if (existsSync(agentPath)) {
      try {
        const agent = readSkillAgent(root, entry.name);
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

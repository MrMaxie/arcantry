import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { parse as parseToml } from 'smol-toml';
import { afterEach, describe, expect, it } from 'vitest';
import { createFixtureDirectory, removeFixtures } from './testHelpers.js';
import {
  findNearestProjectConfig,
  parseProjectConfig,
  projectConfigSchemaLocation,
  renderProjectConfig,
  resolveProject,
} from './projectConfig.js';

afterEach(removeFixtures);

const configured = (extra = '') => `config_version = 1

[tool]
requires = ">=0.3.0 <1.0.0"

[sources.intent]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.history]
kind = "changelog"
path = "CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["intent"]
${extra}`;

describe('project configuration', () => {
  it('parses source relationships, compatibility and private visibility', () => {
    const config = parseProjectConfig(
      `${configured()}
[sources.todo_private]
kind = "todo-txt"
path = ".local/todo.txt"
adapter = "todo-txt@1"
`,
      { toolVersion: '0.3.2' },
    );

    expect(config.sources.history?.from).toEqual(['intent']);
    expect(config.sources.todo_private?.management).toBe('observe');
    expect(config.sources.todo_private?.visibility).toBe('private');
  });

  it('rejects cycles, missing semantic authorities and unsupported tools', () => {
    const cyclic = configured().replace('adapter = "openspec@1"', 'adapter = "openspec@1"\nfrom = ["history"]');
    expect(() => parseProjectConfig(cyclic)).toThrow('source dependency cycle');
    expect(() => parseProjectConfig(configured().replace('from = ["intent"]', 'from = []'))).toThrow(
      'managed changelog sources require',
    );
    expect(() => parseProjectConfig(configured(), { toolVersion: '2.0.0' })).toThrow('does not satisfy');
  });

  it('rejects unknown fields and nested managed OpenSpec authorities', () => {
    expect(() => parseProjectConfig(configured().replace('path = "openspec"', 'path = "openspec"\nmanagment = "manage"'))).toThrow(
      'Unrecognized key',
    );
    expect(() => parseProjectConfig(configured('managed_from = "next"'))).toThrow('full SemVer');
    expect(() => parseProjectConfig(`config_version = 1

[sources.workspace]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"
scope = "."

[sources.package]
kind = "openspec"
path = "packages/app/openspec"
management = "manage"
adapter = "openspec@1"
scope = "packages/app"
`)).toThrow('overlap');
  });

  it('allows absolute paths only for explicitly supplied external configuration', () => {
    const absolute = configured().replace('path = "openspec"', 'path = "C:/project/openspec"');
    expect(() => parseProjectConfig(absolute)).toThrow('Absolute source path requires');
    expect(parseProjectConfig(absolute, { allowAbsolutePaths: true }).sources.intent?.path).toBe('C:/project/openspec');
  });

  it('renders TOML Schema discovery metadata and round-trips', () => {
    const config = parseProjectConfig(configured());
    const rendered = renderProjectConfig(config);

    expect(rendered).toContain(`[toml-schema]`);
    expect(rendered).toContain(`location = "${projectConfigSchemaLocation}"`);
    expect(parseProjectConfig(rendered)).toEqual({
      ...config,
      schemaReference: { location: projectConfigSchemaLocation, version: '1.0.0' },
    });
  });

  it('publishes a TOML Schema 1.0.0 contract for dynamic source tables', async () => {
    const schema = parseToml(await readFile(new URL('../../../schemas/arcantry-config-v1.tosd', import.meta.url), 'utf8')) as Record<string, unknown>;
    const metadata = schema['toml-schema'] as { version: string };
    const elements = schema.elements as { sources: { type: string; itemtype: string } };

    expect(metadata.version).toBe('1.0.0');
    expect(elements.sources).toMatchObject({ type: 'collection', itemtype: 'types.source' });
  });

  it('uses one nearest config and resolves its declared project root', async () => {
    const workspace = await createFixtureDirectory('arcantry-config-');
    const project = join(workspace, 'project');
    const nested = join(project, 'apps', 'web');
    const configPath = join(project, 'config', 'arcantry.toml');
    await mkdir(nested, { recursive: true });
    await mkdir(dirname(configPath), { recursive: true });
    await writeFile(configPath, `${configured()}\n[project]\nroot = ".."\n`);

    expect(await findNearestProjectConfig(nested)).toBeNull();
    const resolved = await resolveProject({ cwd: nested, configPath, toolVersion: '0.3.2' });
    expect(resolved.root).toBe(project);
    expect(resolved.mode).toBe('configured');
  });

  it('allows absolute sources only when an explicit config is outside the resolved project', async () => {
    const project = await createFixtureDirectory('arcantry-internal-config-');
    const absolute = join(project, 'todo.txt').replaceAll('\\', '/');
    const configPath = join(project, 'arcantry.toml');
    await writeFile(configPath, `config_version = 1

[sources.tasks]
kind = "todo-txt"
path = "${absolute}"
adapter = "todo-txt@1"
`);

    await expect(resolveProject({ cwd: project, configPath })).rejects.toThrow('Absolute source path requires');
  });

  it('treats a project without configuration as a valid wild project', async () => {
    const root = await createFixtureDirectory('arcantry-wild-');
    await expect(resolveProject({ cwd: root, toolVersion: '0.3.2' })).resolves.toEqual({
      root,
      configPath: null,
      config: null,
      mode: 'wild',
    });
  });
});

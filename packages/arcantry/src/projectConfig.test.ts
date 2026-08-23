import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { parse as parseToml } from 'smol-toml';
import { afterEach, describe, expect, it } from 'vitest';
import { createFixtureDirectory, removeFixtures } from './testHelpers.js';
import {
  findNearestProjectConfig,
  isPrivateProjectPath,
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

  it.runIf(process.platform === 'win32')('treats .local paths case-insensitively on Windows', () => {
    expect(() => parseProjectConfig(`config_version = 1

[sources.tasks]
kind = "todo-txt"
path = ".LOCAL/todo.txt"
visibility = "shared"
adapter = "todo-txt@1"
`)).toThrow('inside .local');
  });

  it('classifies normalized .local paths as private', () => {
    expect(isPrivateProjectPath('public/../.local/todo.txt')).toBe(true);
    expect(() => parseProjectConfig(`config_version = 1

[sources.tasks]
kind = "todo-txt"
path = "public/../.local/todo.txt"
visibility = "shared"
adapter = "todo-txt@1"
`)).toThrow('inside .local');
  });

  it('rejects cycles, missing semantic authorities and unsupported tools', () => {
    const cyclic = configured().replace('adapter = "openspec@1"', 'adapter = "openspec@1"\nfrom = ["history"]');
    expect(() => parseProjectConfig(cyclic)).toThrow('source dependency cycle');
    expect(() => parseProjectConfig(configured().replace('from = ["intent"]', 'from = []'))).toThrow(
      'managed changelog sources require',
    );
    expect(() => parseProjectConfig(configured(), { toolVersion: '2.0.0' })).toThrow('does not satisfy');
  });

  it('keeps private release meaning private while allowing independent authorities', () => {
    expect(() => parseProjectConfig(`config_version = 1

[sources.private_intent]
kind = "openspec"
path = ".local/openspec"
management = "manage"
adapter = "openspec@1"

[sources.shared_history]
kind = "changelog"
path = "CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["private_intent"]
`)).toThrow('shared changelog sources cannot depend on private OpenSpec sources');

    expect(() => parseProjectConfig(`config_version = 1

[sources.shared_intent]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"

[sources.private_intent]
kind = "openspec"
path = ".local/openspec"
management = "manage"
adapter = "openspec@1"

[sources.private_history]
kind = "changelog"
path = ".local/CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["shared_intent", "private_intent"]
`)).not.toThrow();
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
    const absolutePath = resolve('project', 'openspec').replaceAll('\\', '/');
    const absolute = configured().replace('path = "openspec"', `path = "${absolutePath}"`);
    expect(() => parseProjectConfig(absolute)).toThrow('Absolute source path requires');
    expect(parseProjectConfig(absolute, { allowAbsolutePaths: true }).sources.intent?.path).toBe(absolutePath);
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

  it('parses and renders an explicit local release story', () => {
    const config = parseProjectConfig(`${configured()}
[release]
adapter = "openspec-release@1"
manifests_path = "releases"
changelog_source = "history"
tag_prefix = "v"
repository_url = "https://github.com/example/project"

[[release.version_sources]]
path = "Cargo.toml"
adapter = "cargo-workspace@1"
`);

    expect(config.release).toEqual({
      adapter: 'openspec-release@1',
      manifestsPath: 'releases',
      changelogSource: 'history',
      tagPrefix: 'v',
      repositoryUrl: 'https://github.com/example/project',
      versionSources: [{ path: 'Cargo.toml', adapter: 'cargo-workspace@1' }],
    });
    expect(parseProjectConfig(renderProjectConfig(config))).toEqual({
      ...config,
      schemaReference: { location: projectConfigSchemaLocation, version: '1.0.0' },
    });
  });

  it('defaults openspec-release@2 to the flat single topology', () => {
    const config = parseProjectConfig(`${configured()}
[release]
adapter = "openspec-release@2"
manifests_path = "releases"
changelog_source = "history"

[[release.version_sources]]
path = "package.json"
adapter = "json-package@1"
`);

    expect(config.release).toMatchObject({ adapter: 'openspec-release@2', topology: 'single' });
    expect(parseProjectConfig(renderProjectConfig(config)).release).toEqual(config.release);
  });

  it('parses a composed release-unit graph and round-trips it', () => {
    const config = parseProjectConfig(`${configured()}
[sources.core_history]
kind = "changelog"
path = "packages/core/CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["intent"]

[release]
adapter = "openspec-release@2"
topology = "composed"

[release.units.core]
manifests_path = "releases/core"
changelog_source = "core_history"
tag_prefix = "core/v"

[[release.units.core.version_sources]]
path = "packages/core/package.json"
adapter = "json-package@1"

[[release.units.core.selectors]]
source = "intent"
components = ["product:core"]

[release.units.app]
manifests_path = "releases/app"
changelog_source = "history"
tag_prefix = "app/v"
dependencies = ["core"]

[[release.units.app.version_sources]]
path = "apps/app/package.json"
adapter = "json-package@1"

[[release.units.app.selectors]]
source = "intent"
components = ["product:app"]
`);

    expect(config.release).toMatchObject({
      adapter: 'openspec-release@2',
      topology: 'composed',
      units: {
        core: { dependencies: [], selectors: [{ source: 'intent', components: ['product:core'] }] },
        app: { dependencies: ['core'], selectors: [{ source: 'intent', components: ['product:app'] }] },
      },
    });
    expect(parseProjectConfig(renderProjectConfig(config)).release).toEqual(config.release);
  });

  it('rejects invalid multi-unit release ownership and dependency graphs', () => {
    const multi = `${configured()}
[release]
adapter = "openspec-release@2"
topology = "TOPOLOGY"

[release.units.core]
manifests_path = "releases/shared"
changelog_source = "history"
tag_prefix = "shared/v"
dependencies = DEPENDENCIES

[[release.units.core.version_sources]]
path = "package.json"
adapter = "json-package@1"

[[release.units.core.selectors]]
source = "intent"
components = ["product:core"]

[release.units.app]
manifests_path = "releases/shared"
changelog_source = "history"
tag_prefix = "shared/v"
dependencies = ["core"]

[[release.units.app.version_sources]]
path = "apps/app/package.json"
adapter = "json-package@1"

[[release.units.app.selectors]]
source = "intent"
components = ["product:core"]
`;
    expect(() => parseProjectConfig(multi.replace('TOPOLOGY', 'independent').replace('DEPENDENCIES', '["app"]'))).toThrow('cannot declare dependencies');
    expect(() => parseProjectConfig(multi.replace('TOPOLOGY', 'composed').replace('DEPENDENCIES', '["app"]'))).toThrow(/unique across release units|ownership overlaps|dependency cycle/);
    expect(() => parseProjectConfig(multi.replace('TOPOLOGY', 'composed').replace('DEPENDENCIES', '[]').replace('dependencies = ["core"]', 'dependencies = []'))).toThrow('requires at least one dependency edge');
  });

  it('rejects a release block without a managed changelog authority', () => {
    expect(() => parseProjectConfig(`config_version = 1

[release]
adapter = "openspec-release@1"
manifests_path = "releases"
changelog_source = "missing"

[[release.version_sources]]
path = "package.json"
adapter = "json-package@1"
`)).toThrow('unknown release changelog source');
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

  it('prefers private configuration at the nearest project boundary', async () => {
    const project = await createFixtureDirectory('arcantry-local-config-');
    const nested = join(project, 'apps', 'web');
    await mkdir(join(project, '.local'), { recursive: true });
    await mkdir(nested, { recursive: true });
    await writeFile(join(project, 'arcantry.toml'), 'config_version = 1\n');
    await writeFile(join(project, '.local/arcantry.toml'), 'config_version = 1\n');

    const resolved = await resolveProject({ cwd: nested });

    expect(resolved.root).toBe(project);
    expect(resolved.scope).toBe('private');
    expect(resolved.configPath).toBe(join(project, '.local/arcantry.toml'));
    expect(resolved.shadowedConfigPaths).toEqual([join(project, 'arcantry.toml')]);
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

  it('rejects project-local paths that escape the project root', () => {
    expect(() => parseProjectConfig(`config_version = 1

[sources.tasks]
kind = "todo-txt"
path = "queue/../../todo.txt"
adapter = "todo-txt@1"
`)).toThrow('Source tasks path must stay within the project');

    expect(() => parseProjectConfig(`config_version = 1

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

[release]
adapter = "openspec-release@1"
manifests_path = "../releases"
changelog_source = "history"
tag_prefix = "v"

[[release.version_sources]]
path = "package.json"
adapter = "json-package@1"
`)).toThrow('Release manifests path must stay within the project');

    expect(() => parseProjectConfig(`config_version = 1

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

[release]
adapter = "openspec-release@1"
manifests_path = "releases"
changelog_source = "history"
tag_prefix = "v"

[[release.version_sources]]
path = "../package.json"
adapter = "json-package@1"
`)).toThrow('Release version source path must stay within the project');
  });

  it('treats a project without configuration as a valid wild project', async () => {
    const root = await createFixtureDirectory('arcantry-wild-');
    await expect(resolveProject({ cwd: root, toolVersion: '0.3.2' })).resolves.toEqual({
      root,
      configPath: null,
      config: null,
      mode: 'wild',
      scope: null,
      shadowedConfigPaths: [],
    });
  });
});

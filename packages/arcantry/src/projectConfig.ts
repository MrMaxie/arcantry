import { readFile, stat } from 'node:fs/promises';
import { basename, dirname, isAbsolute, join, parse as parsePath, relative, resolve } from 'node:path';
import { satisfies, valid, validRange } from 'semver';
import { parse, stringify } from 'smol-toml';
import { z } from 'zod';

export const projectConfigVersion = 1 as const;
export const projectConfigFilename = 'arcantry.toml';
export const privateProjectConfigPath = join('.local', projectConfigFilename);
export const projectConfigSchemaLocation =
  'https://mrmaxie.github.io/arcantry/schemas/arcantry-config-v1.tosd';

export const managementSchema = z.enum(['ignore', 'observe', 'validate', 'manage']);
export const sourceKindSchema = z.enum(['openspec', 'changelog', 'todo-txt']);
export const transitionSchema = z.enum(['preserve', 'adopt', 'rebind', 'cutover', 'migrate', 'relocate']);
export const visibilitySchema = z.enum(['shared', 'private']);
export const releaseVersionAdapterSchema = z.enum(['json-package@1', 'cargo-workspace@1']);

const adapterPattern = /^([a-z][a-z0-9-]*)@([0-9]+)$/;
const sourceIdPattern = /^[A-Za-z0-9][A-Za-z0-9._-]*$/;

const schemaReferenceSchema = z.object({
  location: z.string().trim().min(1),
  version: z.string().refine((value) => valid(value) !== null, 'TOML Schema version must be full SemVer.').optional(),
}).strict();

const sourceConfigSchema = z.object({
  kind: sourceKindSchema,
  path: z.string().trim().min(1),
  management: managementSchema.default('observe'),
  adapter: z.string().regex(adapterPattern, 'adapter must use <name>@<integer-version>.'),
  from: z.array(z.string().trim().min(1)).default([]),
  managed_from: z.string().trim().min(1).optional(),
  visibility: visibilitySchema.optional(),
  scope: z.string().trim().min(1).default('.'),
}).strict();

const releaseVersionSourceSchema = z.object({
  path: z.string().trim().min(1),
  adapter: releaseVersionAdapterSchema,
}).strict();

const releaseV1ConfigSchema = z.object({
  adapter: z.literal('openspec-release@1'),
  manifests_path: z.string().trim().min(1),
  changelog_source: z.string().trim().min(1),
  tag_prefix: z.string().min(1).default('v'),
  repository_url: z.string().url().optional(),
  version_sources: z.array(releaseVersionSourceSchema).min(1),
}).strict();

const releaseUnitSelectorSchema = z.object({
  source: z.string().trim().min(1),
  components: z.array(z.string().regex(/^[a-z0-9]+(?:-[a-z0-9]+)*(?::[a-z0-9]+(?:-[a-z0-9]+)*)?$/)).min(1).refine((components) => new Set(components).size === components.length, 'release selector components must be unique.').optional(),
}).strict();

const releaseUnitConfigSchema = z.object({
  manifests_path: z.string().trim().min(1),
  changelog_source: z.string().trim().min(1),
  tag_prefix: z.string().min(1),
  dependencies: z.array(z.string().trim().min(1)).refine((dependencies) => new Set(dependencies).size === dependencies.length, 'release unit dependencies must be unique.').default([]),
  version_sources: z.array(releaseVersionSourceSchema).min(1),
  selectors: z.array(releaseUnitSelectorSchema).min(1),
}).strict();

const releaseV2SingleConfigSchema = z.object({
  adapter: z.literal('openspec-release@2'),
  topology: z.literal('single').default('single'),
  manifests_path: z.string().trim().min(1),
  changelog_source: z.string().trim().min(1),
  tag_prefix: z.string().min(1).default('v'),
  repository_url: z.string().url().optional(),
  version_sources: z.array(releaseVersionSourceSchema).min(1),
}).strict();

const releaseV2MultiConfigSchema = z.object({
  adapter: z.literal('openspec-release@2'),
  topology: z.enum(['independent', 'composed']),
  repository_url: z.string().url().optional(),
  units: z.record(z.string(), releaseUnitConfigSchema).refine((units) => Object.keys(units).length > 0, 'multi-unit release configuration requires at least one unit.'),
}).strict();

const releaseConfigSchema = z.union([releaseV1ConfigSchema, releaseV2SingleConfigSchema, releaseV2MultiConfigSchema]);

const rawProjectConfigSchema = z
  .object({
    config_version: z.literal(projectConfigVersion),
    'toml-schema': schemaReferenceSchema.optional(),
    tool: z
      .object({
        requires: z.string().trim().min(1).refine((value) => validRange(value) !== null, 'tool.requires must be a valid SemVer range.'),
      }).strict()
      .optional(),
    project: z.object({ root: z.string().trim().min(1) }).strict().optional(),
    sources: z.record(z.string(), sourceConfigSchema).default({}),
    release: releaseConfigSchema.optional(),
  })
  .strict()
  .superRefine((config, context) => {
    const effectiveVisibility = (source: z.infer<typeof sourceConfigSchema>): Visibility =>
      source.visibility ?? (isPrivateProjectPath(source.path) ? 'private' : 'shared');

    for (const [id, source] of Object.entries(config.sources)) {
      if (!sourceIdPattern.test(id)) {
        context.addIssue({ code: 'custom', message: 'source ids may contain letters, numbers, dot, underscore and hyphen.', path: ['sources', id] });
      }
      for (const dependency of source.from) {
        if (!(dependency in config.sources)) {
          context.addIssue({ code: 'custom', message: `unknown source dependency: ${dependency}`, path: ['sources', id, 'from'] });
        }
      }
      if (source.kind === 'changelog') {
        if (source.management === 'manage' && source.from.length === 0) {
          context.addIssue({ code: 'custom', message: 'managed changelog sources require at least one OpenSpec source.', path: ['sources', id, 'from'] });
        }
        for (const dependency of source.from) {
          if (config.sources[dependency]?.kind !== 'openspec') {
            context.addIssue({ code: 'custom', message: 'managed changelog dependencies must be OpenSpec sources.', path: ['sources', id, 'from'] });
          } else if (effectiveVisibility(source) === 'shared' && effectiveVisibility(config.sources[dependency]) === 'private') {
            context.addIssue({ code: 'custom', message: 'shared changelog sources cannot depend on private OpenSpec sources.', path: ['sources', id, 'from'] });
          }
        }
      }
      if (source.managed_from !== undefined && (source.kind !== 'changelog' || valid(source.managed_from) === null)) {
        context.addIssue({
          code: 'custom',
          message: 'managed_from requires a changelog source and a full SemVer version.',
          path: ['sources', id, 'managed_from'],
        });
      }
    }

    const visiting = new Set<string>();
    const visited = new Set<string>();
    const visit = (id: string): void => {
      if (visiting.has(id)) {
        context.addIssue({ code: 'custom', message: `source dependency cycle includes ${id}.`, path: ['sources', id, 'from'] });
        return;
      }
      if (visited.has(id)) return;
      visiting.add(id);
      for (const dependency of config.sources[id]?.from ?? []) visit(dependency);
      visiting.delete(id);
      visited.add(id);
    };
    for (const id of Object.keys(config.sources)) visit(id);

    if (config.release !== undefined) validateRawReleaseConfig(config, context);

    const managedOpenSpec = Object.entries(config.sources).filter(
      ([, source]) => source.kind === 'openspec' && source.management === 'manage',
    );
    for (const [index, [id, source]] of managedOpenSpec.entries()) {
      const conflicting = managedOpenSpec.slice(index + 1).find(([, candidate]) =>
        effectiveVisibility(candidate) === effectiveVisibility(source) && scopesOverlap(candidate.scope, source.scope));
      if (conflicting !== undefined) {
        context.addIssue({
          code: 'custom',
          message: `managed OpenSpec sources ${id} and ${conflicting[0]} overlap at scope ${source.scope}.`,
          path: ['sources', id, 'scope'],
        });
      }
    }
  });

export type Management = z.infer<typeof managementSchema>;
export type SourceKind = z.infer<typeof sourceKindSchema>;
export type Transition = z.infer<typeof transitionSchema>;
export type Visibility = z.infer<typeof visibilitySchema>;
export type ReleaseVersionAdapter = z.infer<typeof releaseVersionAdapterSchema>;

export type ProjectSourceConfig = {
  id: string;
  kind: SourceKind;
  path: string;
  management: Management;
  adapter: string;
  from: string[];
  managedFrom?: string;
  visibility: Visibility;
  scope: string;
};

export type ProjectConfig = {
  configVersion: typeof projectConfigVersion;
  schemaReference?: { location: string; version?: string };
  tool?: { requires: string };
  project?: { root: string };
  sources: Record<string, ProjectSourceConfig>;
  release?: ProjectReleaseConfig;
};

export type ReleaseTopology = 'single' | 'independent' | 'composed';
export type ReleaseUnitSelector = { source: string; components?: string[] };
export type ReleaseUnitConfig = {
  manifestsPath: string;
  changelogSource: string;
  tagPrefix: string;
  dependencies: string[];
  versionSources: Array<{ path: string; adapter: ReleaseVersionAdapter }>;
  selectors: ReleaseUnitSelector[];
};
export type ProjectReleaseConfigV1 = {
  adapter: 'openspec-release@1';
  manifestsPath: string;
  changelogSource: string;
  tagPrefix: string;
  repositoryUrl?: string;
  versionSources: Array<{ path: string; adapter: ReleaseVersionAdapter }>;
};
export type ProjectReleaseConfigV2Single = Omit<ProjectReleaseConfigV1, 'adapter'> & {
  adapter: 'openspec-release@2';
  topology: 'single';
};
export type ProjectReleaseConfigV2Multi = {
  adapter: 'openspec-release@2';
  topology: 'independent' | 'composed';
  repositoryUrl?: string;
  units: Record<string, ReleaseUnitConfig>;
};
export type ProjectReleaseConfig = ProjectReleaseConfigV1 | ProjectReleaseConfigV2Single | ProjectReleaseConfigV2Multi;
export type ReleaseSystem = ProjectReleaseConfig;

export type ResolvedProject = {
  root: string;
  configPath: string | null;
  config: ProjectConfig | null;
  mode: 'configured' | 'wild';
  scope: 'shared' | 'private' | 'external' | null;
  shadowedConfigPaths: string[];
};

export const parseAdapter = (adapter: string): { name: string; version: number } => {
  const match = adapter.match(adapterPattern);
  if (match === null) throw new Error(`Invalid adapter: ${adapter}`);
  return { name: match[1]!, version: Number.parseInt(match[2]!, 10) };
};

export const parseProjectConfig = (
  content: string,
  options: { toolVersion?: string; allowAbsolutePaths?: boolean } = {},
): ProjectConfig => {
  const parsed = rawProjectConfigSchema.parse(parse(content));
  if (options.toolVersion !== undefined && parsed.tool !== undefined && !satisfies(options.toolVersion, parsed.tool.requires)) {
    throw new Error(`Arcantry ${options.toolVersion} does not satisfy configured range ${parsed.tool.requires}.`);
  }

  const sources = Object.fromEntries(
    Object.entries(parsed.sources).map(([id, source]) => {
      if (isAbsolute(source.path) && options.allowAbsolutePaths !== true) {
        throw new Error(`Absolute source path requires an explicit external configuration: ${id}.`);
      }
      if (pathEscapesProject(source.path)) throw new Error(`Source ${id} path must stay within the project.`);
      const pathVisibility = isPrivateProjectPath(source.path) ? 'private' : 'shared';
      if (pathVisibility === 'private' && source.visibility === 'shared') {
        throw new Error(`Source ${id} is inside .local and cannot be shared.`);
      }
      return [
        id,
        {
          id,
          kind: source.kind,
          path: source.path,
          management: source.management,
          adapter: source.adapter,
          from: source.from,
          ...(source.managed_from === undefined ? {} : { managedFrom: source.managed_from }),
          visibility: source.visibility ?? pathVisibility,
          scope: source.scope,
        } satisfies ProjectSourceConfig,
      ];
    }),
  );

  if (parsed.release !== undefined) {
    const releasePaths = 'units' in parsed.release
      ? Object.values(parsed.release.units).flatMap((unit) => [
          ['release manifests', unit.manifests_path] as const,
          ...unit.version_sources.map((source) => ['release version source', source.path] as const),
        ])
      : [
          ['release manifests', parsed.release.manifests_path] as const,
          ...parsed.release.version_sources.map((source) => ['release version source', source.path] as const),
        ];
    for (const [label, path] of releasePaths) {
      if (isAbsolute(path) && options.allowAbsolutePaths !== true) {
        throw new Error(`Absolute ${label} path requires an explicit external configuration.`);
      }
      if (pathEscapesProject(path)) throw new Error(`${label[0]!.toUpperCase()}${label.slice(1)} path must stay within the project.`);
    }
  }

  return {
    configVersion: parsed.config_version,
    ...(parsed['toml-schema'] === undefined ? {} : { schemaReference: parsed['toml-schema'] }),
    ...(parsed.tool === undefined ? {} : { tool: parsed.tool }),
    ...(parsed.project === undefined ? {} : { project: parsed.project }),
    sources,
    ...(parsed.release === undefined ? {} : { release: normalizeReleaseConfig(parsed.release) }),
  };
};

export const renderProjectConfig = (config: ProjectConfig): string =>
  stringify({
    config_version: config.configVersion,
    'toml-schema': config.schemaReference ?? { location: projectConfigSchemaLocation, version: '1.0.0' },
    ...(config.tool === undefined ? {} : { tool: config.tool }),
    ...(config.project === undefined ? {} : { project: config.project }),
    sources: Object.fromEntries(
      Object.entries(config.sources).map(([id, source]) => [
        id,
        {
          kind: source.kind,
          path: source.path,
          management: source.management,
          adapter: source.adapter,
          ...(source.from.length === 0 ? {} : { from: source.from }),
          ...(source.managedFrom === undefined ? {} : { managed_from: source.managedFrom }),
          ...(source.visibility === (isPrivateProjectPath(source.path) ? 'private' : 'shared') ? {} : { visibility: source.visibility }),
          ...(source.scope === '.' ? {} : { scope: source.scope }),
        },
      ]),
    ),
    ...(config.release === undefined ? {} : { release: denormalizeReleaseConfig(config.release) }),
  });

type RawProjectConfig = z.infer<typeof rawProjectConfigSchema>;
type RawReleaseConfig = NonNullable<RawProjectConfig['release']>;

function validateRawReleaseConfig(config: RawProjectConfig, context: z.RefinementCtx): void {
  const release = config.release!;
  const validateChangelog = (sourceId: string, path: PropertyKey[]): void => {
    const changelog = config.sources[sourceId];
    if (changelog === undefined) {
      context.addIssue({ code: 'custom', message: `unknown release changelog source: ${sourceId}`, path });
    } else if (changelog.kind !== 'changelog' || changelog.management !== 'manage') {
      context.addIssue({ code: 'custom', message: 'release changelog source must be a managed changelog source.', path });
    }
  };
  if (!('units' in release)) {
    validateChangelog(release.changelog_source, ['release', 'changelog_source']);
    return;
  }

  const unitIds = Object.keys(release.units);
  const ownedPaths = new Map<string, string>();
  const ownedChangelogs = new Map<string, string>();
  const ownedPrefixes = new Map<string, string>();
  const wholeSources = new Map<string, string>();
  const componentOwners = new Map<string, string>();
  let edgeCount = 0;
  for (const [unitId, unit] of Object.entries(release.units)) {
    if (!sourceIdPattern.test(unitId)) context.addIssue({ code: 'custom', message: 'release unit ids may contain letters, numbers, dot, underscore and hyphen.', path: ['release', 'units', unitId] });
    validateChangelog(unit.changelog_source, ['release', 'units', unitId, 'changelog_source']);
    claimUnique(ownedPaths, unit.manifests_path, unitId, 'release manifests path', context, ['release', 'units', unitId, 'manifests_path']);
    claimUnique(ownedChangelogs, unit.changelog_source, unitId, 'release changelog source', context, ['release', 'units', unitId, 'changelog_source']);
    claimUnique(ownedPrefixes, unit.tag_prefix, unitId, 'release tag prefix', context, ['release', 'units', unitId, 'tag_prefix']);
    for (const source of unit.version_sources) claimUnique(ownedPaths, source.path, unitId, 'release-owned path', context, ['release', 'units', unitId, 'version_sources']);
    if (release.topology === 'independent' && unit.dependencies.length > 0) context.addIssue({ code: 'custom', message: 'independent release units cannot declare dependencies.', path: ['release', 'units', unitId, 'dependencies'] });
    edgeCount += unit.dependencies.length;
    for (const dependency of unit.dependencies) if (!unitIds.includes(dependency)) context.addIssue({ code: 'custom', message: `unknown release unit dependency: ${dependency}`, path: ['release', 'units', unitId, 'dependencies'] });
    for (const selector of unit.selectors) {
      const source = config.sources[selector.source];
      if (source === undefined || source.kind !== 'openspec') context.addIssue({ code: 'custom', message: `release selector source must be configured OpenSpec: ${selector.source}`, path: ['release', 'units', unitId, 'selectors'] });
      const changelog = config.sources[unit.changelog_source];
      if (changelog !== undefined && !changelog.from.includes(selector.source)) context.addIssue({ code: 'custom', message: `release selector source must be an authority of ${unit.changelog_source}: ${selector.source}`, path: ['release', 'units', unitId, 'selectors'] });
      if (selector.components === undefined) {
        const owner = wholeSources.get(selector.source) ?? [...componentOwners.keys()].find((key) => key.startsWith(`${selector.source}\0`));
        if (owner !== undefined) context.addIssue({ code: 'custom', message: `release selector ownership overlaps for source ${selector.source}.`, path: ['release', 'units', unitId, 'selectors'] });
        wholeSources.set(selector.source, unitId);
      } else {
        if (wholeSources.has(selector.source)) context.addIssue({ code: 'custom', message: `release selector ownership overlaps for source ${selector.source}.`, path: ['release', 'units', unitId, 'selectors'] });
        for (const component of selector.components) claimUnique(componentOwners, `${selector.source}\0${component}`, unitId, 'release selector component', context, ['release', 'units', unitId, 'selectors']);
      }
    }
  }
  if (release.topology === 'composed') {
    if (edgeCount === 0) context.addIssue({ code: 'custom', message: 'composed release topology requires at least one dependency edge.', path: ['release', 'topology'] });
    validateUnitCycles(release.units, context);
  }
}

function claimUnique(owners: Map<string, string>, value: string, unitId: string, label: string, context: z.RefinementCtx, path: PropertyKey[]): void {
  const owner = owners.get(value);
  if (owner !== undefined) context.addIssue({ code: 'custom', message: `${label} must be unique across release units: ${value}`, path });
  owners.set(value, unitId);
}

function validateUnitCycles(units: Record<string, z.infer<typeof releaseUnitConfigSchema>>, context: z.RefinementCtx): void {
  const visiting = new Set<string>();
  const visited = new Set<string>();
  const visit = (id: string): void => {
    if (visiting.has(id)) {
      context.addIssue({ code: 'custom', message: `release unit dependency cycle includes ${id}.`, path: ['release', 'units', id, 'dependencies'] });
      return;
    }
    if (visited.has(id)) return;
    visiting.add(id);
    for (const dependency of units[id]?.dependencies ?? []) if (dependency in units) visit(dependency);
    visiting.delete(id);
    visited.add(id);
  };
  for (const id of Object.keys(units)) visit(id);
}

function normalizeReleaseConfig(release: RawReleaseConfig): ProjectReleaseConfig {
  if ('units' in release) return {
    adapter: release.adapter,
    topology: release.topology,
    ...(release.repository_url === undefined ? {} : { repositoryUrl: release.repository_url }),
    units: Object.fromEntries(Object.entries(release.units).map(([id, unit]) => [id, {
      manifestsPath: unit.manifests_path,
      changelogSource: unit.changelog_source,
      tagPrefix: unit.tag_prefix,
      dependencies: unit.dependencies,
      versionSources: unit.version_sources,
      selectors: unit.selectors,
    }])),
  };
  return {
    adapter: release.adapter,
    ...('topology' in release ? { topology: release.topology } : {}),
    manifestsPath: release.manifests_path,
    changelogSource: release.changelog_source,
    tagPrefix: release.tag_prefix,
    ...(release.repository_url === undefined ? {} : { repositoryUrl: release.repository_url }),
    versionSources: release.version_sources,
  } as ProjectReleaseConfig;
}

function denormalizeReleaseConfig(release: ProjectReleaseConfig): Record<string, unknown> {
  if ('units' in release) return {
    adapter: release.adapter,
    topology: release.topology,
    ...(release.repositoryUrl === undefined ? {} : { repository_url: release.repositoryUrl }),
    units: Object.fromEntries(Object.entries(release.units).map(([id, unit]) => [id, {
      manifests_path: unit.manifestsPath,
      changelog_source: unit.changelogSource,
      tag_prefix: unit.tagPrefix,
      ...(unit.dependencies.length === 0 ? {} : { dependencies: unit.dependencies }),
      version_sources: unit.versionSources,
      selectors: unit.selectors,
    }])),
  };
  return {
    adapter: release.adapter,
    ...('topology' in release ? { topology: release.topology } : {}),
    manifests_path: release.manifestsPath,
    changelog_source: release.changelogSource,
    tag_prefix: release.tagPrefix,
    ...(release.repositoryUrl === undefined ? {} : { repository_url: release.repositoryUrl }),
    version_sources: release.versionSources,
  };
}

type DiscoveredProjectConfig = { active: string | null; shadowed: string[] };

const discoverProjectConfig = async (start: string): Promise<DiscoveredProjectConfig> => {
  let directory = resolve(start);
  while (true) {
    const privateCandidate = join(directory, privateProjectConfigPath);
    const sharedCandidate = join(directory, projectConfigFilename);
    const [hasPrivate, hasShared] = await Promise.all([isFile(privateCandidate), isFile(sharedCandidate)]);
    if (hasPrivate || hasShared) {
      return {
        active: hasPrivate ? privateCandidate : sharedCandidate,
        shadowed: hasPrivate && hasShared ? [sharedCandidate] : [],
      };
    }
    const parent = dirname(directory);
    if (parent === directory || parsePath(directory).root === directory) return { active: null, shadowed: [] };
    directory = parent;
  }
};

export const findNearestProjectConfig = async (start: string): Promise<string | null> =>
  (await discoverProjectConfig(start)).active;

export const resolveProject = async (options: {
  cwd: string;
  configPath?: string;
  cwdExplicit?: boolean;
  toolVersion?: string;
}): Promise<ResolvedProject> => {
  const cwd = resolve(options.cwd);
  const explicitConfig = options.configPath === undefined ? null : resolve(cwd, options.configPath);
  const discovered = await discoverProjectConfig(cwd);
  const configPath = explicitConfig ?? discovered.active;
  if (configPath === null) {
    return { root: cwd, configPath: null, config: null, mode: 'wild', scope: null, shadowedConfigPaths: [] };
  }

  const content = await readFile(configPath, 'utf8');
  let config = parseProjectConfig(content, {
    toolVersion: options.toolVersion,
    allowAbsolutePaths: explicitConfig !== null,
  });
  const scope = explicitConfig !== null && configPath !== discovered.active
    ? 'external'
    : isPrivateConfigPath(configPath)
      ? 'private'
      : 'shared';
  const configRoot = scope === 'private' ? dirname(dirname(configPath)) : dirname(configPath);
  const root = options.cwdExplicit === true
    ? cwd
    : config.project !== undefined
      ? resolve(configRoot, config.project.root)
      : explicitConfig !== null
        ? cwd
        : configRoot;
  if (explicitConfig !== null && isWithin(root, configPath)) {
    config = parseProjectConfig(content, { toolVersion: options.toolVersion, allowAbsolutePaths: false });
  }
  const shadowedConfigPaths = explicitConfig === null
    ? discovered.shadowed
    : [discovered.active, ...discovered.shadowed].filter(
        (path): path is string => path !== null && resolve(path) !== resolve(configPath),
      );
  return { root, configPath, config, mode: 'configured', scope, shadowedConfigPaths };
};

const scopesOverlap = (left: string, right: string): boolean => {
  const normalize = (value: string): string => {
    const normalized = value.replaceAll('\\', '/').replace(/^\.\//, '').replace(/^\/$|\/$/g, '');
    return normalized === '.' ? '' : normalized;
  };
  const a = normalize(left);
  const b = normalize(right);
  return a === '' || b === '' || a === b || a.startsWith(`${b}/`) || b.startsWith(`${a}/`);
};

const pathEscapesProject = (path: string): boolean => {
  let depth = 0;
  for (const segment of path.replaceAll('\\', '/').split('/')) {
    if (segment === '' || segment === '.') continue;
    if (segment === '..') {
      if (depth === 0) return true;
      depth -= 1;
    } else {
      depth += 1;
    }
  }
  return false;
};

const isWithin = (parent: string, child: string): boolean => {
  const value = relative(resolve(parent), resolve(child));
  return value === '' || (!value.startsWith('..') && !isAbsolute(value));
};

export const isPrivateProjectPath = (path: string): boolean => {
  const segments: string[] = [];
  for (const segment of path.replaceAll('\\', '/').split('/')) {
    if (segment === '' || segment === '.') continue;
    if (segment === '..') {
      if (segments.length > 0 && segments.at(-1) !== '..') segments.pop();
      else segments.push(segment);
    } else segments.push(segment);
  }
  const [first = ''] = segments;
  return process.platform === 'win32' ? first.toLowerCase() === '.local' : first === '.local';
};

const isPrivateConfigPath = (path: string): boolean =>
  basename(dirname(resolve(path))).toLowerCase() === '.local' && basename(path).toLowerCase() === projectConfigFilename;

const isFile = async (path: string): Promise<boolean> => {
  try {
    return (await stat(path)).isFile();
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') return false;
    throw error;
  }
};

export const relativeProjectPath = (root: string, path: string): string => relative(root, path).replaceAll('\\', '/');

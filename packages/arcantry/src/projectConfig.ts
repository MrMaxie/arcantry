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
};

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

  return {
    configVersion: parsed.config_version,
    ...(parsed['toml-schema'] === undefined ? {} : { schemaReference: parsed['toml-schema'] }),
    ...(parsed.tool === undefined ? {} : { tool: parsed.tool }),
    ...(parsed.project === undefined ? {} : { project: parsed.project }),
    sources,
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
  });

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

const isWithin = (parent: string, child: string): boolean => {
  const value = relative(resolve(parent), resolve(child));
  return value === '' || (!value.startsWith('..') && !isAbsolute(value));
};

const isPrivateProjectPath = (path: string): boolean => {
  const normalized = path.replaceAll('\\', '/').replace(/^\.\//, '');
  return normalized === '.local' || normalized.startsWith('.local/');
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

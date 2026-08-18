import { readFile, stat } from 'node:fs/promises';
import { isAbsolute, join, resolve } from 'node:path';
import {
  type Management,
  type ProjectSourceConfig,
  type ResolvedProject,
  type SourceKind,
  type Visibility,
  parseAdapter,
  relativeProjectPath,
} from './projectConfig.js';
import { inspectChangelog } from './changelog.js';

export type AdapterDescriptor = {
  id: string;
  kind: SourceKind;
  read: boolean;
  write: boolean;
  format: string;
};

export type AdapterStatus = 'supported' | 'unsupported' | 'wrong-kind';
export type DetectionConfidence = 'high' | 'medium' | 'low';

export type ProjectSource = ProjectSourceConfig & {
  absolutePath: string;
  exists: boolean;
  origin: 'configured' | 'discovered';
  confidence: DetectionConfidence;
  adapterStatus: AdapterStatus;
};

export type KnowledgeInspection = {
  root: string;
  mode: ResolvedProject['mode'];
  configPath: string | null;
  configScope: ResolvedProject['scope'];
  shadowedConfigPaths: string[];
  sources: ProjectSource[];
  diagnostics: string[];
};

export type KnowledgeDiagnostic = {
  severity: 'error' | 'warning';
  sourceId: string;
  message: string;
  repair?: string;
};

export type KnowledgeValidation = {
  valid: boolean;
  diagnostics: KnowledgeDiagnostic[];
};

const adapterDescriptors: AdapterDescriptor[] = [
    { id: 'openspec@1', kind: 'openspec', read: true, write: true, format: 'OpenSpec' },
    { id: 'keep-a-changelog@1', kind: 'changelog', read: true, write: true, format: 'Keep a Changelog 1.x' },
    { id: 'keep-a-changelog@2', kind: 'changelog', read: true, write: true, format: 'Keep a Changelog 2.x' },
    { id: 'todo-txt@1', kind: 'todo-txt', read: true, write: true, format: 'todo.txt' },
];

export const builtInAdapters: ReadonlyMap<string, AdapterDescriptor> = new Map(
  adapterDescriptors.map((adapter) => [adapter.id, adapter] as const),
);

const candidates: Array<{
  id: string;
  kind: SourceKind;
  path: string;
  visibility: Visibility;
  expected: 'file' | 'directory';
}> = [
  { id: 'openspec', kind: 'openspec', path: 'openspec', visibility: 'shared', expected: 'directory' },
  { id: 'changelog', kind: 'changelog', path: 'CHANGELOG.md', visibility: 'shared', expected: 'file' },
  { id: 'todo-root', kind: 'todo-txt', path: 'todo.txt', visibility: 'shared', expected: 'file' },
  { id: 'todo-local', kind: 'todo-txt', path: '.local/todo.txt', visibility: 'private', expected: 'file' },
];

export const inspectKnowledge = async (project: ResolvedProject): Promise<KnowledgeInspection> => {
  const configured = Object.values(project.config?.sources ?? {});
  const sources: ProjectSource[] = [];
  const diagnostics: string[] = [];
  const configuredPaths = new Set<string>();

  for (const source of configured) {
    const absolutePath = resolveSourcePath(project.root, source.path);
    configuredPaths.add(normalizePath(absolutePath));
    const exists = await pathExists(absolutePath);
    const inspected = finalizeSource(source, absolutePath, exists, 'configured', 'high');
    sources.push(inspected);
    appendAdapterDiagnostic(inspected, diagnostics);
  }

  for (const candidate of candidates) {
    const absolutePath = join(project.root, candidate.path);
    if (configuredPaths.has(normalizePath(absolutePath))) continue;
    const kind = await pathKind(absolutePath);
    if (kind !== candidate.expected) continue;
    const detected = await detectAdapter(candidate.kind, absolutePath);
    const source: ProjectSourceConfig = {
      id: uniqueId(candidate.id, sources),
      kind: candidate.kind,
      path: candidate.path,
      management: 'observe',
      adapter: detected.adapter,
      from: [],
      visibility: candidate.visibility,
      scope: '.',
    };
    const inspected = finalizeSource(source, absolutePath, true, 'discovered', detected.confidence);
    sources.push(inspected);
    appendAdapterDiagnostic(inspected, diagnostics);
  }

  return {
    root: project.root,
    mode: project.mode,
    configPath: project.configPath,
    configScope: project.scope,
    shadowedConfigPaths: project.shadowedConfigPaths,
    sources: sources.sort((left, right) => left.id.localeCompare(right.id)),
    diagnostics,
  };
};

export const adapterFor = (source: Pick<ProjectSourceConfig, 'adapter' | 'kind'>): AdapterDescriptor | null => {
  const descriptor = builtInAdapters.get(source.adapter) ?? null;
  return descriptor?.kind === source.kind ? descriptor : null;
};

export const validateKnowledge = async (inspection: KnowledgeInspection, doctor = false): Promise<KnowledgeValidation> => {
  const diagnostics: KnowledgeDiagnostic[] = [];
  for (const source of inspection.sources) {
    if (source.management === 'ignore') continue;
    if (source.adapterStatus !== 'supported') {
      diagnostics.push({
        severity: source.management === 'observe' ? 'warning' : 'error',
        sourceId: source.id,
        message: source.adapterStatus === 'wrong-kind'
          ? `Adapter ${source.adapter} does not support ${source.kind}.`
          : `Adapter ${source.adapter} is not supported by this Arcantry version.`,
        ...(doctor ? { repair: 'Use a supported adapter or a compatible Arcantry version.' } : {}),
      });
      continue;
    }
    if (!source.exists) {
      if (source.management === 'validate' || source.management === 'manage') {
        diagnostics.push({
          severity: 'error',
          sourceId: source.id,
          message: `Configured source is missing at ${source.path}.`,
          ...(doctor ? { repair: `Plan adoption for source ${source.id}.` } : {}),
        });
      }
      continue;
    }
    if (source.management === 'observe') continue;

    if (source.kind === 'openspec' && !(await pathExists(join(source.absolutePath, 'config.yaml')))) {
      diagnostics.push({
        severity: 'error',
        sourceId: source.id,
        message: 'OpenSpec config.yaml is missing.',
        ...(doctor ? { repair: `Repair or re-adopt source ${source.id}.` } : {}),
      });
    }
    if (source.kind === 'changelog') {
      const report = inspectChangelog(await readFile(source.absolutePath, 'utf8'));
      const expectedFormat = source.adapter === 'keep-a-changelog@2' ? 'keep-a-changelog-2' : 'keep-a-changelog-1';
      if (report.format !== expectedFormat) {
        diagnostics.push({
          severity: 'error',
          sourceId: source.id,
          message: `Changelog content does not match ${source.adapter}.`,
          ...(doctor ? { repair: `Plan a cutover or migration for source ${source.id}.` } : {}),
        });
      }
      for (const message of report.diagnostics) {
        diagnostics.push({ severity: 'error', sourceId: source.id, message });
      }
    }
  }
  return { valid: diagnostics.every((diagnostic) => diagnostic.severity !== 'error'), diagnostics };
};

export const adapterStatus = (source: Pick<ProjectSourceConfig, 'adapter' | 'kind'>): AdapterStatus => {
  const descriptor = builtInAdapters.get(source.adapter);
  if (descriptor === undefined) return 'unsupported';
  return descriptor.kind === source.kind ? 'supported' : 'wrong-kind';
};

const finalizeSource = (
  source: ProjectSourceConfig,
  absolutePath: string,
  exists: boolean,
  origin: ProjectSource['origin'],
  confidence: DetectionConfidence,
): ProjectSource => ({
  ...source,
  absolutePath,
  exists,
  origin,
  confidence,
  adapterStatus: adapterStatus(source),
});

const appendAdapterDiagnostic = (source: ProjectSource, diagnostics: string[]): void => {
  if (source.adapterStatus === 'unsupported') {
    diagnostics.push(`Source ${source.id} requires unsupported adapter ${source.adapter}.`);
  } else if (source.adapterStatus === 'wrong-kind') {
    diagnostics.push(`Adapter ${source.adapter} cannot handle ${source.kind} source ${source.id}.`);
  }
  if (!source.exists && source.management !== 'ignore' && source.management !== 'observe') {
    diagnostics.push(`Configured ${source.management} source ${source.id} is missing at ${source.path}.`);
  }
};

const detectAdapter = async (
  kind: SourceKind,
  path: string,
): Promise<{ adapter: string; confidence: DetectionConfidence }> => {
  if (kind === 'openspec') return { adapter: 'openspec@1', confidence: 'high' };
  if (kind === 'todo-txt') return { adapter: 'todo-txt@1', confidence: 'high' };

  const content = (await readFile(path, 'utf8')).replace(/^\uFEFF/, '');
  if (content.includes('keepachangelog.com/en/2.0.0')) return { adapter: 'keep-a-changelog@2', confidence: 'high' };
  if (content.includes('keepachangelog.com/en/1.')) return { adapter: 'keep-a-changelog@1', confidence: 'high' };
  if (/^## \[(?:Unreleased|\d+\.\d+\.\d+)\]/m.test(content)) {
    return { adapter: 'keep-a-changelog@1', confidence: 'medium' };
  }
  return { adapter: 'changelog@0', confidence: 'low' };
};

const resolveSourcePath = (root: string, path: string): string => (isAbsolute(path) ? resolve(path) : resolve(root, path));

const uniqueId = (candidate: string, sources: ProjectSource[]): string => {
  if (!sources.some((source) => source.id === candidate)) return candidate;
  let suffix = 2;
  while (sources.some((source) => source.id === `${candidate}-${suffix}`)) suffix += 1;
  return `${candidate}-${suffix}`;
};

const normalizePath = (path: string): string => resolve(path).toLowerCase();

const pathExists = async (path: string): Promise<boolean> => (await pathKind(path)) !== 'missing';

const pathKind = async (path: string): Promise<'file' | 'directory' | 'missing'> => {
  try {
    const value = await stat(path);
    if (value.isDirectory()) return 'directory';
    return 'file';
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') return 'missing';
    throw error;
  }
};

export const displaySourcePath = (inspection: Pick<KnowledgeInspection, 'root'>, source: ProjectSource): string =>
  isAbsolute(source.path) ? source.path : relativeProjectPath(inspection.root, source.absolutePath);

export const canWriteSource = (source: ProjectSource): boolean =>
  source.management === 'manage' && adapterFor(source)?.write === true;

export const sourceManagementOrder: readonly Management[] = ['ignore', 'observe', 'validate', 'manage'];

export const adapterFamily = (source: ProjectSourceConfig): { name: string; version: number } => parseAdapter(source.adapter);

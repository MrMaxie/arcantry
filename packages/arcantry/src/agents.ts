import { lstat, readFile } from 'node:fs/promises';
import { join } from 'node:path';
import type { AgentName } from './config.js';

export type AgentArtifact = {
  agent: AgentName;
  path: string;
  kind: 'markdown-section' | 'cursor-rule';
  detectionPaths: string[];
};

export type AgentDetection = {
  agent: AgentName;
  detected: boolean;
  reasons: string[];
};

export const agentArtifacts: readonly AgentArtifact[] = [
  {
    agent: 'codex',
    path: 'AGENTS.md',
    kind: 'markdown-section',
    detectionPaths: ['AGENTS.md', '.codex'],
  },
  {
    agent: 'claude',
    path: 'CLAUDE.md',
    kind: 'markdown-section',
    detectionPaths: ['CLAUDE.md', '.claude'],
  },
  {
    agent: 'cursor',
    path: '.cursor/rules/arcantry.mdc',
    kind: 'cursor-rule',
    detectionPaths: ['.cursor', '.cursorrules'],
  },
] as const;

const pathExists = async (path: string): Promise<boolean> => {
  try {
    await lstat(path);
    return true;
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') {
      return false;
    }
    throw error;
  }
};

export const detectAgents = async (root: string): Promise<AgentDetection[]> =>
  Promise.all(
    agentArtifacts.map(async (artifact) => {
      const matches = await Promise.all(
        artifact.detectionPaths.map(async (path) => ({ path, exists: await pathExists(join(root, path)) })),
      );
      return {
        agent: artifact.agent,
        detected: matches.some((match) => match.exists),
        reasons: matches.filter((match) => match.exists).map((match) => match.path),
      };
    }),
  );

export const getAgentArtifact = (agent: AgentName): AgentArtifact => {
  const artifact = agentArtifacts.find((candidate) => candidate.agent === agent);
  if (artifact === undefined) {
    throw new Error(`Unsupported agent: ${agent}`);
  }
  return artifact;
};

export const readAgentEntrypoint = async (root: string, agent: AgentName): Promise<string | null> => {
  const path = join(root, getAgentArtifact(agent).path);
  try {
    return await readFile(path, 'utf8');
  } catch (error) {
    if (error instanceof Error && 'code' in error && error.code === 'ENOENT') {
      return null;
    }
    throw error;
  }
};

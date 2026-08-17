import { z } from 'zod';

export const arcantryConfigSchemaVersion = 1 as const;

export const agentNameSchema = z.enum(['codex', 'claude', 'cursor']);
export const sourceModeSchema = z.enum(['readonly', 'readwrite', 'operational']);
export const docsModeSchema = z.enum(['shared', 'local', 'none']);

export const sourceSchema = z.object({
  name: z.string().trim().min(1),
  mode: sourceModeSchema,
});

export const arcantryConfigSchema = z
  .object({
    schemaVersion: z.literal(arcantryConfigSchemaVersion),
    agents: z.array(agentNameSchema).min(1),
    operationalSource: z.string().trim().min(1),
    sources: z.array(sourceSchema).min(1),
    docs: docsModeSchema,
  })
  .superRefine((config, context) => {
    const uniqueAgents = new Set(config.agents);
    if (uniqueAgents.size !== config.agents.length) {
      context.addIssue({ code: 'custom', message: 'agents must not contain duplicates.', path: ['agents'] });
    }

    const sourceNames = new Set<string>();
    for (const [index, source] of config.sources.entries()) {
      if (sourceNames.has(source.name)) {
        context.addIssue({ code: 'custom', message: `Duplicate source: ${source.name}`, path: ['sources', index, 'name'] });
      }
      sourceNames.add(source.name);
    }

    const operationalSources = config.sources.filter((source) => source.mode === 'operational');
    if (operationalSources.length !== 1) {
      context.addIssue({ code: 'custom', message: 'sources must contain exactly one operational source.', path: ['sources'] });
    }
    if (operationalSources[0]?.name !== config.operationalSource) {
      context.addIssue({
        code: 'custom',
        message: 'operationalSource must name the source whose mode is operational.',
        path: ['operationalSource'],
      });
    }
  });

export type AgentName = z.infer<typeof agentNameSchema>;
export type SourceMode = z.infer<typeof sourceModeSchema>;
export type ArcantrySource = z.infer<typeof sourceSchema>;
export type DocsMode = z.infer<typeof docsModeSchema>;
export type ArcantryConfig = z.infer<typeof arcantryConfigSchema>;

export type CreateArcantryConfigInput = {
  agents?: AgentName[];
  operationalSource?: string;
  sources?: ArcantrySource[];
  docs: DocsMode;
};

export const createArcantryConfig = (input: CreateArcantryConfigInput): ArcantryConfig => {
  const operationalSource = input.operationalSource ?? 'local';
  const sources = input.sources ?? [{ name: operationalSource, mode: 'operational' as const }];

  return arcantryConfigSchema.parse({
    schemaVersion: arcantryConfigSchemaVersion,
    agents: input.agents ?? ['codex'],
    operationalSource,
    sources,
    docs: input.docs,
  });
};

export const renderArcantryConfig = (config: ArcantryConfig): string => `${JSON.stringify(arcantryConfigSchema.parse(config), null, 2)}\n`;

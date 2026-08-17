import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { Command } from 'commander';
import { ZodError } from 'zod';
import { doctorSkills, findCatalogRoot, inspectSkill, linkSkill, loadCatalog, unlinkSkill } from './catalog.js';
import { type AgentName, type ArcantrySource, agentNameSchema, docsModeSchema, sourceModeSchema } from './config.js';
import {
  doctorRepository,
  initRepository,
  removeRepository,
  updateRepository,
  validateRepository,
  type RepositoryReport,
  type RepositoryResult,
} from './repository.js';

const packageManifest = JSON.parse(
  readFileSync(new URL('../package.json', import.meta.url), 'utf8'),
) as { version: string };

export const arcantryVersion = packageManifest.version;

export type OutputWriter = (message: string) => void;
export type CliContext = { cwd: string; stdout: OutputWriter; stderr: OutputWriter };
export type CliResult = { exitCode: number };

const exitCodes = new WeakMap<Command, number>();

export const buildProgram = (context: CliContext): Command => {
  const program = new Command();
  program
    .name('arcantry')
    .description('Manage Arcantry repository adoption and skill links.')
    .version(arcantryVersion)
    .option('--cwd <path>', 'Run against another repository or catalog location.')
    .showHelpAfterError()
    .exitOverride()
    .configureOutput({
      writeOut: (message) => context.stdout(message),
      writeErr: (message) => context.stderr(message),
      outputError: (message, write) => write(message),
    });

  const repo = program.command('repo').description('Manage repository adoption.');
  repo
    .command('init')
    .description('Initialize Arcantry-managed repository artifacts.')
    .requiredOption('--docs <mode>', 'Documentation mode: shared, local, or none.')
    .option('--agent <agent>', 'Agent entrypoint to manage; repeat to select more than one.', collectAgent, [])
    .option('--source <name=mode>', 'Ordered source and mode; repeat to select more than one.', collectSource, [])
    .option('--operational-source <name>', 'Name of the single operational source.')
    .action(async (options: { docs: string; agent: AgentName[]; source: ArcantrySource[]; operationalSource?: string }) => {
      await execute(program, context, async () => {
        const sources = options.source.length > 0 ? options.source : undefined;
        const inferredOperationalSource = sources?.find((source) => source.mode === 'operational')?.name;
        const result = await initRepository(commandCwd(program, context), {
          docs: docsModeSchema.parse(options.docs),
          agents: options.agent.length > 0 ? options.agent : undefined,
          sources,
          operationalSource: options.operationalSource ?? inferredOperationalSource,
        });
        renderRepositoryResult(context, result);
      });
    });

  repo
    .command('update')
    .description('Refresh owned artifacts while preserving configuration.')
    .action(async () => execute(program, context, async () => renderRepositoryResult(context, await updateRepository(commandCwd(program, context)))));

  repo
    .command('doctor')
    .description('Diagnose repository adoption without changing it.')
    .action(async () => execute(program, context, async () => renderRepositoryReport(program, context, await doctorRepository(commandCwd(program, context)))));

  repo
    .command('validate')
    .description('Validate repository adoption without changing it.')
    .action(async () => execute(program, context, async () => renderRepositoryReport(program, context, await validateRepository(commandCwd(program, context)))));

  repo
    .command('remove')
    .description('Remove only verified Arcantry-owned artifacts and sections.')
    .action(async () => execute(program, context, async () => renderRepositoryResult(context, await removeRepository(commandCwd(program, context)))));

  const skills = program.command('skills').description('Inspect the catalog and manage skill links.');
  skills
    .command('list')
    .description('List skills in the canonical catalog.')
    .option('--catalog-root <path>', 'Directory containing catalog.json and skills/.')
    .action(async (options: { catalogRoot?: string }) => {
      await execute(program, context, async () => {
        const root = await catalogRoot(program, context, options.catalogRoot);
        const catalog = await loadCatalog(root);
        for (const entry of catalog.skills) context.stdout(`${entry.name}\t${entry.tags.join(', ')}\n`);
      });
    });

  skills
    .command('inspect <name>')
    .description('Show public metadata for one skill.')
    .option('--catalog-root <path>', 'Directory containing catalog.json and skills/.')
    .action(async (name: string, options: { catalogRoot?: string }) => {
      await execute(program, context, async () => {
        const inspection = await inspectSkill(await catalogRoot(program, context, options.catalogRoot), name);
        context.stdout(`${inspection.entry.name}\n${inspection.metadata.summary}\nTags: ${inspection.entry.tags.join(', ')}\n`);
        for (const scenario of inspection.metadata.scenarios) context.stdout(`- ${scenario.title}: ${scenario.outcome}\n`);
      });
    });

  skills
    .command('link <name>')
    .description('Link one catalog skill into an agent skill directory.')
    .option('--catalog-root <path>', 'Directory containing catalog.json and skills/.')
    .option('--target <path>', 'Agent skill directory.')
    .option('--replace', 'Back up an ordinary target or replace a different link before linking.')
    .action(async (name: string, options: { catalogRoot?: string; target?: string; replace?: boolean }) => {
      await execute(program, context, async () => {
        const result = await linkSkill({
          catalogRoot: await catalogRoot(program, context, options.catalogRoot),
          name,
          targetRoot: options.target,
          replace: options.replace,
        });
        context.stdout(result.status === 'unchanged' ? `Already linked: ${name}\n` : `Linked: ${name}\n`);
        if (result.backup !== null) context.stdout(`Backup: ${result.backup}\n`);
      });
    });

  skills
    .command('unlink <name>')
    .description('Remove only an exact link to one catalog skill.')
    .option('--catalog-root <path>', 'Directory containing catalog.json and skills/.')
    .option('--target <path>', 'Agent skill directory.')
    .action(async (name: string, options: { catalogRoot?: string; target?: string }) => {
      await execute(program, context, async () => {
        await unlinkSkill({ catalogRoot: await catalogRoot(program, context, options.catalogRoot), name, targetRoot: options.target });
        context.stdout(`Unlinked: ${name}\n`);
      });
    });

  skills
    .command('doctor')
    .description('Validate catalog packages and optionally inspect a skill link directory.')
    .option('--catalog-root <path>', 'Directory containing catalog.json and skills/.')
    .option('--target <path>', 'Agent skill directory to inspect.')
    .action(async (options: { catalogRoot?: string; target?: string }) => {
      await execute(program, context, async () => {
        const report = await doctorSkills(await catalogRoot(program, context, options.catalogRoot), options.target);
        for (const error of report.errors) context.stderr(`ERROR: ${error}\n`);
        for (const warning of report.warnings) context.stdout(`WARNING: ${warning}\n`);
        if (report.valid) context.stdout('Skill catalog is valid.\n');
        else exitCodes.set(program, 1);
      });
    });

  return program;
};

export const runCli = async (argv = process.argv, options: Partial<CliContext> = {}): Promise<CliResult> => {
  const context: CliContext = {
    cwd: options.cwd ?? process.cwd(),
    stdout: options.stdout ?? ((message) => process.stdout.write(message)),
    stderr: options.stderr ?? ((message) => process.stderr.write(message)),
  };
  const program = buildProgram(context);
  try {
    await program.parseAsync(argv);
  } catch (error) {
    if (isCommanderExit(error)) return { exitCode: error.exitCode };
    throw error;
  }
  return { exitCode: exitCodes.get(program) ?? 0 };
};

const execute = async (program: Command, context: CliContext, action: () => Promise<void>): Promise<void> => {
  try {
    await action();
  } catch (error) {
    context.stderr(`Error: ${formatError(error)}\n`);
    exitCodes.set(program, 1);
  }
};

const commandCwd = (program: Command, context: CliContext): string => resolve(program.opts<{ cwd?: string }>().cwd ?? context.cwd);

const catalogRoot = async (program: Command, context: CliContext, explicit?: string): Promise<string> =>
  explicit === undefined ? findCatalogRoot(commandCwd(program, context)) : resolve(explicit);

const renderRepositoryResult = (context: CliContext, result: RepositoryResult): void => {
  if (result.applied.length === 0) {
    context.stdout('No changes required.\n');
    return;
  }
  for (const change of result.applied) {
    const path = resolve(change.path) === change.path ? '.git/info/exclude' : change.path;
    context.stdout(`${change.action}: ${path}\n`);
  }
};

const renderRepositoryReport = (program: Command, context: CliContext, report: RepositoryReport): void => {
  for (const diagnostic of report.diagnostics) {
    const write = diagnostic.severity === 'error' ? context.stderr : context.stdout;
    write(`${diagnostic.severity.toUpperCase()}: ${diagnostic.path}: ${diagnostic.message}\n`);
  }
  if (report.valid) context.stdout('Repository adoption is valid.\n');
  else exitCodes.set(program, 1);
};

const collectAgent = (value: string, previous: AgentName[]): AgentName[] => [...previous, agentNameSchema.parse(value)];

const collectSource = (value: string, previous: ArcantrySource[]): ArcantrySource[] => {
  const parts = value.split('=');
  if (parts.length !== 2 || parts[0]?.trim() === '' || parts[1]?.trim() === '') {
    throw new Error('--source must use <name=readonly|readwrite|operational>.');
  }
  return [...previous, { name: parts[0].trim(), mode: sourceModeSchema.parse(parts[1].trim()) }];
};

const formatError = (error: unknown): string => {
  if (error instanceof ZodError) return error.issues.map((issue) => issue.message).join(' ');
  return error instanceof Error ? error.message : String(error);
};

const isCommanderExit = (error: unknown): error is { exitCode: number } =>
  typeof error === 'object' && error !== null && 'exitCode' in error && typeof (error as { exitCode?: unknown }).exitCode === 'number';

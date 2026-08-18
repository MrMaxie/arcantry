import { readFileSync } from 'node:fs';
import { readFile } from 'node:fs/promises';
import { relative, resolve } from 'node:path';
import { Command } from 'commander';
import { ZodError } from 'zod';
import {
  doctorPrivateSkills,
  doctorSkills,
  findCatalogRoot,
  inspectPrivateSkill,
  inspectSkill,
  linkSkillTargets,
  listPrivateSkills,
  loadCatalog,
  privateSkillExists,
  repositoryClaudeSkillTargetRoot,
  repositorySkillTargetRoot,
  rollbackSkillLinks,
  unlinkSkillTargets,
  userClaudeSkillTargetRoot,
  userSkillTargetRoot,
  type SkillLinkOptions,
} from './catalog.js';
import {
  doctorRepository,
  initRepository,
  removeRepository,
  repositoryCompatibilitySchema,
  repositoryScopeSchema,
  resolveRepositoryRoot,
  updateRepository,
  validateRepository,
  type RepositoryReport,
  type RepositoryResult,
} from './repository.js';
import { displaySourcePath, inspectKnowledge, validateKnowledge, type KnowledgeInspection } from './knowledge.js';
import { applyProjectPlan, createProjectPlan, createWriteOperation, parseProjectPlan, renderProjectPlan, serializeProjectPlan, type PlanOperation } from './projectPlan.js';
import { resolveProject, transitionSchema } from './projectConfig.js';
import { planSourceTransition } from './sourceTransition.js';
import { addTodoTask, completeTodoTask, inspectTodoTasks, moveTodoTask } from './todoTxt.js';
import { planLocalGitExclude, planLocalGitExcludeEntries } from './privateState.js';
import type { ProjectSource } from './knowledge.js';

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
    .option('--config <path>', 'Use one explicit arcantry.toml file without config merging.')
    .showHelpAfterError()
    .exitOverride()
    .configureOutput({
      writeOut: (message) => context.stdout(message),
      writeErr: (message) => context.stderr(message),
      outputError: (message, write) => write(message),
    });

  const repo = program.command('repo').description('Manage repository adoption.');
  repo
    .command('inspect')
    .description('Discover project knowledge sources without changing them.')
    .option('--json', 'Write the complete machine-readable inspection.')
    .action(async (options: { json?: boolean }) => {
      await execute(program, context, async () => {
        const inspection = await inspectKnowledge(await commandProject(program, context));
        if (options.json === true) context.stdout(`${JSON.stringify(inspection, null, 2)}\n`);
        else renderKnowledgeInspection(context, inspection);
      });
    });

  repo
    .command('plan')
    .description('Plan one explicit source transition without changing the project.')
    .requiredOption('--source <id>', 'Source id reported by repo inspect.')
    .requiredOption('--transition <transition>', 'Transition: preserve, adopt, rebind, cutover, migrate, or relocate.')
    .option('--to-path <path>', 'Target source path for rebind or relocate.')
    .option('--to-adapter <adapter>', 'Target versioned adapter.')
    .option('--managed-from <version>', 'First managed SemVer version for changelog cutover.')
    .option('--delete-source', 'Delete the verified source after relocation.')
    .option('--json', 'Write the serializable plan required by repo apply.')
    .action(async (options: {
      source: string;
      transition: string;
      toPath?: string;
      toAdapter?: string;
      managedFrom?: string;
      deleteSource?: boolean;
      json?: boolean;
    }) => {
      await execute(program, context, async () => {
        const inspection = await inspectKnowledge(await commandProject(program, context));
        const plan = await planSourceTransition(inspection, {
          sourceId: options.source,
          transition: transitionSchema.parse(options.transition),
          toolVersion: arcantryVersion,
          ...(options.toPath === undefined ? {} : { targetPath: options.toPath }),
          ...(options.toAdapter === undefined ? {} : { targetAdapter: options.toAdapter }),
          ...(options.managedFrom === undefined ? {} : { managedFrom: options.managedFrom }),
          deleteSource: options.deleteSource === true,
        });
        if (options.json === true) {
          context.stdout(serializeProjectPlan(plan));
          if (plan.conflicts.length > 0) exitCodes.set(program, 1);
        } else renderPlanResult(program, context, plan);
      });
    });

  repo
    .command('apply')
    .description('Apply an unchanged serialized transition plan.')
    .requiredOption('--plan <path>', 'Plan JSON file, or - for standard input.')
    .action(async (options: { plan: string }) => {
      await execute(program, context, async () => {
        const content = options.plan === '-' ? await readStandardInput() : await readFile(resolve(commandCwd(program, context), options.plan), 'utf8');
        const result = await applyProjectPlan(parseProjectPlan(content), arcantryVersion);
        if (result.applied.length === 0) context.stdout('No file changes.\n');
        else for (const operation of result.applied) context.stdout(`${operation.action}: ${operation.path}\n`);
      });
    });

  repo
    .command('init')
    .description('Initialize minimal shared or private Arcantry repository state.')
    .requiredOption('--scope <scope>', 'Configuration scope: shared or private.')
    .option('--compat <compatibility>', 'Add an explicit compatibility adapter: claude.')
    .action(async (options: { scope: string; compat?: string }) => {
      await execute(program, context, async () => {
        const result = await initRepository(
          commandCwd(program, context),
          repositoryScopeSchema.parse(options.scope),
          parseCompatibility(options.compat),
        );
        renderRepositoryResult(context, result);
      });
    });

  repo
    .command('update')
    .description('Refresh owned artifacts while preserving configuration.')
    .requiredOption('--scope <scope>', 'Configuration scope: shared or private.')
    .option('--compat <compatibility>', 'Add or refresh an explicit compatibility adapter: claude.')
    .action(async (options: { scope: string; compat?: string }) => execute(program, context, async () =>
      renderRepositoryResult(context, await updateRepository(
        commandCwd(program, context),
        repositoryScopeSchema.parse(options.scope),
        parseCompatibility(options.compat),
      ))));

  repo
    .command('doctor')
    .description('Diagnose repository adoption without changing it.')
    .action(async () => execute(program, context, async () => {
      renderRepositoryReport(program, context, await doctorRepository(commandCwd(program, context), commandConfigPath(program, context)));
      await renderKnowledgeValidation(program, context, true);
    }));

  repo
    .command('validate')
    .description('Validate repository adoption without changing it.')
    .action(async () => execute(program, context, async () => {
      renderRepositoryReport(program, context, await validateRepository(commandCwd(program, context), false, commandConfigPath(program, context)));
      await renderKnowledgeValidation(program, context, false);
    }));

  repo
    .command('remove')
    .description('Remove only verified Arcantry-owned artifacts and sections.')
    .requiredOption('--scope <scope>', 'Configuration scope: shared or private.')
    .action(async (options: { scope: string }) => execute(program, context, async () =>
      renderRepositoryResult(context, await removeRepository(commandCwd(program, context), repositoryScopeSchema.parse(options.scope)))));

  const todo = program.command('todo').description('Use root or private todo.txt queues without imposing a workflow taxonomy.');
  todo
    .command('list')
    .description('List todo.txt tasks from one or all detected queues.')
    .option('--source <id>', 'Todo source id, root, or local.')
    .action(async (options: { source?: string }) => {
      await execute(program, context, async () => {
        const inspection = await inspectKnowledge(await commandProject(program, context));
        const sources = options.source === undefined
          ? inspection.sources.filter((source) => source.kind === 'todo-txt' && source.exists)
          : [resolveTodoSource(inspection, options.source, false)];
        if (sources.length === 0) context.stdout('No todo.txt tasks.\n');
        for (const source of sources) {
          context.stdout(`[${source.id}]\n`);
          for (const task of inspectTodoTasks(await readFile(source.absolutePath, 'utf8'))) {
            context.stdout(`${task.line}\t${task.raw}\n`);
          }
        }
      });
    });

  todo
    .command('add <task>')
    .description('Preview or apply one todo.txt task addition.')
    .option('--source <id>', 'Todo source id, root, or local.')
    .option('--apply', 'Apply the previewed file change.')
    .action(async (task: string, options: { source?: string; apply?: boolean }) => {
      await execute(program, context, async () => {
        const inspection = await inspectKnowledge(await commandProject(program, context));
        const source = selectTodoSource(inspection, options.source, true);
        const current = source.exists ? await readFile(source.absolutePath, 'utf8') : '';
        const operations = [await createWriteOperation(inspection.root, source.path, addTodoTask(current, task), source.visibility)];
        if (source.visibility === 'private') await planLocalGitExclude(inspection.root, operations);
        await handleTodoPlan(program, context, createProjectPlan({
          toolVersion: arcantryVersion,
          root: inspection.root,
          sourceId: source.id,
          transition: 'adopt',
          adapter: 'todo-txt@1',
          operations,
        }), options.apply === true);
      });
    });

  todo
    .command('complete <line>')
    .description('Preview or apply completion of one todo.txt line.')
    .option('--source <id>', 'Todo source id, root, or local.')
    .option('--date <date>', 'Completion date in YYYY-MM-DD.')
    .option('--apply', 'Apply the previewed file change.')
    .action(async (line: string, options: { source?: string; date?: string; apply?: boolean }) => {
      await execute(program, context, async () => {
        const inspection = await inspectKnowledge(await commandProject(program, context));
        const source = selectTodoSource(inspection, options.source, false);
        const current = await readFile(source.absolutePath, 'utf8');
        const desired = completeTodoTask(current, parseLine(line), options.date ?? localDate());
        await handleTodoPlan(program, context, createProjectPlan({
          toolVersion: arcantryVersion,
          root: inspection.root,
          sourceId: source.id,
          transition: 'adopt',
          adapter: 'todo-txt@1',
          operations: desired === current ? [] : [await createWriteOperation(inspection.root, source.path, desired, source.visibility)],
        }), options.apply === true);
      });
    });

  todo
    .command('move <line>')
    .description('Preview or apply an explicit move between todo.txt queues.')
    .requiredOption('--from <id>', 'Source todo id, root, or local.')
    .requiredOption('--to <id>', 'Target todo id, root, or local.')
    .option('--apply', 'Apply the previewed file changes.')
    .action(async (line: string, options: { from: string; to: string; apply?: boolean }) => {
      await execute(program, context, async () => {
        const inspection = await inspectKnowledge(await commandProject(program, context));
        const source = resolveTodoSource(inspection, options.from, false);
        const target = resolveTodoSource(inspection, options.to, true);
        if (source.id === target.id) throw new Error('Todo move source and target must differ.');
        const moved = moveTodoTask(
          await readFile(source.absolutePath, 'utf8'),
          target.exists ? await readFile(target.absolutePath, 'utf8') : '',
          parseLine(line),
        );
        const operations = [
          await createWriteOperation(inspection.root, source.path, moved.source, source.visibility),
          await createWriteOperation(inspection.root, target.path, moved.target, target.visibility),
        ];
        if (target.visibility === 'private') await planLocalGitExclude(inspection.root, operations);
        await handleTodoPlan(program, context, createProjectPlan({
          toolVersion: arcantryVersion,
          root: inspection.root,
          sourceId: source.id,
          transition: 'relocate',
          adapter: 'todo-txt@1',
          operations,
        }), options.apply === true);
      });
    });

  const skills = program.command('skills').description('Inspect the catalog and manage skill links.');
  skills
    .command('list')
    .description('List public catalog skills or private repository skills.')
    .option('--catalog-root <path>', 'Directory containing catalog.json and skills/.')
    .option('--scope <scope>', 'Inventory scope: public or private.', 'public')
    .action(async (options: { catalogRoot?: string; scope: string }) => {
      await execute(program, context, async () => {
        if (options.scope === 'private') {
          const root = await resolveRepositoryRoot(commandCwd(program, context));
          for (const skill of await listPrivateSkills(root)) context.stdout(`${skill.name}\tprivate\t${skill.description}\n`);
          return;
        }
        if (options.scope !== 'public') throw new Error('--scope must be public or private.');
        const root = await catalogRoot(program, context, options.catalogRoot);
        const catalog = await loadCatalog(root);
        for (const entry of catalog.skills) context.stdout(`${entry.name}\t${entry.family}\t${entry.tags.join(', ')}\n`);
      });
    });

  skills
    .command('inspect <name>')
    .description('Show metadata for one public or private skill.')
    .option('--catalog-root <path>', 'Directory containing catalog.json and skills/.')
    .option('--scope <scope>', 'Inventory scope: public or private.', 'public')
    .action(async (name: string, options: { catalogRoot?: string; scope: string }) => {
      await execute(program, context, async () => {
        if (options.scope === 'private') {
          const inspection = await inspectPrivateSkill(await resolveRepositoryRoot(commandCwd(program, context)), name);
          context.stdout(`${inspection.name}\n${inspection.description}\nVisibility: private\nSource: ${inspection.directory}\n`);
          return;
        }
        if (options.scope !== 'public') throw new Error('--scope must be public or private.');
        const inspection = await inspectSkill(await catalogRoot(program, context, options.catalogRoot), name);
        context.stdout(`${inspection.entry.name}\n${inspection.metadata.summary}\nFamily: ${inspection.entry.family}\nTags: ${inspection.entry.tags.join(', ')}\n`);
        for (const scenario of inspection.metadata.scenarios) context.stdout(`- ${scenario.title}: ${scenario.outcome}\n`);
      });
    });

  skills
    .command('link <name>')
    .description('Link one canonical skill into the universal Agent Skills directory.')
    .option('--catalog-root <path>', 'Directory containing catalog.json and skills/.')
    .option('--scope <scope>', 'Skill scope: user, repo, or private.')
    .option('--compat <compatibility>', 'Also add a compatibility alias: claude.')
    .option('--target <path>', 'Advanced explicit Agent Skills directory.')
    .option('--replace', 'Back up an ordinary target or replace a different link before linking.')
    .action(async (name: string, options: SkillCommandOptions) => {
      await execute(program, context, async () => {
        const operation = await resolveSkillOperation(program, context, name, options);
        const results = await linkSkillTargets({ ...operation.source, name, replace: options.replace }, operation.targetRoots);
        try {
          if (operation.privateRoot !== null) await excludePrivateSkillLinks(operation.privateRoot, name, operation.targetRoots);
        } catch (error) {
          await rollbackSkillLinks(results);
          throw error;
        }
        context.stdout(results.every((result) => result.status === 'unchanged') ? `Already linked: ${name}\n` : `Linked: ${name}\n`);
        for (const result of results) if (result.backup !== null) context.stdout(`Backup: ${result.backup}\n`);
      });
    });

  skills
    .command('unlink <name>')
    .description('Remove only an exact link to one catalog skill.')
    .option('--catalog-root <path>', 'Directory containing catalog.json and skills/.')
    .option('--scope <scope>', 'Skill scope: user, repo, or private.')
    .option('--compat <compatibility>', 'Also remove the compatibility alias: claude.')
    .option('--target <path>', 'Advanced explicit Agent Skills directory.')
    .action(async (name: string, options: SkillCommandOptions) => {
      await execute(program, context, async () => {
        const operation = await resolveSkillOperation(program, context, name, options);
        const results = await unlinkSkillTargets({ ...operation.source, name }, operation.targetRoots);
        context.stdout(results.every((result) => result.status === 'unchanged') ? `Already unlinked: ${name}\n` : `Unlinked: ${name}\n`);
      });
    });

  skills
    .command('doctor')
    .description('Validate skill packages and optionally inspect universal and compatibility links.')
    .option('--catalog-root <path>', 'Directory containing catalog.json and skills/.')
    .option('--scope <scope>', 'Skill scope to inspect: user, repo, or private.')
    .option('--compat <compatibility>', 'Also inspect a compatibility alias: claude.')
    .option('--target <path>', 'Advanced explicit Agent Skills directory to inspect.')
    .action(async (options: SkillCommandOptions) => {
      await execute(program, context, async () => {
        let report: Awaited<ReturnType<typeof doctorSkills>>;
        if (options.scope === 'private') {
          const root = await resolveRepositoryRoot(commandCwd(program, context));
          const targetRoots = await resolveSkillTargetRoots(program, context, options);
          const publicCatalog = await loadCatalog(await catalogRoot(program, context, options.catalogRoot));
          report = await doctorPrivateSkills(root, targetRoots, new Set(publicCatalog.skills.map((entry) => entry.name)));
        } else {
          const targetRoots = options.scope === undefined && options.compat === undefined && options.target === undefined
            ? undefined
            : await resolveSkillTargetRoots(program, context, options);
          report = await doctorSkills(await catalogRoot(program, context, options.catalogRoot), targetRoots);
        }
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

const commandConfigPath = (program: Command, context: CliContext): string | undefined => {
  const config = program.opts<{ config?: string }>().config;
  return config === undefined ? undefined : resolve(commandCwd(program, context), config);
};

const commandProject = async (program: Command, context: CliContext) => {
  const options = program.opts<{ cwd?: string; config?: string }>();
  return resolveProject({
    cwd: resolve(options.cwd ?? context.cwd),
    ...(options.config === undefined ? {} : { configPath: options.config }),
    cwdExplicit: options.cwd !== undefined,
    toolVersion: arcantryVersion,
  });
};

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
    if (diagnostic.repair !== undefined) write(`Repair: ${diagnostic.repair}\n`);
  }
  if (report.valid) context.stdout('Repository adoption is valid.\n');
  else exitCodes.set(program, 1);
};

const renderKnowledgeInspection = (context: CliContext, inspection: KnowledgeInspection): void => {
  context.stdout(`Mode: ${inspection.mode}\n`);
  context.stdout(`Config: ${inspection.configPath === null ? 'none' : `${inspection.configScope ?? 'external'} (${inspection.configPath})`}\n`);
  for (const path of inspection.shadowedConfigPaths) context.stdout(`Shadowed config: ${path}\n`);
  if (inspection.sources.length === 0) context.stdout('No knowledge sources detected.\n');
  for (const source of inspection.sources) {
    context.stdout(
      `${source.id}\t${source.kind}\t${source.management}\t${source.adapter}\t${source.confidence}\t${source.exists ? 'present' : 'missing'}\t${displaySourcePath(inspection, source)}\n`,
    );
  }
  for (const diagnostic of inspection.diagnostics) context.stdout(`WARNING: ${diagnostic}\n`);
};

const renderPlanResult = (program: Command, context: CliContext, plan: ReturnType<typeof parseProjectPlan>): void => {
  context.stdout(renderProjectPlan(plan));
  if (plan.conflicts.length > 0) exitCodes.set(program, 1);
};

const renderKnowledgeValidation = async (program: Command, context: CliContext, doctor: boolean): Promise<void> => {
  const report = await validateKnowledge(await inspectKnowledge(await commandProject(program, context)), doctor);
  for (const diagnostic of report.diagnostics) {
    const write = diagnostic.severity === 'error' ? context.stderr : context.stdout;
    write(`${diagnostic.severity.toUpperCase()}: ${diagnostic.sourceId}: ${diagnostic.message}\n`);
    if (diagnostic.repair !== undefined) write(`Repair: ${diagnostic.repair}\n`);
  }
  if (report.valid) context.stdout('Knowledge stack is valid.\n');
  else exitCodes.set(program, 1);
};

const handleTodoPlan = async (program: Command, context: CliContext, plan: ReturnType<typeof parseProjectPlan>, apply: boolean): Promise<void> => {
  if (!apply) {
    renderPlanResult(program, context, plan);
    context.stdout('Run the same command with --apply to write these changes.\n');
    return;
  }
  const result = await applyProjectPlan(plan, arcantryVersion);
  for (const operation of result.applied) context.stdout(`${operation.action}: ${operation.path}\n`);
  if (result.applied.length === 0) context.stdout('No file changes.\n');
};

const selectTodoSource = (inspection: KnowledgeInspection, requested: string | undefined, allowMissing: boolean): ProjectSource => {
  if (requested !== undefined) return resolveTodoSource(inspection, requested, allowMissing);
  const sources = inspection.sources.filter((source) => source.kind === 'todo-txt' && (allowMissing || source.exists));
  if (sources.length === 1) return sources[0]!;
  if (sources.length === 0) throw new Error('No todo.txt source exists; choose --source root or --source local.');
  throw new Error('More than one todo.txt source exists; choose --source explicitly.');
};

const resolveTodoSource = (inspection: KnowledgeInspection, requested: string, allowMissing: boolean): ProjectSource => {
  const id = requested === 'root' ? 'todo-root' : requested === 'local' ? 'todo-local' : requested;
  const existing = inspection.sources.find((source) => source.id === id && source.kind === 'todo-txt');
  if (existing !== undefined && (allowMissing || existing.exists)) return existing;
  if (allowMissing && (id === 'todo-root' || id === 'todo-local')) {
    const path = id === 'todo-root' ? 'todo.txt' : '.local/todo.txt';
    return {
      id,
      kind: 'todo-txt',
      path,
      absolutePath: resolve(inspection.root, path),
      management: 'observe',
      adapter: 'todo-txt@1',
      from: [],
      visibility: id === 'todo-local' ? 'private' : 'shared',
      scope: '.',
      exists: false,
      origin: 'discovered',
      confidence: 'high',
      adapterStatus: 'supported',
    };
  }
  throw new Error(`Todo source is missing: ${id}.`);
};

const parseLine = (value: string): number => {
  const line = Number.parseInt(value, 10);
  if (!Number.isSafeInteger(line) || line < 1 || String(line) !== value) throw new Error('Todo line must be a positive integer.');
  return line;
};

const localDate = (): string => {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, '0');
  const day = String(now.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
};

type SkillCommandOptions = {
  catalogRoot?: string;
  scope?: string;
  compat?: string;
  target?: string;
  replace?: boolean;
};

type ResolvedSkillOperation = {
  source: Pick<SkillLinkOptions, 'catalogRoot' | 'sourceDirectory'>;
  targetRoots: string[];
  privateRoot: string | null;
};

const resolveSkillOperation = async (
  program: Command,
  context: CliContext,
  name: string,
  options: SkillCommandOptions,
): Promise<ResolvedSkillOperation> => {
  const targetRoots = await resolveSkillTargetRoots(program, context, options);
  if (options.scope === 'private') {
    const root = await resolveRepositoryRoot(commandCwd(program, context));
    const privateSkill = await inspectPrivateSkill(root, name);
    const publicRoot = await catalogRoot(program, context, options.catalogRoot);
    if ((await loadCatalog(publicRoot)).skills.some((entry) => entry.name === name)) {
      throw new Error(`Skill name conflict: ${name} exists in both the public catalog and .local/skills.`);
    }
    return { source: { sourceDirectory: privateSkill.directory }, targetRoots, privateRoot: root };
  }
  if (options.scope === 'repo') {
    const root = await resolveRepositoryRoot(commandCwd(program, context));
    if (await privateSkillExists(root, name)) {
      throw new Error(`Skill name conflict: ${name} exists in both the public catalog and .local/skills.`);
    }
  }
  return {
    source: { catalogRoot: await catalogRoot(program, context, options.catalogRoot) },
    targetRoots,
    privateRoot: null,
  };
};

const resolveSkillTargetRoots = async (
  program: Command,
  context: CliContext,
  options: Pick<SkillCommandOptions, 'scope' | 'compat' | 'target'>,
): Promise<string[]> => {
  if (options.target !== undefined && (options.scope !== undefined || options.compat !== undefined)) {
    throw new Error('--target cannot be combined with --scope or --compat.');
  }
  if (options.target !== undefined) return [resolve(commandCwd(program, context), options.target)];
  const compatibility = parseCompatibility(options.compat);
  if (compatibility !== null && options.scope === undefined) {
    throw new Error('--compat requires --scope user|repo|private.');
  }
  if (options.scope === 'user') {
    return compatibility === 'claude' ? [userSkillTargetRoot(), userClaudeSkillTargetRoot()] : [userSkillTargetRoot()];
  }
  if (options.scope === 'repo' || options.scope === 'private') {
    const root = await resolveRepositoryRoot(commandCwd(program, context));
    return compatibility === 'claude'
      ? [repositorySkillTargetRoot(root), repositoryClaudeSkillTargetRoot(root)]
      : [repositorySkillTargetRoot(root)];
  }
  if (options.scope !== undefined) throw new Error('--scope must be user, repo, or private.');
  throw new Error('Choose --scope user|repo|private or provide --target.');
};

const excludePrivateSkillLinks = async (root: string, name: string, targetRoots: string[]): Promise<void> => {
  const entries = ['.local/'];
  for (const targetRoot of targetRoots) {
    const relativeRoot = relative(resolve(root), resolve(targetRoot)).replaceAll('\\', '/');
    if (!relativeRoot.startsWith('../') && relativeRoot !== '') entries.push(`${relativeRoot}/${name}`);
  }
  const operations: PlanOperation[] = [];
  await planLocalGitExcludeEntries(root, entries, operations);
  if (operations.length === 0) return;
  await applyProjectPlan(createProjectPlan({
    toolVersion: arcantryVersion,
    root,
    sourceId: `private-skill-${name}`,
    transition: 'preserve',
    adapter: 'agent-skill@1',
    operations,
  }), arcantryVersion);
};

const parseCompatibility = (value: string | undefined): 'claude' | null =>
  value === undefined ? null : repositoryCompatibilitySchema.parse(value);

const formatError = (error: unknown): string => {
  if (error instanceof ZodError) return error.issues.map((issue) => issue.message).join(' ');
  return error instanceof Error ? error.message : String(error);
};

const isCommanderExit = (error: unknown): error is { exitCode: number } =>
  typeof error === 'object' && error !== null && 'exitCode' in error && typeof (error as { exitCode?: unknown }).exitCode === 'number';

const readStandardInput = async (): Promise<string> => {
  let content = '';
  process.stdin.setEncoding('utf8');
  for await (const chunk of process.stdin) content += chunk;
  return content;
};

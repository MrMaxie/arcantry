---
title: CLI
description: Command reference for repository adoption, project knowledge, todo.txt queues, local releases, and skills.
---

The executable is `arcantry`. Repository and source commands are under `repo`, queues under `todo`, local release operations under `release`, and procedural capabilities under `skills`.

## Global options

| Option | Contract |
| --- | --- |
| `--cwd <path>` | Run against another project or catalog location. |
| `--output <path>` | Save a preview to a new file without overwriting an existing file. |
| `--config <path>` | Select one explicit TOML configuration without merging. |
| `-V, --version` | Print the installed version. |
| `-h, --help` | Print command help. |

Global options appear before the command group.

## Repository adoption

| Command | Contract |
| --- | --- |
| `arcantry repo init --scope <shared\|private> [--compat claude]` | Create minimal configuration and universal managed guidance for one scope. Private scope also ensures the local Git exclusion. |
| `arcantry repo update --scope <shared\|private> [--compat claude]` | Refresh verified universal guidance and optionally add the Claude import adapter. |
| `arcantry repo detach [--capability <id>...] [--apply] [--json]` | Preview or apply full or capability-scoped project ownership transfer. |
| `arcantry repo remove --scope <shared\|private>` | Remove only verified configuration and managed guidance for one scope. |
| `arcantry repo validate` | Validate the active repository boundary and configured knowledge sources without writing. |
| `arcantry repo doctor` | Add repair guidance to the same read-only validation. |

Initialization does not create package-manager, runtime, task-runner, OpenSpec, changelog, or todo artifacts.

<!-- cli-evidence: remove-owned-only -->

`AGENTS.md` and `.local/AGENTS.md` remain canonical. `--compat claude` creates a managed import in `CLAUDE.md` or locally excluded `CLAUDE.local.md` without copying the guidance.

## Project knowledge

### `repo inspect`

```sh
arcantry repo inspect [--detailed|--json]
```

Reports the project boundary, active and shadowed configuration, present and absent standard sources, applicable methodologies, and `.local` policy without writing. `--detailed` expands the human-readable evidence. `--json` returns the complete versioned data model.

### `repo plan`

```sh
arcantry repo plan --source <id> --transition <strategy>
  [--to-path <path>]
  [--to-adapter <adapter>]
  [--from <source...>]
  [--managed-from <version>]
  [--delete-source]
  [--json]
```

`--transition` accepts `preserve`, `adopt`, `rebind`, `cutover`, `migrate`, or `relocate`. During adoption, `--from` records explicit source dependencies. JSON output is the serializable input for apply.

### `repo apply`

```sh
arcantry repo apply --plan <path|-> [--allow-outside <path>...]
```

Applies an unchanged plan to the currently resolved project. `-` reads standard input. Apply rejects a different project root, conflicts, changed inputs, incompatible tool versions, corrupt planned content, and operations outside the project before writing. Repeat `--allow-outside` for each exact external operation path that the plan is allowed to change.

## Todo commands

| Command | Contract |
| --- | --- |
| `arcantry todo list [--source <id>] [--json]` | List one or all detected queues. JSON includes queue and line hashes. `root` and `local` are aliases. |
| `arcantry todo add <task> [--source <id>] [--apply]` | Preview or add one task. |
| `arcantry todo complete <line> [--source <id>] [--date <YYYY-MM-DD>] [--apply]` | Preview or complete one task. |
| `arcantry todo move <line> --from <id> --to <id> [--apply]` | Preview or move one line between queues. |
| `arcantry todo defer <line> --source <id> [--until <YYYY-MM-DD>] [--wait <slug>] [--apply]` | Preview or defer one line by date, manual condition, or both. |
| `arcantry todo resume <line> --source <id> [--apply]` | Preview or remove both deferral markers. |

Mutations preview unless `--apply` is present. When multiple queues exist, choose the source explicitly.

## Release commands

Release commands require a configured `[release]` block. They manage local files only and never commit, tag, push, publish, or change CI.

| Command | Contract |
| --- | --- |
| `arcantry release baseline <version> --date <YYYY-MM-DD> [--unit <id>] [--apply] [--json]` | Preview or record an existing project or unit version as the release baseline. |
| `arcantry release plan [--unit <id>] [--json]` | Report the current and next version, effective impact, selected changes and dependency readiness. |
| `arcantry release cut [--date <YYYY-MM-DD>] [--unit <id>] [--apply] [--json]` | Preview or write the next unit manifest, configured versions, and managed changelog. |
| `arcantry release render [--unit <id>] [--apply] [--json]` | Preview or write the deterministic managed changelog. |
| `arcantry release check [--unit <id>] [--sealed]` | Check release consistency. `--sealed` also requires complete scoped assignment and the existing Git seal. |

`baseline`, `cut`, and `render` only print their drift-checked plan unless `--apply` is present. A normal check allows active and unassigned work; a sealed check is the final release gate.

`independent` and `composed` projects require `--unit` for baseline, plan, cut and render. An unscoped normal check validates every unit. A sealed multi-unit check requires `--unit` and ignores unrelated work owned by other units.

Composed plan JSON includes `dependencies`, newer `pendingDependencies`, and `ready`. A parent can adopt a newer direct dependency only when one of its selected OpenSpec outcomes acknowledges that dependency.

## Skill commands

| Command | Contract |
| --- | --- |
| `arcantry skills list [--scope <public\|private>] [--catalog-root <path>]` | List public catalog skills or private repository skills. |
| `arcantry skills inspect <name> [--scope <public\|private>] [--catalog-root <path>]` | Show one public or private canonical package. |
| `arcantry skills status [<name>] [--scope <user\|repo>] [--target <path>] [--catalog-root <path>] [--check] [--json]` | Report local management, version, revision, digest and drift. Network access occurs only with `--check`. |
| `arcantry skills update <name> [--scope <user\|repo>] [--target <path>] [--catalog-root <path>] [--compat claude]` | Preview one update from the official GitHub `master` or an explicit local catalog. Use global `--output` to save the exact plan. |
| `arcantry skills apply --plan <path\|->` | Apply one unchanged update plan after rechecking its source package, installed preimage and targets. |
| `arcantry skills link <name> (--scope <user\|repo\|private> [--compat claude] \| --target <path>) [--replace]` | Link one canonical skill into a standard scope or one advanced explicit destination. |
| `arcantry skills unlink <name> (--scope <user\|repo\|private> [--compat claude] \| --target <path>)` | Remove only exact universal, compatibility, or explicit links. |
| `arcantry skills doctor [--scope <user\|repo\|private>] [--compat claude] [--target <path>] [--catalog-root <path>]` | Validate packages and optionally inspect universal and compatibility links. |

`--target` cannot be combined with `--scope` or `--compat`. User scope targets `~/.agents/skills`; repository and private scopes target `<repo>/.agents/skills`. `--compat claude` also targets the corresponding `.claude/skills` directory. Private scope reads the canonical package from `.local/skills` and excludes its links locally. `--replace` backs up an ordinary target instead of overwriting it silently.

Each public skill has its own `1.0.0` version, role, exact source revision and SHA-256 package digest. Managed updates use immutable per-skill snapshots and receipts outside skill discovery directories. Unmanaged, development-linked, locally modified and private skills are reported but never adopted or overwritten automatically.

Skills remain usable without project configuration. A declared tool dependency does not authorize an external write.

## Project answers

| Command | Purpose |
| --- | --- |
| `arcantry context [--detailed] [--json]` | Discover project state, sources, work and available tools. |
| `arcantry next [--change <id>] [--json]` | Recommend the next step and report dependency blockers. |
| `arcantry explain <topic> [--json]` | Read the format, example and project rules for a topic. |
| `arcantry mcp` | Serve the same read-only answers and release plans over stdio. |

Topics: `proposal`, `specs`, `design`, `tasks`, `release`, `todo`, `versions`, `changelog`, `rules`, `workflow`.

Project text and context profiles never grant conversational approval. Configure `[workflow].order` and `[workflow.dependencies]` for a deliberate work sequence. Optional `[context].focus` and `exclude` arrays describe relevance only.

Without the CLI, read `AGENTS.md`, the selected OpenSpec configuration and change tasks, and the relevant todo queue directly. Use the project's schema templates. Missing Arcantry, OpenSpec or Varlock executables do not prohibit ordinary file-based work.

An MCP host can start `arcantry --cwd <path> mcp`. The server exposes `context`, `next`, `explain` and `release_plan`; it cannot apply changes or publish anything. Start it only for a project whose context the host is authorized to read.

## Local diagnostics and recovery

| Command | Purpose |
| --- | --- |
| `arcantry diagnostics` | Report allowlisted platform, source and tool metadata without paths, contents or environment values. |
| `arcantry repo recover [--acknowledge]` | Inspect an interrupted transaction; acknowledge only after all targets are restored or all planned results are verified. |

Use the global `--output` option with a preview command to save its exact plan. Saved files are not overwritten. Apply a saved plan through `arcantry repo apply --plan`; changed inputs or targets are rejected. A plan may contain private content, which is disclosed when saving it. Diagnostic output can also be saved explicitly with `--output` and is never uploaded.

Environment schemas (`.env.schema`, `.local/.env.schema` or configured `environment-schema` sources using `env-spec@1`) support observation only. Discovery does not run Varlock, read schema contents or load environment values.

Recovery runs against the exact project root supplied by `--cwd`. If the interrupted plan included external paths, pass the same exact `--allow-outside` paths to `arcantry repo recover`. It reports mixed or edited targets for manual review and only acknowledges a completely original or completely applied result. Staging interruptions before the journal is created can leave unused `.arcantry-*.tmp` files; these never prove ownership and are not automatically removed. The recovery contract covers process interruption, not a guarantee against storage-device or power failure.

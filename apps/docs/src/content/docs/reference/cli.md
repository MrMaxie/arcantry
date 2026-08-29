---
title: CLI
description: Command reference for repository adoption, project knowledge, todo.txt queues, local releases, and skills.
---

The executable is `arcantry`. Repository and source commands are under `repo`, queues under `todo`, local release operations under `release`, and procedural capabilities under `skills`.

## Global options

| Option | Contract |
| --- | --- |
| `--cwd <path>` | Run against another project or catalog location. |
| `--config <path>` | Select one explicit TOML configuration without merging. |
| `-V, --version` | Print the installed version. |
| `-h, --help` | Print command help. |

Global options appear before the command group.

## Repository adoption

| Command | Contract |
| --- | --- |
| `repo init --scope <shared\|private> [--compat claude]` | Create minimal configuration and universal managed guidance for one scope. Private scope also ensures the local Git exclusion. |
| `repo update --scope <shared\|private> [--compat claude]` | Refresh verified universal guidance and optionally add the Claude import adapter. |
| `repo remove --scope <shared\|private>` | Remove only verified configuration and managed guidance for one scope. |
| `repo validate` | Validate the active repository boundary and configured knowledge sources without writing. |
| `repo doctor` | Add repair guidance to the same read-only validation. |

Initialization does not create package-manager, runtime, task-runner, OpenSpec, changelog, or todo artifacts.

<!-- cli-evidence: remove-owned-only -->

`AGENTS.md` and `.local/AGENTS.md` remain canonical. `--compat claude` creates a managed import in `CLAUDE.md` or locally excluded `CLAUDE.local.md` without copying the guidance.

## Project knowledge

### `repo inspect`

```text
arcantry repo inspect [--json]
```

Reports the active and shadowed configuration and every discovered or configured source without writing.

### `repo plan`

```text
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

```text
arcantry repo apply --plan <path|-> [--allow-outside <path>...]
```

Applies an unchanged plan to the currently resolved project. `-` reads standard input. Apply rejects a different project root, conflicts, changed inputs, incompatible tool versions, corrupt planned content, and operations outside the project before writing. Repeat `--allow-outside` for each exact external operation path that the plan is allowed to change.

## Todo commands

| Command | Contract |
| --- | --- |
| `todo list [--source <id>]` | List one or all detected queues. `root` and `local` are aliases. |
| `todo add <task> [--source <id>] [--apply]` | Preview or add one task. |
| `todo complete <line> [--source <id>] [--date <YYYY-MM-DD>] [--apply]` | Preview or complete one task. |
| `todo move <line> --from <id> --to <id> [--apply]` | Preview or move one line between queues. |

Mutations preview unless `--apply` is present. When multiple queues exist, choose the source explicitly.

## Release commands

Release commands require a configured `[release]` block. They manage local files only and never commit, tag, push, publish, or change CI.

| Command | Contract |
| --- | --- |
| `release baseline <version> --date <YYYY-MM-DD> [--unit <id>] [--apply] [--json]` | Preview or record an existing project or unit version as the release baseline. |
| `release plan [--unit <id>] [--json]` | Report the current and next version, effective impact, selected changes and dependency readiness. |
| `release cut [--date <YYYY-MM-DD>] [--unit <id>] [--apply] [--json]` | Preview or write the next unit manifest, configured versions, and managed changelog. |
| `release render [--unit <id>] [--apply] [--json]` | Preview or write the deterministic managed changelog. |
| `release check [--unit <id>] [--sealed]` | Check release consistency. `--sealed` also requires complete scoped assignment and the existing Git seal. |

`baseline`, `cut`, and `render` only print their drift-checked plan unless `--apply` is present. A normal check allows active and unassigned work; a sealed check is the final release gate.

`independent` and `composed` projects require `--unit` for baseline, plan, cut and render. An unscoped normal check validates every unit. A sealed multi-unit check requires `--unit` and ignores unrelated work owned by other units.

Composed plan JSON includes `dependencies`, newer `pendingDependencies`, and `ready`. A parent can adopt a newer direct dependency only when one of its selected OpenSpec outcomes acknowledges that dependency.

## Skill commands

| Command | Contract |
| --- | --- |
| `skills list [--scope <public\|private>] [--catalog-root <path>]` | List public catalog skills or private repository skills. |
| `skills inspect <name> [--scope <public\|private>] [--catalog-root <path>]` | Show one public or private canonical package. |
| `skills link <name> (--scope <user\|repo\|private> [--compat claude] \| --target <path>) [--replace]` | Link one canonical skill into a standard scope or one advanced explicit destination. |
| `skills unlink <name> (--scope <user\|repo\|private> [--compat claude] \| --target <path>)` | Remove only exact universal, compatibility, or explicit links. |
| `skills doctor [--scope <user\|repo\|private>] [--compat claude] [--target <path>] [--catalog-root <path>]` | Validate packages and optionally inspect universal and compatibility links. |

`--target` cannot be combined with `--scope` or `--compat`. User scope targets `~/.agents/skills`; repository and private scopes target `<repo>/.agents/skills`. `--compat claude` also targets the corresponding `.claude/skills` directory. Private scope reads the canonical package from `.local/skills` and excludes its links locally. `--replace` backs up an ordinary target instead of overwriting it silently.

Skills remain usable without project configuration. A declared tool dependency does not authorize an external write.

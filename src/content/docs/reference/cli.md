---
title: CLI
description: Command reference for repository adoption, project knowledge, todo.txt queues, and skills.
---

The executable is `arcantry`. Repository and source commands are under `repo`, queues under `todo`, and procedural capabilities under `skills`.

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
| `repo init --scope shared|private` | Create minimal configuration and managed guidance for one scope. Private scope also ensures the local Git exclusion. |
| `repo update --scope shared|private` | Refresh verified managed guidance without rewriting configuration. |
| `repo remove --scope shared|private` | Remove only verified configuration and managed guidance for one scope. |
| `repo validate` | Validate the active repository boundary and configured knowledge sources without writing. |
| `repo doctor` | Add repair guidance to the same read-only validation. |

Initialization does not create package-manager, runtime, task-runner, OpenSpec, changelog, or todo artifacts.

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
  [--managed-from <version>]
  [--delete-source]
  [--json]
```

`--transition` accepts `preserve`, `adopt`, `rebind`, `cutover`, `migrate`, or `relocate`. JSON output is the serializable input for apply.

### `repo apply`

```text
arcantry repo apply --plan <path|->
```

Applies an unchanged plan. `-` reads standard input. Apply rejects conflicts, changed inputs, incompatible tool versions, and corrupt planned content before writing.

## Todo commands

| Command | Contract |
| --- | --- |
| `todo list [--source <id>]` | List one or all detected queues. `root` and `local` are aliases. |
| `todo add <task> [--source <id>] [--apply]` | Preview or add one format-preserving task. |
| `todo complete <line> [--source <id>] [--date <YYYY-MM-DD>] [--apply]` | Preview or complete one task. |
| `todo move <line> --from <id> --to <id> [--apply]` | Preview or move one line between queues. |

Mutations preview unless `--apply` is present. When multiple queues exist, choose the source explicitly.

## Skill commands

| Command | Contract |
| --- | --- |
| `skills list [--catalog-root <path>]` | List each skill with its family and tags. |
| `skills inspect <name> [--catalog-root <path>]` | Show purpose, family, tags, and usage scenarios. |
| `skills link <name> --scope user|repo [--agent codex|claude|gemini] [--replace]` | Link one canonical skill into the selected agent's native skill directory. |
| `skills link <name> --target <path> [--replace]` | Link to one advanced explicit destination. |
| `skills unlink <name> --scope user|repo [--agent codex|claude|gemini]` | Remove only the exact Arcantry link for that scope and agent. |
| `skills doctor [--scope user|repo] [--agent codex|claude|gemini] [--target <path>]` | Validate packages and optionally inspect link health. |

`--target` cannot be combined with `--scope` or `--agent`. Without `--agent`, user scope targets `~/.agents/skills` and repository scope targets `<repo>/.agents/skills`. Claude Code targets `.claude/skills`; Gemini CLI targets `.gemini/skills`. `--replace` backs up an ordinary target instead of overwriting it silently.

Skills remain usable without project configuration. A declared tool dependency does not authorize an external write.

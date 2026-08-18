---
title: CLI
description: Complete command reference for project knowledge, todo.txt queues, and skills.
---

The executable is `arcantry`. Project knowledge commands are under `repo`, short queues are under `todo`, and procedural capabilities are under `skills`.

## Global options

Global options appear before the command group:

| Option | Contract |
| --- | --- |
| `--cwd <path>` | Run against another project or catalog location. An explicit value is the resolved project root. |
| `--config <path>` | Select one `arcantry.toml` without merging configuration layers. The path resolves from `--cwd` or the process working directory. |
| `-V, --version` | Print the installed Arcantry version. |
| `-h, --help` | Print command help. |

Example:

```sh
arcantry --cwd ./project --config ../control/project.toml repo inspect --json
```

## Project knowledge commands

### `repo inspect`

```text
arcantry repo inspect [--json]
```

Discovers and describes the resolved source stack without writing. `--json` emits the complete machine-readable inspection.

### `repo plan`

```text
arcantry repo plan --source <id> --transition <strategy>
  [--to-path <path>]
  [--to-adapter <adapter>]
  [--managed-from <version>]
  [--delete-source]
  [--json]
```

`--source` uses an id reported by `repo inspect`. `--transition` is `preserve`, `adopt`, `rebind`, `cutover`, `migrate`, or `relocate`. The optional target path, target adapter, changelog boundary, and verified source deletion refine transitions that need them. `--json` emits the serialized plan required by `repo apply`; a plan with conflicts exits unsuccessfully.

### `repo apply`

```text
arcantry repo apply --plan <path|->
```

Applies an unchanged serialized plan. `-` reads the plan from standard input. Apply rejects tool-version mismatch, conflicts, changed inputs, and corrupt planned content before accepting the transition.

### `repo validate`

```text
arcantry repo validate
```

Runs read-only validation. Configured projects use source management and adapter contracts. Legacy projects use the private legacy repository contract.

### `repo doctor`

```text
arcantry repo doctor
```

Runs read-only diagnostics and adds repair guidance. It does not apply repairs or upgrade adapters.

## Legacy repository compatibility

These commands support repositories that still use `.local/arcantry.json`. They are not the recommended entrypoint for new adoption.

| Command | Contract |
| --- | --- |
| `repo init --docs none [--agent <agent>...] [--source <name=mode>...] [--operational-source <name>]` | Initialize legacy managed sections. Only `--docs none` is accepted; `shared` and `local` fail before writes. |
| `repo update` | Refresh verified legacy-owned artifacts while preserving configuration and user content. |
| `repo remove` | Remove only verified legacy-owned artifacts and sections. |

`repo validate` and `repo doctor` automatically use legacy behavior only when the legacy contract is present and the versioned project knowledge stack is not selected.

## Todo commands

| Command | Contract |
| --- | --- |
| `todo list [--source <id>]` | List one or all detected queues. `root` and `local` are standard aliases. |
| `todo add <task> [--source <id>] [--apply]` | Preview or add one format-preserving task. |
| `todo complete <line> [--source <id>] [--date <YYYY-MM-DD>] [--apply]` | Preview or complete one task, using the local date by default. |
| `todo move <line> --from <id> --to <id> [--apply]` | Preview or move one line explicitly between queues. |

Todo mutations preview a plan unless `--apply` is present. A mutating command must select a source when more than one queue is available.

## Skill commands

| Command | Contract |
| --- | --- |
| `skills list [--catalog-root <path>]` | List validated skills in the selected catalog. |
| `skills inspect <name> [--catalog-root <path>]` | Show public purpose, tags, and usage scenarios. |
| `skills link <name> [--catalog-root <path>] [--target <path>] [--replace]` | Link one canonical skill. `--replace` permits a backup or replacement instead of silently overwriting. |
| `skills unlink <name> [--catalog-root <path>] [--target <path>]` | Remove only the verified Arcantry-managed link. |
| `skills doctor [--catalog-root <path>] [--target <path>]` | Check catalog packages and optional link health without writing. |

Skills remain usable without configuration or recognized project sources. A skill's declared dependencies do not authorize external writes.

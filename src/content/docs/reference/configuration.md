---
title: Configuration
description: Define optional project roots, source responsibilities, compatibility, and adapters in arcantry.toml.
---

`arcantry.toml` is optional. Use it when discovery alone cannot express the project root, source paths, management responsibilities, compatibility, or relationships that the project needs.

## Resolution precedence

Arcantry resolves at most one configuration:

1. `--config <path>` selects one explicit file.
2. Otherwise, the nearest `arcantry.toml` in the current directory or an ancestor is selected.
3. If neither exists, Arcantry uses configuration-free discovery.

Configurations are never merged. An explicit `--cwd <path>` is the project root. Without explicit `--cwd`, `[project].root` resolves relative to the configuration file; when that field is absent, the configuration directory is the root.

An explicit configuration can live outside the target project. This supports a zero-footprint or private control plane:

```sh
arcantry --cwd ./project --config ../control/project.toml repo inspect
```

## Complete example

```toml
config_version = 1

[toml-schema]
location = "https://mrmaxie.github.io/arcantry/schemas/arcantry-config-v1.tosd"
version = "1.0.0"

[tool]
requires = ">=0.3.0 <1.0.0"

[project]
root = "../workspace"

[sources.intent]
kind = "openspec"
path = "openspec"
management = "manage"
adapter = "openspec@1"
scope = "."

[sources.history]
kind = "changelog"
path = "CHANGELOG.md"
management = "manage"
adapter = "keep-a-changelog@2"
from = ["intent"]
managed_from = "1.0.0"
visibility = "shared"

[sources.todo_private]
kind = "todo-txt"
path = ".local/todo.txt"
management = "observe"
adapter = "todo-txt@1"
visibility = "private"
```

## Top-level fields

| Field | Required | Contract |
| --- | --- | --- |
| `config_version` | Yes | Configuration format version. The current value is `1`. |
| `[toml-schema].location` | No | Non-empty URL or path used by editors to find the TOML Schema. |
| `[toml-schema].version` | No | Full SemVer version of that schema. |
| `[tool].requires` | No | SemVer range that the running Arcantry version must satisfy. |
| `[project].root` | No | Project root relative to the configuration file when `--cwd` is not explicit. |
| `[sources.<id>]` | No | One independently versioned knowledge source. An id starts with a letter or number, then allows letters, numbers, `.`, `_`, and `-`. |

## Source fields

| Field | Required | Default | Contract |
| --- | --- | --- | --- |
| `kind` | Yes | - | `openspec`, `changelog`, or `todo-txt`. |
| `path` | Yes | - | Non-empty source path, normally relative to the resolved project root. |
| `management` | No | `observe` | `ignore`, `observe`, `validate`, or `manage`. |
| `adapter` | Yes | - | Versioned adapter id in `<name>@<integer-version>` form. The name starts with a lowercase letter and then allows lowercase letters, numbers, and `-`. |
| `from` | No | `[]` | Source ids that provide semantic input to this source. |
| `managed_from` | No | - | Full SemVer changelog boundary used for managed future history. |
| `visibility` | No | Path-based | `private` for `.local` paths, otherwise `shared`. |
| `scope` | No | `.` | Project scope governed by an OpenSpec authority. |

## Graph and authority rules

`from` relationships form a directed acyclic graph. Every referenced id must exist. A managed changelog requires at least one OpenSpec source in `from`; an observed changelog can exist without OpenSpec. Managed OpenSpec sources cannot overlap at the same scope.

## Paths and privacy

Project-local configuration rejects absolute source paths. Absolute paths are accepted only through an explicit external configuration, and not when that configuration resolves inside the project it controls. A relative source path inside `.local` is always private and cannot be declared shared.

The serialized plan format contains target paths and planned write content. Store plan files according to the most sensitive source they affect; keeping a plan outside the target project does not make its contents public.

## Monorepos

Use `[project].root` or explicit `--cwd` to establish the shared root. Give each authority a non-overlapping `scope`, and give each source a distinct id and path. One configuration can then describe several OpenSpec authorities or changelog projections without merging configuration files.

## Versions and adapters

Configuration format, Arcantry tool, source adapter, and source data versions are independent. Updating Arcantry does not rewrite a supported older adapter. Built-in compatibility is listed in [Project knowledge stack](/arcantry/reference/repository-contract/#built-in-adapter-compatibility).

The public editor contract is [arcantry-config-v1.tosd](/arcantry/schemas/arcantry-config-v1.tosd). It follows TOML Schema 1.0.0. Runtime validation additionally enforces SemVer compatibility, graph cycles, authority overlap, path privacy, and managed changelog relationships.

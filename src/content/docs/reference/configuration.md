---
title: Configuration
description: Configure shared or private project knowledge sources with one TOML schema.
---

Arcantry uses the same schema for shared `arcantry.toml` and private `.local/arcantry.toml`. Configuration is optional.

## Resolution precedence

Arcantry selects one file:

1. `--config <path>` selects an explicit file.
2. Otherwise, each directory from the working directory toward the filesystem root is checked for `.local/arcantry.toml`, then `arcantry.toml`.
3. The first match is active. A sibling file at that boundary is reported as shadowed.
4. Without a match, Arcantry uses configuration-free discovery.

Configurations are never merged. A private configuration resolves its default project root from the directory that contains `.local`, not from `.local` itself.

## Minimal configuration

```toml
config_version = 1

[toml-schema]
location = "https://mrmaxie.github.io/arcantry/schemas/arcantry-config-v1.tosd"
version = "1.0.0"
```

`repo init --scope shared` creates this contract in `arcantry.toml`. `repo init --scope private` creates it in `.local/arcantry.toml`.

## Source example

```toml
config_version = 1

[toml-schema]
location = "https://mrmaxie.github.io/arcantry/schemas/arcantry-config-v1.tosd"
version = "1.0.0"

[tool]
requires = ">=1.0.0 <2.0.0"

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

[sources.tasks]
kind = "todo-txt"
path = ".local/todo.txt"
management = "observe"
adapter = "todo-txt@1"
visibility = "private"
```

## Top-level fields

| Field | Required | Contract |
| --- | --- | --- |
| `config_version` | Yes | Configuration format version. The value is `1`. |
| `[toml-schema].location` | No | URL or path used by editors to find the TOML Schema. |
| `[toml-schema].version` | No | Full SemVer version of that schema. |
| `[tool].requires` | No | SemVer range the running Arcantry version must satisfy. |
| `[project].root` | No | Project root relative to the configuration boundary. |
| `[sources.<id>]` | No | One independently versioned knowledge source. |

## Source fields

| Field | Required | Default | Contract |
| --- | --- | --- | --- |
| `kind` | Yes | - | `openspec`, `changelog`, or `todo-txt`. |
| `path` | Yes | - | Source path, normally relative to the project root. |
| `management` | No | `observe` | `ignore`, `observe`, `validate`, or `manage`. |
| `adapter` | Yes | - | Versioned adapter id in `<name>@<integer-version>` form. |
| `from` | No | `[]` | Source ids that provide semantic input to this source. |
| `managed_from` | No | - | Full SemVer boundary for managed changelog meaning. |
| `visibility` | No | Path-based | `private` for `.local` paths, otherwise `shared`. |
| `scope` | No | `.` | Project scope governed by an OpenSpec authority. |

## Relationships and privacy

`from` relationships form an acyclic graph. A managed changelog requires at least one OpenSpec source. Managed OpenSpec scopes cannot overlap.

Project-local configuration rejects absolute source paths. An explicit external configuration may use them. A source inside `.local` is private and cannot be declared shared.

Shared and private source content is not synchronized automatically. Use inspection and an explicit transition plan to promote, relocate, or preserve it.

The editor contract is [arcantry-config-v1.tosd](/arcantry/schemas/arcantry-config-v1.tosd). Runtime validation also enforces SemVer compatibility, graph cycles, authority overlap, path privacy, and changelog dependencies.

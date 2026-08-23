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

[release]
adapter = "openspec-release@1"
manifests_path = "releases"
changelog_source = "history"
tag_prefix = "v"
repository_url = "https://github.com/example/project"

[[release.version_sources]]
path = "Cargo.toml"
adapter = "cargo-workspace@1"
```

The equivalent private intent and release sources use `.local/openspec` and `.local/CHANGELOG.md`. They can be configured explicitly or discovered without configuration.

## Top-level fields

| Field | Required | Contract |
| --- | --- | --- |
| `config_version` | Yes | Configuration format version. The value is `1`. |
| `[toml-schema].location` | No | URL or path used by editors to find the TOML Schema. |
| `[toml-schema].version` | No | Full SemVer version of that schema. |
| `[tool].requires` | No | SemVer range the running Arcantry version must satisfy. |
| `[project].root` | No | Project root relative to the configuration boundary. |
| `[sources.<id>]` | No | One independently versioned knowledge source. |
| `[release]` | No | Local OpenSpec-backed release planning and rendering. |

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

`from` relationships form an acyclic graph. A managed changelog requires at least one OpenSpec source. Shared and private OpenSpec authorities can govern the same project scope independently, but authorities with the same visibility cannot overlap.

Project-local configuration rejects absolute source paths. An explicit external configuration may use them. A source inside `.local` is private and cannot be declared shared.

Shared and private source content is not synchronized automatically. Use inspection and an explicit transition plan to promote, relocate, or preserve it.

A shared changelog cannot depend on private OpenSpec because collaborators could not reproduce that release meaning. A private changelog may depend on shared OpenSpec, private OpenSpec, or both.

## Release fields

| Field | Required | Default | Contract |
| --- | --- | --- | --- |
| `adapter` | Yes | - | `openspec-release@1` or the opt-in `openspec-release@2`. |
| `topology` | v2 only | `single` | `single`, `independent`, or `composed`. |
| `manifests_path` | Single only | - | Directory containing `<version>.yaml` release manifests. |
| `changelog_source` | Single only | - | Id of the managed changelog source for the release. |
| `tag_prefix` | Single only | `v` | Prefix used only for generated changelog links. |
| `repository_url` | No | - | Repository URL used for comparison links. Links are omitted when absent. |
| `version_sources` | Single only | - | One or more explicit `path` and `adapter` tables. |
| `units` | Multi-unit only | - | Named release-unit tables for `independent` and `composed`. |

`openspec-release@1` keeps the original single-release contract unchanged. `openspec-release@2` with no topology also uses the flat single-release shape. Multi-unit configurations move manifest, changelog, tag, version-source, selector and dependency ownership into each unit:

```toml
[release]
adapter = "openspec-release@2"
topology = "composed"

[release.units.core]
manifests_path = "releases/core"
changelog_source = "core_history"
tag_prefix = "core/v"

[[release.units.core.version_sources]]
path = "packages/core/package.json"
adapter = "json-package@1"

[[release.units.core.selectors]]
source = "shared_openspec"
components = ["product:core"]

[release.units.app]
manifests_path = "releases/app"
changelog_source = "app_history"
tag_prefix = "app/v"
dependencies = ["core"]

[[release.units.app.version_sources]]
path = "apps/app/package.json"
adapter = "json-package@1"

[[release.units.app.selectors]]
source = "shared_openspec"
components = ["product:app"]
```

An `independent` topology forbids dependencies. A `composed` topology requires an acyclic dependency graph with at least one edge. Paths, changelog sources, version-source paths and tag prefixes are owned by exactly one unit.

Every selector names an OpenSpec source used by the unit's changelog. Omitting `components` claims the whole source exclusively. Component selectors may split a shared OpenSpec source between units, but ownership cannot overlap. A release-bearing archived change that matches no unit is an error.

Version source adapters are `json-package@1` for a top-level JSON `version` and `cargo-workspace@1` for `[workspace.package] version` in Cargo TOML. Release commands do not inspect or update an unconfigured version file.

A baseline manifest anchors an existing version without reconstructing unknown history. Later versions are computed from archived OpenSpec release artifacts. Internal artifacts stay in manifests and SemVer planning but are omitted from the public changelog. In composed projects, each parent manifest pins exact direct-dependency versions. A child release never bumps its parent automatically.

The editor contract is [arcantry-config-v1.tosd](/arcantry/schemas/arcantry-config-v1.tosd). Runtime validation also enforces SemVer compatibility, graph cycles, authority overlap, path privacy, and changelog dependencies.

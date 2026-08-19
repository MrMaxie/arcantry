---
title: Adoption paths
description: Choose configuration-free, private, shared, external, or skills-only Arcantry adoption.
---

Inspect first, then adopt only the boundaries the project needs.

```sh
arcantry repo inspect
```

Inspection is read-only and works without Git, configuration, or recognized sources.

In a Git repository, adoption resolves `.local/` before using local state. If the configured default remote branch already tracks `.local/`, Arcantry preserves that established repository policy and reports the conflict with its private-local convention. Otherwise it keeps the directory private through `.git/info/exclude`. If only the current index tracks `.local/`, removal from the index is planned separately, requires explicit approval, and preserves the working files.

## Choose an adoption path

| Need | Start here | Repository footprint |
| --- | --- | --- |
| Inspect only | `arcantry repo inspect` | None. |
| Private project setup | `arcantry repo init --scope private` | `.local/arcantry.toml`, `.local/AGENTS.md`, and a local Git exclusion. |
| Shared project setup | `arcantry repo init --scope shared` | `arcantry.toml` and a managed section in `AGENTS.md`. |
| External control | Use `--cwd` with `--config`. | None in the target project. |
| One source transition | Inspect, plan, review, and apply. | Only the explicitly applied source change. |
| Skills only | Inspect or link a catalog skill. | One link in a standard Agent Skills directory. |
| Private skill fallback | Keep an explicitly selected package or override in `.local/skills`. | Locally excluded `.agents` links and optional Claude aliases. |

## Private setup

Use private scope for personal workflow, workstation-specific context, or a project that should not track Arcantry configuration:

```sh
arcantry repo init --scope private
arcantry repo validate
```

Arcantry manages `.local/arcantry.toml` and the Arcantry section in `.local/AGENTS.md`. In a Git repository it also ensures that `.local/` appears in `.git/info/exclude`. It does not add `.local/` to the shared `.gitignore`.

## Shared setup

Use shared scope when collaborators and automation should read the same project boundary:

```sh
arcantry repo init --scope shared
arcantry repo validate
```

Arcantry manages `arcantry.toml` and only its marked section in `AGENTS.md`. Surrounding project instructions remain project-owned.

Both initialization paths are minimal. They do not create package manifests, lockfiles, runtime configuration, task runners, OpenSpec sources, changelogs, or todo queues.

Add `--compat claude` only when Claude Code must consume the same guidance. Shared compatibility creates a managed `CLAUDE.md` import of `AGENTS.md`; private compatibility creates a locally excluded `CLAUDE.local.md` import of `.local/AGENTS.md`.

## Shared and private configuration together

The two files use the same schema and remain independent. At each directory Arcantry checks `.local/arcantry.toml` before `arcantry.toml`. The first match is active, the sibling is reported as shadowed, and neither file is merged.

Use an explicit reviewed promotion or relocation when content must move between shared and private sources. Do not mirror the files automatically.

Configuration-free inspection recognizes `openspec`, `.local/openspec`, `CHANGELOG.md`, `.local/CHANGELOG.md`, `todo.txt`, and `.local/todo.txt` independently.

## External configuration

Keep configuration outside the target when the project must remain unchanged:

```sh
arcantry --cwd ./project --config ../control/project.toml repo inspect
```

An explicit file has highest precedence. It may use absolute source paths when it remains outside the project it controls.

## Adopt one source

Configuration describes source responsibility. It does not create the source. Inspect the stack, use the reported source id, and review a plan before applying it:

```sh
arcantry repo plan --source <id> --transition adopt --json > plan.json
arcantry repo apply --plan plan.json
```

Available transitions preserve, adopt, rebind, cut over, migrate, or relocate one source. Apply refuses changed inputs and corrupt planned content.

## Use queues or skills independently

Todo queues and skills do not require project configuration:

```sh
arcantry todo list
arcantry skills list
arcantry skills inspect <name>
arcantry skills link <name> --scope user
```

Todo mutations preview by default and write only with `--apply`. A linked skill remains a procedure, not project authority.

Prefer user-scoped skills. Adoption does not install, update, or copy them automatically. When a required skill is missing, choose between the recommended user-wide installation and a private repository package. When the user-wide skill is unsuitable for the required version or compatibility, choose between updating it user-wide and creating a local override.

Use `.local/skills/<name>/SKILL.md` with `arcantry skills link <name> --scope private` only after selecting the private fallback. Add `--compat claude` only for the extra branded alias.

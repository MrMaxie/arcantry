---
name: adopt-arcantry
description: Adopt or validate Arcantry in shared, private, or configuration-free project scope. Use to inspect sources, initialize minimal repository state, plan explicit transitions, or remove verified Arcantry-owned artifacts.
---

# Adopt Arcantry

Adopt only the information layers the project needs. Preserve project-owned content and stop when a plan reports a conflict.

## Inspect

1. Read the applicable repository and private instructions.
2. Run `arcantry repo inspect` before proposing changes.
3. Identify the active and shadowed configuration, then list each OpenSpec, changelog, and todo.txt source with its adapter, visibility, management level, and dependencies.
4. Treat `.local/` as a privacy boundary and keep shared and private sources independent.

An explicit `--config <path>` wins. Otherwise Arcantry checks `.local/arcantry.toml` before `arcantry.toml` at each directory while walking toward the filesystem root. It uses the first match and never merges configuration files.

## Initialize the repository boundary

Choose one explicit scope:

```text
arcantry repo init --scope shared
arcantry repo init --scope private
```

Shared initialization manages `arcantry.toml` and the Arcantry section in `AGENTS.md`. Private initialization manages `.local/arcantry.toml`, `.local/AGENTS.md`, and the local Git exclusion. It does not create package manifests, runtime configuration, task runners, OpenSpec sources, changelogs, or todo files.

Use the same explicit scope with `repo update` and `repo remove`. Removal is limited to verified Arcantry-owned configuration and managed guidance.

## Choose source responsibility

Choose `ignore`, `observe`, `validate`, or `manage` independently for each source. A configured capability does not authorize an external write.

Choose an explicit transition when structure must change:

- `preserve`: leave data and responsibility unchanged.
- `adopt`: manage an existing source in place.
- `rebind`: connect the role to another existing source.
- `cutover`: preserve earlier history and manage only a new boundary.
- `migrate`: convert only meaning that can be recovered without guessing.
- `relocate`: copy and verify a target before a separately planned deletion.

## Plan and apply

1. Run `arcantry repo plan --source <id> --transition <strategy> --json`.
2. Review conflicts, input hashes, visibility, adapter versions, and ordered operations.
3. Apply only with explicit user authorization using `arcantry repo apply --plan <path|->`.
4. If an input changed after planning, inspect again and create a new plan.

Todo commands are previews unless `--apply` is present. When both shared and private queues exist, select the source explicitly. Do not invent workflow tags.

## Verify

1. Run `arcantry repo inspect` again.
2. Run `arcantry repo validate` and, when repair guidance is useful, `arcantry repo doctor`.
3. Confirm private content did not enter previews, logs, tracked files, or packages.
4. Confirm unrelated project files and native build tooling remain unchanged.
5. Report applied transitions, preserved boundaries, conflicts, and checks not run.

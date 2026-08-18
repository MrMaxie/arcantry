---
name: adopt-arcantry
description: Inspect, plan, apply, validate, or remove an Arcantry project knowledge stack while preserving project-owned sources, private state, and native tooling. Use for configured or configuration-free adoption in existing repositories, new projects, monorepos, or directories without Git.
---

# Adopt Arcantry

Treat greenfield and brownfield as properties of each discovered source, not as product modes. Preserve project-owned content and stop when a plan reports a conflict.

## Inspect

1. Read the applicable repository and private instructions.
2. Run `arcantry repo inspect` before proposing changes.
3. Identify each OpenSpec, changelog, and todo.txt source, including its adapter, visibility, management level, and dependencies.
4. Treat `.local/` as a privacy boundary. Do not classify or manage `.docs/`.

Use `--config <path>` only when the user supplies or authorizes an explicit configuration. Otherwise accept the nearest `arcantry.toml` or configuration-free discovery. Do not merge configuration layers.

## Choose responsibility

Choose `ignore`, `observe`, `validate`, or `manage` independently for each source. A configured capability does not authorize an external write.

Choose an explicit transition when structure must change:

- `preserve`: leave data and responsibility unchanged.
- `adopt`: manage an existing source in place.
- `rebind`: connect the role to another existing source.
- `cutover`: preserve earlier history and manage only a new boundary.
- `migrate`: convert only meaning that can be recovered without guessing.
- `relocate`: copy and verify a target before a separately planned deletion.

Never migrate automatically after an Arcantry or adapter update.

## Plan and apply

1. Run `arcantry repo plan --source <id> --transition <strategy> --json`.
2. Review conflicts, input hashes, visibility, adapter versions, and ordered operations.
3. Apply only with explicit user authorization using `arcantry repo apply --plan <path|->`.
4. If an input changed after planning, inspect again and create a new plan.

Todo commands are previews unless `--apply` is present. When both root and private queues exist, select the source explicitly. Do not invent inbox or outbox tags.

Use legacy `arcantry repo init --docs none`, `update`, and `remove` only for an existing legacy contract. Never select `shared` or `local`; existing `.docs/` content remains project-owned.

## Verify

1. Run `arcantry repo inspect` again.
2. Run `arcantry repo validate` and, when repair guidance is useful, `arcantry repo doctor`.
3. Confirm private content did not enter previews, logs, tracked files, or packages.
4. Confirm unrelated project files and native build tooling remain unchanged.
5. Report applied transitions, preserved boundaries, conflicts, and checks not run.

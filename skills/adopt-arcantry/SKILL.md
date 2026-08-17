---
name: adopt-arcantry
description: Integrate Arcantry into an existing Git repository through the Arcantry CLI while preserving project instructions, existing OpenSpec history, private local state, and user-owned tooling. Use when Codex needs to initialize, update, validate, diagnose, or remove Arcantry-managed repository artifacts safely.
---

# Adopt Arcantry

Integrate Arcantry through its CLI. Preserve repository-owned files and stop when an operation reports a conflict.

## Inspect

1. Read the applicable `AGENTS.md` and private operational instructions.
2. Inspect Git status and existing `.local/arcantry.json`, `openspec/`, `justfile`, and `mise.toml`.
3. Run `arcantry repo doctor` when an installation already exists.
4. Determine the requested agents, operational source, ordered sources, and documentation mode.

## Require explicit choices

Before initialization, obtain an explicit `docs` choice:

- `shared`: keep reusable non-specification knowledge in tracked `.docs/`.
- `local`: keep `.docs/` private through `.git/info/exclude`.
- `none`: do not create `.docs/`.

Default `operationalSource` to `local` only when the user did not select another configured source. Never infer an external write authorization from a `readwrite` or `operational` source mode.

## Apply

1. Run `arcantry repo init --docs <shared|local|none>` for a new adoption. Add repeatable `--agent <codex|claude|cursor>`, `--source <name=readonly|readwrite|operational>`, and `--operational-source <name>` flags only for explicitly selected values. Use global `--cwd <path>` when targeting another repository.
2. When no agent or source is supplied, accept the CLI defaults of `codex` and `local=operational`. Use `arcantry repo update` for an existing adoption; it must preserve the complete existing source configuration.
3. Let the CLI create only missing managed artifacts.
4. Stop and report any conflict involving an existing `justfile`, `mise.toml`, incompatible OpenSpec layout, or user-owned file.
5. Do not replace a regular directory, remove an unknown artifact, or edit project instructions unless the user explicitly authorizes that exact change.

Use `arcantry repo remove` only when the user explicitly requests removal. It may remove only artifacts recognized as Arcantry-generated; preserve specifications, operational data, and unknown files.

## Verify

1. Run `arcantry repo validate`.
2. Run `arcantry repo doctor`.
3. Inspect Git status and confirm `.local/` remains private.
4. Report created or updated behavior, conflicts, and checks that were not run.

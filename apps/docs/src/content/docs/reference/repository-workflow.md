---
title: Inspect, plan and apply
description: Change one project knowledge source through a drift-safe, explicit plan.
---

Use this flow when a source must be adopted, rebound, cut over, migrated, or relocated. Inspection and planning are read-only. Application is the separate mutation boundary.

## Inspect the current stack

```sh
arcantry repo inspect
```

Inspection reports the resolved root, configuration mode, source ids, paths, kinds, adapters, visibility, management, existence, and compatibility. Add `--json` for the complete machine-readable result.

Use the reported source id in the next step. Inspection does not create a configuration or source.

<!-- cli-evidence: inspect-read-only -->

## Plan one transition

```sh
arcantry repo plan --source history --transition cutover --managed-from 1.0.0
```

Available transitions are:

| Transition | Purpose |
| --- | --- |
| `preserve` | Keep the current source and ownership boundary. |
| `adopt` | Begin using or managing a source at its current path. |
| `rebind` | Change the responsibility or adapter relationship without moving the source. |
| `cutover` | Preserve earlier changelog history and manage only from a SemVer boundary. |
| `migrate` | Convert recoverable source meaning through an explicit adapter transition. |
| `relocate` | Write a verified target and optionally remove the verified source. |

Use `--to-path`, `--to-adapter`, `--managed-from`, and `--delete-source` only when the selected transition needs them. Planning reports conflicts and does not write.

<!-- cli-evidence: plan-read-only -->

## Serialize and protect the plan

`repo apply` accepts the complete JSON plan produced by `--json`:

```sh
arcantry repo plan --source history --transition cutover --managed-from 1.0.0 --json > plan.json
```

The plan records its format version, Arcantry version, project root, source and adapter ids, expected input hashes, ordered operations, desired write content, content hashes, visibility, notes, and conflicts. Because write content can include private source data, store the plan at the sensitivity of the most private operation. A plan may live outside the target project and does not need to become repository metadata.

## Apply an unchanged plan

```sh
arcantry repo apply --plan plan.json
```

Use `--plan -` to read JSON from standard input. Before writing, apply resolves the current project again and verifies the plan root, format, exact Arcantry version, conflicts, path authority, expected path hashes, and planned content hashes. Operations inside the project root need no extra flag. Repeat `--allow-outside <path>` for each exact external operation path; permission for one path does not include its parent, children, or siblings. Apply stages and verifies writes, commits the ordered operations, verifies the targets, and rolls back committed operations if the transaction fails before final cleanup. A cleanup failure after the committed result is verified remains a successful apply and is reported as a warning.

For relocation with deletion, the target is staged and verified before the separately requested source deletion is committed.

## Replan after drift

If a source changes after planning or during apply, Arcantry rejects the plan before accepting the changed state. Do not edit hash fields or reuse the stale plan. Run `repo inspect`, recreate the plan from current inputs, and apply the replacement.

<!-- cli-evidence: apply-rejects-drift -->

An Arcantry update can also invalidate a serialized plan because plans bind to the exact tool version. Replan with the version that will perform the apply.

## Validate without repair

```sh
arcantry repo validate
arcantry repo doctor
```

`repo validate` checks configured `validate` and `manage` responsibilities without changing a source. `repo doctor` adds repair guidance but remains read-only. Neither command upgrades adapters or applies a plan.

External trackers, pull requests, and design tools may provide context, but source configuration and skills do not grant permission to modify those systems.

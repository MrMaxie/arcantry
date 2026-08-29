---
title: todo.txt queues
description: Preview and apply changes to shared or private todo.txt queues.
---

Arcantry treats todo.txt as a short work queue, not as product authority or release history. It recognizes a shared root `todo.txt` and a private `.local/todo.txt` independently.

## List queues

List every detected queue:

```sh
arcantry todo list
```

Select one configured source id or use the standard `root` and `local` aliases:

```sh
arcantry todo list --source root
arcantry todo list --source local
```

## Preview before writing

Todo mutations produce a plan by default. They do not write until `--apply` is present.

<!-- cli-evidence: todo-preview-first -->

```sh
arcantry todo add "Review adapter +Arcantry @desk" --source root
arcantry todo add "Review private notes @desk" --source local
```

Review the reported operations, then repeat the command with `--apply`:

```sh
arcantry todo add "Review adapter +Arcantry @desk" --source root --apply
```

An explicitly selected missing root, private, or configured source can be planned and created by `todo add`. Listing and previewing remain non-mutating; only `--apply` writes the file.

When both queues exist, a mutating command without `--source` fails rather than guessing.

## Complete a task

Line numbers come from `todo list`. Completion uses the local date unless `--date` supplies an ISO date.

```sh
arcantry todo complete 3 --source root
arcantry todo complete 3 --source root --date 2026-08-18 --apply
```

## Move between shared and private queues

Moving a task always names both endpoints. It does not require or add inbox, outbox, project, or context tags.

```sh
arcantry todo move 2 --from root --to local
arcantry todo move 2 --from root --to local --apply
```

The applied move stages both queue updates as one plan. When the target is private, Arcantry also plans the relevant `.local` Git exclusion if Git is present.

## Use the official todo.txt baseline

Arcantry uses the [official todo.txt format](https://github.com/todotxt/todo.txt) as its baseline. Each task occupies one non-empty physical line. Priority, creation date, `+project`, `@context`, and `key:value` metadata are optional:

```text
Review adapter
(A) 2026-08-19 Review adapter +Arcantry @desk
2026-08-19 Review adapter owner:Maxie
x 2026-08-20 2026-08-19 Review adapter +Arcantry @desk
```

Completed tasks place lowercase `x` first, followed by the completion date, an optional creation date, and the task text. When a project or selected source defines a compatible convention, follow it; otherwise, use this baseline without inventing optional fields.

Arcantry updates existing queues incrementally. `todo add` appends one task, `todo move` preserves the selected raw line, and `todo complete` rewrites only the selected task into completed-task order. Untouched lines, line endings, a byte-order mark, trailing-newline state, and extensions remain unchanged. Older or noncanonical entries do not block a scoped change and are not normalized implicitly.

Arcantry does not turn the queue into an OpenSpec authority, judge task quality, or infer a workflow taxonomy.

Use [Configuration](/arcantry/reference/configuration/) to place additional todo.txt sources or assign explicit management levels.

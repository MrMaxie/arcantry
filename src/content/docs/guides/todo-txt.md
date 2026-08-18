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

```sh
arcantry todo add "Review adapter +Arcantry @desk" --source root
arcantry todo add "Review private notes @desk" --source local
```

Review the reported operations, then repeat the command with `--apply`:

```sh
arcantry todo add "Review adapter +Arcantry @desk" --source root --apply
```

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

## Preserve the todo.txt format

Arcantry preserves untouched lines, line endings, a byte-order mark, completion dates, priorities, arbitrary `+project` and `@context` tags, and `key:value` metadata. It does not turn the queue into an OpenSpec authority or infer a workflow taxonomy.

Use [Configuration](/arcantry/reference/configuration/) to place additional todo.txt sources or assign explicit management levels.

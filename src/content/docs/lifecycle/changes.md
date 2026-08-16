---
title: Changes
description: The OpenSpec change is Arcantry's unit of intent.
---

A change should describe one coherent product or engineering outcome. It can require many commits; those commits do not become separate release entries.

## Required artifacts

Arcantry extends the normal spec-driven flow with `release.md`:

```text
proposal ─┬─> specs ──> tasks
          ├─> design ──> tasks
          └─> release
```

`release.md` records the release-facing interpretation of the change while the proposal and specs retain the deeper rationale and behavioral contract.

## Archive means delivered

Only archived changes are eligible for a release. OpenSpec moves completed changes into `openspec/changes/archive/YYYY-MM-DD-<change>/` and preserves their artifacts. Arcantry deliberately uses that boundary instead of trying to infer completion from merged commits.

## Internal changes

A change may be marked `internal`. It remains in the OpenSpec archive and can affect SemVer when appropriate, but it is omitted from the public changelog body.

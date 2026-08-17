---
title: Changes
description: The OpenSpec change is Arcantry's unit of intent.
---

A change should describe one coherent product or engineering outcome. It can require many commits; those commits do not become separate release entries.

Every completed product or engineering change requires an OpenSpec record. The record may be written before implementation or recovered afterward, but the repository cannot be treated as complete until the change is specified, verified, archived and assigned to a version.

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

If implementation came first, create the same complete artifact set before archiving it. Postfactum recovery is not a commit-derived changelog entry and does not reduce the specification or verification requirements.

## Internal changes

A change may be marked `internal`. It remains in the OpenSpec archive, must be assigned to a SemVer release and is omitted only from the public changelog body.

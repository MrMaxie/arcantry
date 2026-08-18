---
title: Change lifecycle
description: Define and archive product or engineering changes made to Arcantry itself.
---

This page describes contribution rules for the Arcantry repository. Projects that use the CLI may choose their own delivery lifecycle; adopting Arcantry does not make OpenSpec mandatory for them.

Inside Arcantry, one OpenSpec change describes one coherent product or engineering outcome. It can require many commits; those commits do not become separate release entries.

Every completed Arcantry product or engineering change requires an OpenSpec record. The record may be written before implementation or recovered afterward, but Arcantry cannot treat its own repository state as complete until the change is specified, verified, archived, and assigned to a version.

## Required artifacts

Arcantry extends the normal spec-driven flow with `release.md`:

```text
proposal -+-> specs --> tasks
          +-> design --> tasks
          +-> release
```

`release.md` records the release-facing interpretation of the Arcantry change while the proposal and specs retain the deeper rationale and behavioral contract.

## Archive means delivered

Only archived Arcantry changes are eligible for an Arcantry release. OpenSpec moves completed changes into `openspec/changes/archive/YYYY-MM-DD-<change>/` and preserves their artifacts. Arcantry uses that boundary instead of inferring completion from merged commits.

If implementation came first, create the same complete artifact set before archiving it. Postfactum recovery is not a commit-derived changelog entry and does not reduce the specification or verification requirements.

## Internal changes

An Arcantry change may be marked `internal`. It remains in the OpenSpec archive, must be assigned to a SemVer release, and is omitted only from the public changelog body.

For the general relationship between project authorities and changelog projections, see [Project knowledge stack](/arcantry/reference/repository-contract/#relationships-and-meaning).

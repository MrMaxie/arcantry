---
title: Releases
description: Build version numbers and changelog entries from delivered OpenSpec changes.
---

A release is a named set of archived OpenSpec changes.

## `release.md`

Each change carries a small release artifact:

```md
---
category: changed
impact: minor
visibility: public
---

# Configurable farm layout

The farm layout can now be rebuilt without replacing the map wholesale.
```

The frontmatter is machine-readable. The body is already suitable for release notes and should describe the delivered outcome, not implementation steps.

### Category

`added`, `changed`, `fixed`, `deprecated`, `removed` or `security`.

### Impact

`none`, `patch`, `minor` or `major`. The release planner chooses the highest impact among included changes.

### Visibility

`public` includes the entry in `CHANGELOG.md`; `internal` keeps it in the delivery record without publishing the prose.

## Release manifest

A manifest only groups change IDs under a version. It does not copy descriptions from the changes.

```yaml
version: 1.4.0
changes:
  - configurable-farm-layout
  - fix-save-corruption
```

The generator resolves each ID against `openspec/changes/archive/`, verifies `release.md`, and renders the changelog.

## Why commits are not inputs

A single change may contain exploratory commits, refactors, test fixes and implementation corrections. Those are valuable Git history, but aggregating them produces an implementation log rather than a changelog. Arcantry keeps the two histories intentionally separate.

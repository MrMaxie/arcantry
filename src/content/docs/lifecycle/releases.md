---
title: Releases
description: Build version numbers and changelog entries from delivered OpenSpec changes.
---

A release is a dated set of archived OpenSpec changes.

## `release.md`

Each change carries the release-facing outcome and its version impact:

```md
---
category: changed
impact: minor
visibility: public
components:
  - repository-lifecycle
---

# Configurable farm layout

The farm layout can now be rebuilt without replacing the map wholesale.
```

The body describes the delivered outcome. It is not an implementation summary.

`category` is one of `added`, `changed`, `fixed`, `deprecated`, `removed` or `security`.

`impact` is `none`, `patch`, `minor` or `major`. Planning uses the highest impact among unassigned archived changes.

`visibility: public` publishes the entry in `CHANGELOG.md`. `internal` keeps the change in release state without publishing its prose.

`components` lists one or more stable affected surfaces, such as `cli`, `catalog`, `repository-adoption` or `skill:<name>`.

## Manifest

A release manifest contains only identity and grouping data:

```yaml
version: 1.4.0
date: 2026-08-16
changes:
  - configurable-farm-layout
  - fix-save-corruption
```

Change IDs must resolve to `openspec/changes/archive/` and may belong to only one release.

## Flow

```text
OpenSpec change
    ↓
implementation + verification
    ↓
archive
    ↓
just release-plan
    ↓
just release-cut
    ↓
just release-render
```

`release-cut` creates the manifest from the computed plan. It does not ask commits what changed.

`release-check` validates the complete release state and compares the committed `CHANGELOG.md` with an in-memory render. `just check` runs it automatically.

## Changelog provenance

Published entries carry an invisible OpenSpec source marker:

```html
<!-- openspec: configurable-farm-layout -->
```

This keeps generated output traceable without adding noise for readers.

## Git history

Commits remain the implementation audit trail. A change can contain exploratory commits, refactors, test fixes and corrections without any of them becoming release notes. The archived OpenSpec change is the release unit.

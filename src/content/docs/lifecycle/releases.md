---
title: Releases
description: Build version numbers and changelog entries from delivered OpenSpec changes.
---

A release is a dated set of archived OpenSpec changes. It is required to complete repository work even when no package, Git tag or GitHub Release is published.

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

New completed changes use `patch`, `minor` or `major`. Planning rejects an unassigned `none` change and uses the highest impact among the remaining changes. Historical release data may retain `none` for compatibility.

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
    |
    v
implementation + verification
    |
    v
archive
    |
    v
just release-plan
    |
    v
just release-cut
    |
    v
just release-render
    |
    v
align package and plugin versions
    |
    v
commit the complete release state
    |
    v
just ci
```

`release-cut` creates the manifest from the computed plan. It does not ask commits what changed.

`release-check` rejects active changes, unassigned archives, distribution version drift, stale generated changelog content, uncommitted work and commits after the latest release manifest. `just check` runs it automatically.

The commit that introduces the newest release manifest is the release seal. It must also contain the archived OpenSpec changes, aligned distribution versions and generated changelog. A later commit opens a new change cycle and requires a newer internal release before the repository can pass final validation again.

## Changelog provenance

Published entries carry an invisible OpenSpec source marker:

```html
<!-- openspec: configurable-farm-layout -->
```

This keeps generated output traceable without adding noise for readers.

## Git history

Commits remain the implementation audit trail. A change can contain exploratory commits, refactors, test fixes and corrections without any of them becoming release notes. The archived OpenSpec change is the release unit.

Release validation uses Git only to prove that no repository work follows the newest release seal. It never derives release prose, category, impact, visibility or components from a commit message or diff.

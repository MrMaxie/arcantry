---
name: maintain-release-story
description: Align accepted OpenSpec outcomes, project version conventions, distributable versions, and consumer-facing changelog prose when planning, preparing, or auditing a release without publishing it.
---

# Maintain Release Story

Describe what consumers gain, lose, or must account for. Git commits are implementation evidence, not release meaning.

## Workflow

1. Read the configured adapter, topology, selected unit, schema-classified changes, latest unit manifest, version sources, and changelog.
2. Before drafting any changelog or provider change list, build a source-to-outcome ledger. Decompose each matching release-bearing change into independently meaningful consumer outcomes, assign every outcome its truthful category, and record its visibility, components, prose, and justified global or unit impact. Treat OpenSpec boundaries as provenance and release-consumption boundaries, not output-item boundaries: one change may produce multiple entries or categories; multiple changes may share an entry only when they describe the same consumer outcome. Skip only schemas without release artifacts.
3. Use the configured version strategy: SemVer impact, monotonic integer, or calendar plus daily revision. Respect package version constraints. Use the changelog preset or the project template, preserving earlier history. For composed units, report pins and require a parent outcome to acknowledge adopted direct dependencies; never auto-bump parents.
4. Preview one unit set: change IDs, version, dependency pins, version updates, one changelog entry or requested list item per ledger outcome under its truthful category, and each requested publication artifact separately. For copy-paste-ready text, use the destination's established format, return only the requested artifact, and wrap the literal payload in a fenced code block with the appropriate language tag unless the user explicitly requests rendered prose; keep repository changelog entries, public release notes, and provider-specific change lists distinct, preserve a requested list as one consumer outcome per item, and add a heading only when the destination requires it or the user asks.
5. Apply only the requested preparation step. Tagging, publishing, pushing, and external announcements require separate explicit authority.
6. Validate ownership, versions, assignments, pins, changelog, packages, and repository status. Unscoped checks cover every unit.

Exclude internal refactors and commit narration unless consumers are affected. Do not invent history. Keep shared prose per outcome; split materially different consumer stories.

Never derive shared changelogs from private OpenSpec. Private changelogs may use configured shared and private sources. Independent units have no root release; parents do not copy child entries.

Implementation, commit, deployment and release are separate decisions. A missing release does not block ordinary work. Existing authorization remains valid; do not request it again. Without the CLI, read configuration, schema templates, manifests and source files directly and show the exact proposed edits.

---
name: maintain-release-story
description: Align accepted OpenSpec outcomes, unit-aware SemVer, distributable versions, and consumer-facing changelog prose when planning, preparing, or auditing a release without publishing it.
---

# Maintain Release Story

Describe what consumers gain, lose, or must account for. Git commits are implementation evidence, not release meaning.

## Workflow

1. Read the configured adapter, topology, selected unit, schema-classified changes, latest unit manifest, version sources, and changelog.
2. Confirm each matching release-bearing change has visibility, components, consumer prose, and justified global or unit impact. Skip only schemas without release artifacts.
3. Compute the unit version from effective impacts. For composed units, report pins and require a parent outcome to acknowledge adopted direct dependencies; never auto-bump parents.
4. Preview one unit set: change IDs, version, dependency pins, version updates, changelog categories, and each requested publication artifact separately. For copy-paste-ready text, use the destination's established format, return only the requested artifact, and wrap the literal payload in a fenced code block with the appropriate language tag unless the user explicitly requests rendered prose; keep repository changelog entries, public release notes, and provider-specific change lists distinct, preserve a requested list as one consumer outcome per item, and add a heading only when the destination requires it or the user asks.
5. Apply only the requested preparation step. Tagging, publishing, pushing, and external announcements require separate explicit authority.
6. Validate ownership, versions, assignments, pins, changelog, packages, and repository status. Unscoped checks cover every unit.

Exclude internal refactors and commit narration unless consumers are affected. Do not invent history. Keep shared prose per outcome; split materially different consumer stories.

Never derive shared changelogs from private OpenSpec. Private changelogs may use configured shared and private sources. Independent units have no root release; parents do not copy child entries.

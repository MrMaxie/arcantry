---
name: maintain-release-story
description: Align accepted OpenSpec outcomes, SemVer, distributable versions, and consumer-facing changelog prose when planning, preparing, or auditing a release without publishing it.
---

# Maintain Release Story

Describe what consumers gain, lose, or must account for. Git commits are implementation evidence, not release meaning.

## Workflow

1. Read the active OpenSpec changes, their release artifacts, the latest release manifest, native package manifests, and the configured shared or private changelog.
2. Confirm that every releasable change has visibility, affected components, a consumer-facing title, and justified SemVer impact.
3. Determine the next version from the highest included impact and the current release manifest.
4. Preview one release set: included change IDs, version, native version updates, and changelog categories derived from the release artifacts.
5. Apply only the requested preparation step. Tagging, publishing, pushing, and external announcements require separate explicit authority.
6. Validate exact version alignment, complete change coverage, changelog formatting, package contents, and repository status.

Keep internal refactors and commit narration out of public release text unless they materially change a consumer outcome. Do not invent historical entries when the source release artifacts do not establish them.

Never derive a shared changelog entry from private OpenSpec. A private changelog may compose private and shared release intent when its configuration records those dependencies.

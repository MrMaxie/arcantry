---
name: intake-repository-work
description: Capture issue, pull-request, review, and CI work in repository-local operational artifacts while preserving source provenance and project conventions. Use when `.local/arcantry.json` selects `local` as the operational source or when the user asks to normalize repository work into private local state.
---

# Intake Repository Work

Capture executable work in private repository-local artifacts. Treat external providers as supporting sources, not automatic write targets.

## Inspect

1. Read applicable project and private operational instructions.
2. Read `.local/arcantry.json` and confirm `operationalSource` is `local`.
3. Inspect existing `.local/issues/`, `.local/issues-notes/`, `.local/reviews/`, and `.local/pr-notes/` conventions.
4. Inspect `.docs/templates/` when shared templates exist.
5. Classify the work as issue, pull request, review, CI, or change-note intake.

## Capture

Prefer established repository templates. Otherwise use the matching bundled asset:

- `assets/issue.md` for the executable request and acceptance criteria.
- `assets/issue-notes.md` for progress, decisions, outputs, and the resume point.
- `assets/review.md` and `assets/review-answers.md` for revision-scoped review work.
- `assets/pr-notes.md` for user-facing change notes.

Use stable IDs already present in the repository or supplied by the user. For review revisions, continue the repository's numbering convention rather than replacing earlier review artifacts.

## Source boundaries

- Record the exact source system, ID, URL, and freshness when work came from a remote system.
- Read remote context only when requested, required by project instructions, or necessary to fill a blocking gap.
- Do not overwrite local operational truth merely because a remote source differs; record the discrepancy and its freshness.
- Do not write to any external provider without current authorization for the exact target and action.
- Treat `readwrite` configuration as capability metadata, never as authorization.

## Protect local state

- Keep operational artifacts under `.local/` and ensure `.local` is listed in `.git/info/exclude`.
- Keep reusable non-specification knowledge in `.docs/` only when the repository selected `shared` or `local` documentation.
- Keep specifications and release history in `openspec/`; do not place specifications under `.docs/`.
- Preserve existing artifacts and templates. Ask only when unresolved ambiguity would materially change the captured task.

## Verify

Re-read the created or updated artifact, confirm source fields and acceptance criteria, inspect Git status, and report which remote sources were refreshed.

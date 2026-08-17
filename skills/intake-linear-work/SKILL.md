---
name: intake-linear-work
description: Capture issue, pull-request, review, CI, and project work in Linear while preserving remote-source provenance and explicit write authorization. Use when `.local/arcantry.json` selects Linear as the operational source or when the user explicitly asks to create or update Linear operational state.
---

# Intake Linear Work

Normalize operational work into the configured Linear team or project. Keep private configuration local and require current authorization before every mutation.

## Inspect

1. Read applicable project and private operational instructions.
2. Read `.local/arcantry.json` and resolve the configured Linear source, team, project, and source policy.
3. Inspect the target Linear project, resources, existing issues, documents, and comments.
4. Classify the work as local, self-generated, remote ticket, remote pull-request review, change note, review answer, or CI work.
5. Fetch another provider only when requested, required by project policy, or necessary to fill a blocking gap.

Stop before a Linear write when the connector is unavailable, the exact target cannot be resolved, or the user has not authorized that mutation. Do not treat `readwrite` or `operational` mode as authorization.

## Capture

Use the matching bundled asset:

- `assets/linear-issue.md` for issues and their source provenance.
- `assets/progress-comment.md` for material status transitions and resume points.
- `assets/remote-ticket.md` for a faithful remote-ticket clone.
- `assets/remote-pr-review.md` for review or comment intake.
- `assets/pr-note-proposal.md` for owner-addressed change-note proposals.

Resolve the current Linear profile before publishing an owner-addressed proposal. Derive the mention from the current profile; never guess, cache, or hardcode it.

## Source and write policy

- Treat Linear as canonical only for this configured operational workflow.
- Keep `.local/` for private configuration and machine-local context, not duplicated issue state.
- Record exact source system, source ID, source URL, and capture freshness for remote-derived work.
- Keep other external systems read-only unless the user separately authorizes an exact target and write.
- Refresh remote-derived work before an external reply, closure, or decision that depends on current source state.
- Do not assign a person, change status, publish a comment, or create an issue without authorization covering that action.

## Write for Linear readers

Write concise English artifacts centered on outcome, scope, acceptance criteria, current state, and the next action. Keep local paths, private configuration, workflow names, and diagnostic dumps out of Linear.

## Verify

Re-read the resulting issue, document, or comment; confirm project placement, provenance, links, and current status. Report the exact mutations made and any source that was not refreshed.


---
name: maintain-agent-guidance
description: Audit and update universal AGENTS.md guidance at the narrowest user, repository, nested, or private scope through exact approval-gated proposals.
---

# Maintain Agent Guidance

Put each rule at the narrowest scope that must inherit it. Guidance changes affect future context on every matching task, so prefer replacement, consolidation, or deletion over accumulating reminders.

## Scope model

- User guidance applies across unrelated projects for one user.
- Repository guidance applies to every contributor and agent in one repository.
- Nested guidance applies only below a specific directory.
- `.local/AGENTS.md` holds private workstation or personal workflow rules and must remain untracked.

Treat `AGENTS.md` and `.local/AGENTS.md` as the universal sources. Do not create independent branded copies. When Claude Code compatibility is explicitly requested, prefer a managed `CLAUDE.md` import of `@AGENTS.md` or a locally excluded `CLAUDE.local.md` import of `@.local/AGENTS.md`; preserve any surrounding Claude-specific content.

Do not move private paths, credentials, personal service setup, or machine-specific values into shared guidance.

## Workflow

1. Read the applicable guidance chain and the evidence for the requested change.
2. Identify overlaps, conflicts, stale facts, and the exact decision point the rule must influence.
3. Draft independently numbered proposals. Each proposal must state the audience, target, exact add, replace, or delete diff, expected behavior, and context-cost tradeoff.
4. Wait for item-level approval. Apply only accepted proposal IDs and do not bundle adjacent cleanup.
5. Re-read each target before editing. If its preimage changed, revise the proposal instead of applying a stale diff.
6. Validate nesting, language, commands, privacy boundaries, managed-section ownership, and any requested compatibility import.
7. Report applied IDs and leave rejected or undecided proposals unchanged.

If `.local` is used in a Git repository, verify that `.local/` is present in `.git/info/exclude`. Do not edit `.gitignore` for private agent state unless the user explicitly requests a shared policy.

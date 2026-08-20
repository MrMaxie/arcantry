---
name: reconcile-project-sources
description: Explain drift between shared and private OpenSpec, changelog, or todo sources and prepare one explicit transition that preserves both until apply.
---

# Reconcile Project Sources

Treat every configured source as independent. Reconciliation is an explicit reviewed transition, not background synchronization.

Use `protect-local-boundary` whenever an inspected source is under `.local/` or a transition crosses the private boundary. It owns filesystem handling and promotion safety; this skill owns semantic reconciliation.

## Workflow

1. Run `arcantry repo inspect` and identify the active configuration, shadowed configuration, source scopes, management levels, adapters, and dependencies.
2. Compare only sources the user placed in scope. Summarize semantic differences without copying private content into shared output.
3. Choose one action per source: preserve, adopt, rebind, cutover, migrate, or relocate.
4. Generate a serializable plan with `arcantry repo plan --source <id> --transition <action> --json` when the CLI supports the transition.
5. Present source and target, retained history, input hashes, planned writes, separately requested deletions, and privacy impact.
6. Apply only after explicit authorization. If any input changes, discard the plan and inspect again.
7. Verify the target before any authorized source deletion and re-run inspection after applying.

Never merge two configurations, mirror content automatically, or treat a newer timestamp as semantic authority. Do not reveal private source content merely to prove that drift exists.

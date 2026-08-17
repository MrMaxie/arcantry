# Decision Rules

## Evidence precedence

Use this order without collapsing the underlying fields:

1. A matching path rule in `skills.config` takes precedence over a matching name rule.
2. A conflicting set of matching config rules is a finding; use the last rule from the higher-precedence selector only for the effective-state estimate.
3. A skill under a supplied repository `.agents/skills` root is repo-local regardless of invocation policy.
4. A global skill with `allow_implicit_invocation: false` is global explicit.
5. A global enabled skill with `true` or an absent policy is global implicit for catalog-pressure analysis.

Keep `scope`, `enabled`, `invocation`, `config_decision_source`, and `recommended_tier` as separate facts.

## Tier candidates

### Global implicit

Recommend only when the workflow is cross-project, commonly useful, safe to activate automatically, and distinguishable from neighboring skills within the visible prefix of its description.

### Global explicit

Prefer for request-only style modifiers, meta-workflows, rare operations, expensive or potentially disruptive workflows, and skills whose user intent must be explicit.

### Repo-local

Prefer for a single product, repository, framework installation, private workflow, or team convention. Do not move a skill merely because its name contains a framework; verify actual portability and ownership first.

### Disabled

Recommend only with evidence of an unused plugin, obsolete workflow, exact duplicate, superseded skill, invalid package, or a user decision. Similar descriptions alone are insufficient.

## Collision types

- `exact-name`: deterministic discovery ambiguity; inspect both paths.
- `exact-description`: deterministic metadata duplication; the bodies may still differ.
- `workflow-family`: heuristic; related phase-specific skills may be intentionally separate.
- `semantic-trigger-overlap`: heuristic based on shared meaningful terms and overlap coefficient; manually inspect triggers and near-miss cases.

For every heuristic collision, consider three outcomes: keep separate with sharper triggers, consolidate behind one router, or make one explicit-only. Do not assume consolidation is always better.

## Description review

The script defaults to a 110-character comparison budget because overloaded catalogs may shorten descriptions near that size. This is a diagnostic threshold, not an OpenAI product guarantee.

Review whether the first 100 characters establish:

1. the unique action;
2. the primary object or domain;
3. the strongest positive trigger;
4. a critical exclusion when it distinguishes a nearby skill.

Prefer shortening or replacing broad openings over appending more trigger prose.

## Human evidence required

Ask for or infer from approved local evidence before deciding:

- real usage frequency;
- whether a plugin is still wanted;
- whether a domain skill belongs to one repository;
- which overlapping workflow is canonical;
- whether a migration or consolidation cost is acceptable.


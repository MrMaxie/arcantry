---
name: evaluate-skill-change
description: Compare a baseline and candidate skill on synthetic routing and behavior cases. Use only when explicitly invoked with $evaluate-skill-change; do not edit, install, or automatically adopt the candidate.
---

# Evaluate Skill Change

Gate skill changes with repeatable evidence rather than accepting plausible prose.

## Inputs

Require:

- an exact baseline skill directory;
- an exact candidate skill directory or proposed patch applied only to a temporary copy;
- a JSONL case file following [references/evaluation-protocol.md](references/evaluation-protocol.md).

Use supplied generic cases or reusable cases colocated with the target skill.
Keep project-specific cases and run artifacts under `.local/self-improvement`.

## Workflow

1. Run `scripts/validate_cases.py <cases.jsonl>`.
2. Run `python scripts/resolve_quick_validate.py` to locate the validator owned by the installed `$skill-creator`, then run that exact `quick_validate.py` against baseline and candidate. If validation fails only because `yaml` or PyYAML is unavailable and `uv` exists, retry with `uv run --with pyyaml python <validator> <skill-directory>` without a global install. Report a controlled validation gap if neither path is available.
3. Confirm the candidate uses at most three independent edit groups. Record additions, replacements, and deletions separately.
4. Evaluate baseline and candidate with identical prompts in isolated fresh contexts. Keep at least three cases held out from candidate drafting.
5. Measure routing precision, routing recall, rubric pass rate, critical regressions, and skill-body size delta.
6. Apply the gate in the protocol and return exactly one decision: `accept`, `reject`, or `needs-review`.

## Safety

- Reject any new privacy, authorization, destructive-action, scope, or secret-handling regression.
- Reject a candidate that improves no failing case or degrades any held-out protected behavior.
- Return `needs-review` when evidence is incomplete, results are tied but behavior changed, or body size grows by more than 10 percent without an accepted reason.
- Do not reward instruction growth by itself.
- Do not edit, install, enable, commit, push, or publish the candidate.

## Output

Report case counts, baseline and candidate metrics, critical regressions, context delta, decision, and the minimum evidence supporting it. Keep raw model outputs only in the approved local run directory.

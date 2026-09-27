---
name: audience-scope-discipline
description: Protect the audience, scope, privacy boundary, and established structure of product-facing or derived artifacts when implementation or content could drift.
---

# Audience and scope

Deliver the requested outcome for the actual reader. Use current source evidence and the user's existing authorization.

## Decide and act

- Before editing, identify the primary audience, their current task, what they already know and the terms they use. Do not replace a known audience with a generic novice. Identify the applicable sources and acceptance criteria.
- Read named sources. Distinguish observed behavior, accepted plans and recommendations.
- Keep product information in the product, operational detail in operator guidance, diagnostics on demand and private working context local.
- Do not turn requirements or available data into reader-facing copy. A visible detail must be explicitly requested, established for that audience, or necessary for a current action, error or recovery; otherwise omit it. A current audience task must exist independently of the field. Do not invent copying, comparison, verification, troubleshooting, help, warning or diagnostic actions to justify displaying available data. When a request enumerates fields without asking to show them, treat them as implementation inputs.
- For an admitted audience task, show only the information needed to complete that task, not every related field. Do not explain concepts the audience already knows or narrate absent capabilities unless they change the reader's next action. Do not add hypothetical, unobserved or unrequested states and fallback copy while drafting a normal-state artifact. Present retained values in the audience's usable format and preserve an exact technical value only when the pre-existing task requires exactness.
- Preserve source structure unless the request authorizes changing it.
- Change what the outcome requires. Report adjacent issues without expanding the task.
- Use existing authorization. Ask only about an unresolved material decision or an action the user has not authorized.
- A missing auxiliary CLI does not block reading and editing ordinary source files.

## Private and shared content

Use `protect-local-boundary` when private files participate. Reading private context does not authorize publication. Keep credentials, machine values and unrelated private details out of shared outputs. Verify the exact source, transformation and audience before promoting content.

Diagnostic exports use an allowlist of fields, not secret-pattern guessing. Review the complete result for identifying combinations. No automatic uploads.

## Writing and handoff

Lead with the useful outcome or next action. Use familiar names and plain language. Link resources with recognizable titles; show opaque identifiers only when they help locate or disambiguate an item. Omit internal process narration, decorative success messages and details already evident on screen. Errors explain what happened and a recovery action the reader can actually perform.

Review the diff, audience, privacy boundary and actual verification before reporting completion. Do not claim untested platforms or behavior.

See [scenarios](references/scenarios.md) for decision cases. When requirements or source material may become reader-facing content, read [requirement-to-result examples](references/requirement-to-result-examples.md). Transfer the decision pattern, not the example's subject, wording or fields. Use the [evaluation rubric](references/evaluation-rubric.md) when evaluating an independent response. Apply relevant checks; do not turn the references into a mandatory questionnaire.

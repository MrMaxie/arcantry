# Decision Rules

Use this reference to decide whether a communication problem needs no durable edit, a project rule, a private local rule, or a global skill change.

## Contents

1. Evidence standard
2. Diagnosis matrix
3. Operational effectiveness test
4. Target selection
5. Global skill transfer test
6. Skill-use analysis
7. Proposal quality gate
8. Proposal format

## Evidence standard

Separate observed facts from interpretation.

Assign one primary diagnosis to each finding. When evidence supports two independent causes, split them into separate findings rather than assigning contradictory labels to one proposal.

Strong evidence includes:

- an explicit user constraint followed by a conflicting visible response or action;
- an explicit user correction that identifies the intended actor, scope, source, or decision rule;
- a relevant instruction that is proven to have applied at the time;
- repeated behavior across selected conversations;
- a tool or command result that directly explains the disputed outcome.

Current guidance, inferred intent, and conversation summaries can support a hypothesis but do not establish historical causation by themselves. Label uncertainty and instruction drift.

Call an inference "obvious" only when the expected conclusion followed from at least one of:

- an explicit instruction in the current or still-applicable earlier turn;
- an applicable higher-priority instruction;
- a project fact the agent could have discovered cheaply before asking;
- a stable user preference actually available to that session.

Otherwise classify the issue as ambiguity or missing context rather than a failure to guess.

## Diagnosis matrix

| Diagnosis | Test | Default response |
| --- | --- | --- |
| `execution_lapse` | One isolated failure occurred even though applicable guidance was available, actionable, actor-specific, correctly routed, and not in conflict; no non-repetitive setup change would improve reliability. | None. Report the lapse and evidence. Do not strengthen text solely through repetition. |
| `guidance_effectiveness_gap` | Applicable guidance is absent, ambiguous, conflicting, incorrectly placed or routed, or lacks an actionable trigger, actor, precedence rule, or decision checkpoint; a durable mechanism change could improve reliability. | Select the narrowest target separately and replace, relocate, route, or operationalize the rule. Do not merely duplicate it. |
| `interaction_ambiguity` | More than one material interpretation remained after reasonable inspection. | Usually none. Describe the agent-side disambiguation strategy. Offer prompt advice only when the user explicitly requests it. |
| `tool_or_platform_limit` | The desired behavior was blocked by the available interface, missing history, permissions, or truncation. | Usually none. State the limitation and supported fallback. |

Choose the diagnosis by failure mechanism. Then choose the target independently with `Target selection`. Project, local, repository, and global scope describe target layers; routing describes where the mechanism failed. Do not use either as a second diagnosis.

## Operational effectiveness test

Run this test before assigning `execution_lapse`:

1. **Availability:** Was the rule loaded or cheaply discoverable at the decision point?
2. **Trigger:** Did it say when it applies in terms the current task exposed?
3. **Actor and action:** Did it identify who must or must not act and what to do next? When producing a handoff, did it require checking the owner of every reported next action instead of restating an agent restriction as a user instruction?
4. **Precedence and routing:** Did source order, skill routing, or workflow structure lead the agent to the rule without a conflicting signal?
5. **Observed reliability:** Was the failure isolated, or did it repeat across conversations, recur after correction, or cause several related mistakes in one workflow?

Use `guidance_effectiveness_gap` when any of the first four checks fails or when repeated or cascading evidence shows that the current setup does not reliably produce the intended decision. One case is sufficient only when it exposes a structural mechanism gap and the exact proposal repairs that mechanism. A valid improvement changes the mechanism: replace ambiguous text, move it to the decision point, route the relevant skill, name the actor, define precedence, or add a small checklist or gate.

Use `execution_lapse` only when all first four checks pass, the failure is isolated, and every candidate durable change would merely repeat already effective guidance. Do not use the label to end analysis after noticing that a similar sentence exists.

User corrections are evidence, not instructions to defend the prior classification. Re-evaluate the finding when the user identifies a missed actor, broader scope, repeated behavior, or discoverable project fact.

## Target selection

Choose the narrowest audience that must inherit the rule:

1. **No file:** isolated execution lapse, one-off ambiguity, or tool limitation with no feasible durable improvement.
2. **Private local file:** machine path, private service setup, personal workflow, or locally available tools.
3. **Project instruction:** shared repository command, authorization boundary, source of truth, validation rule, or domain-specific routing.
4. **Repository skill:** reusable workflow that belongs only to one repository or product family.
5. **Global skill:** general procedure that should behave the same across unrelated projects.

Do not place a rule in a broader layer for convenience. Do not make `.local` shared. Do not add private values to any instruction file.

For a `guidance_effectiveness_gap`, prefer the layer that owns the failing decision point. Workstation login and private tool setup belong in `.local`; repository-specific task selection, acceptance scope, and action ownership belong in project guidance or a repository skill; cross-project workflow defects may belong in a global skill only after the transfer test.

Before naming a target, verify that it is an active control surface: authoritative for the audience, actually loaded or routed before the failing decision, and expected to remain in use. A transcript, brief, draft, or descriptive resource is evidence, not guidance, unless a current workflow consumes it at that decision point.

## Global skill transfer test

Before proposing a global edit, answer all of these:

1. What exact current sentence is absent, wrong, or ambiguous?
2. Would a correct agent following the current text still plausibly fail?
3. Does the rule remain correct in at least one contrasting project or task?
4. Would a project-local exception solve the observed case with less cross-project risk?
5. Can the fix replace or delete text instead of only adding more?

If answer 2 is no, do not edit the global skill. Run the operational effectiveness test and route any supported fix to the narrowest project or local layer; use `execution_lapse` only when that test also passes. If answer 3 is no or unknown, prefer a narrower layer and mark the global hypothesis as unproven. A single case may justify a global proposal only when it exposes an explicit contradiction or a clear safety-critical omission; label the evidence as single-case.

## Skill-use analysis

For each used or plausibly omitted skill:

- confirm that its description matched the task signal;
- distinguish explicit invocation from implicit selection;
- compare visible behavior with the actual rule, not with the skill's name;
- check whether a project router overrode or narrowed global behavior;
- avoid claiming that a skill caused an outcome when the transcript does not establish that it was loaded;
- treat current skill content as potentially newer than the analyzed conversation.

## Proposal quality gate

A proposal is ready only when it:

- fixes a demonstrated durable decision mechanism reusable within the selected audience, rather than incident wording or a stylistic preference;
- keeps incident-specific identifiers, transient metadata, and one-off examples in evidence unless they are stable concepts of the selected audience;
- uses `guidance_effectiveness_gap`, not `execution_lapse`, when the proposed diff changes a trigger, actor, placement, route, precedence rule, or decision checkpoint;
- names one logical target and one audience;
- contains an exact minimal diff; for a replacement, enumerates each independent obligation in the preimage as preserved, intentionally changed, or excluded from the proposal and therefore left intact;
- preserves safety, authorization, privacy, and scope boundaries unless changing that boundary is the proposal's explicit, evidenced purpose;
- explains why no narrower target is sufficient;
- identifies overlapping guidance before drafting and prefers consolidation, replacement, relocation, deletion, or no change; an additive diff must explain why no existing text can carry the behavior and state its net line effect;
- treats an explicit size or no-growth constraint as an acceptance criterion;
- states compatibility and overfitting risk;
- can be accepted or rejected independently.

Use confidence labels:

- **High:** direct evidence and applicable guidance are established.
- **Medium:** the pattern is credible but historical guidance or causation is incomplete.
- **Low:** the proposal is exploratory; do not recommend applying it without more evidence.

## Proposal format

```text
P1 — <short outcome>
Status: proposed
Evidence: <minimal quote or observable event>
Impact: <what went wrong>
Diagnosis: <category>, <confidence>
Audience: <who inherits the rule>
Target: <exact file>
Why here: <why broader and narrower alternatives are wrong>
Risk: <overfitting, compatibility, or drift risk>
Preservation: <for replacements, independent obligations preserved or intentionally changed>
Diff:
<small exact diff>
Decision: accept P1 | revise P1: ... | reject P1
```

For a no-change finding, omit `Target` and `Diff`, list the candidate durable interventions considered, and state why each would be redundant, harmful, unsupported, or over-broad. Do not assign it a proposal ID unless the user needs to decide a non-file workflow change.


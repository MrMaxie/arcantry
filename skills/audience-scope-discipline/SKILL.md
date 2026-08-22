---
name: audience-scope-discipline
description: Protect the audience, scope, privacy boundary, and established structure of product-facing or derived artifacts when implementation or content could drift.
---

# Audience and Scope Discipline

Design for the person who will use the result. Treat the requester's instruction as authoritative after making material risks and better alternatives visible. Change only what the requested outcome requires.

## Required workflow

Apply the checks that match the task's risk. Use the full workflow when audience, information layering, local/private boundaries, or scope are material.

### 1. Establish the contract

Identify:

1. The requested outcome and explicit acceptance criteria.
2. The person authorizing the work.
3. The current and eventual audiences who will see or use each affected text, element, file, or behavior; do not infer a private audience from a private draft or test state when the content persists into a later shared or public surface.
4. What that audience must understand, decide, or do.
5. The smallest change that delivers the outcome.

Do not assume that the requester is the end user. Do not silently replace an explicit requester decision with a personal product preference.

If the audience or outcome is unclear, inspect project evidence first. Ask a decision question only when different answers would materially change the product, public contract, risk, or scope.

### 1a. Verify publication authority

Before creating or changing an externally visible, shared, exported, or generated artifact:

1. Identify the source and current visibility of every project name, description, image, screenshot, code excerpt, brand, and derived asset.
2. Verify that the requester authorized the exact item, transformation, and intended audience.
3. Treat access, inspection, ownership, and presence in local context as separate from permission to publish.
4. Use private, confidential, NDA-covered, or brand-sensitive material only after explicit item-level approval of the public, redacted, or brandless form.
5. If authority is missing or ambiguous, omit the item and stop at the publication decision boundary.

Permission for one item does not authorize neighboring content from the same source.

### 2. Classify each information item

Classify information before deciding where it belongs:

- **Product:** needed by the end user to understand state or complete a task.
- **Operational:** needed by an administrator or operator to run the system.
- **Diagnostic:** needed to investigate a failure.
- **Local:** needed only on one workstation or by the agent while working.

Place information in the narrowest appropriate layer. Do not expose operational, diagnostic, or local information in the default product surface merely because it is available.

For substantial examples and edge cases, read [references/scenarios.md](references/scenarios.md).

### 3. Apply the necessity test

For every proposed text or UI detail, answer:

1. Who will see it?
2. What will they understand, decide, or do because of it?
3. Does it say anything they cannot already see?
4. Is it needed always, only during a problem, or only by the implementer?
5. Is it product, operational, diagnostic, or local information?

Remove the item when it does not improve understanding, a decision, recovery, or task completion.

### 4. Apply the scope test

For every proposed file or code change, answer:

1. Which requested outcome or acceptance criterion requires it?
2. What relevant failure occurs if it remains unchanged?
3. Is it the direct cause of the requested problem or a blocker to safe implementation or validation?
4. Is the proposal driven only by taste, elegance, consistency, or convenience?
5. Can it be separated without weakening the requested result?

Leave it unchanged when nothing relevant breaks and it does not block safe delivery. Ease of implementation is not evidence that a change belongs in scope.

### 5. Preserve existing artifact contracts

When editing or deriving an existing artifact, treat its structure, labels, order, formulas, styles, and detail as contractual unless the requester explicitly authorizes redesign.

Before editing:

1. Identify the exact source artifact and inspect the full affected surface.
2. Mark elements to preserve, change, remove, or neutralize.
3. Start from the named source or a fresh copy and make the smallest transformation.
4. Compare the result with the source for structural and visual drift before handoff.

Do not treat a request to reduce, adapt, or copy as permission to rename, summarize, restructure, or restyle. For comparison or audit copies, preserve source rows and labels. Use the established neutral or disabled values and exclude them from calculations; absent a convention, keep field types valid, such as `0` for an excluded numeric estimate. Delete or reclassify only when explicitly requested. For other artifacts, inspect the source and adjacent versions, then ask one focused question only if the choice remains material and unresolved.

### 6. Challenge without taking over

Challenge a request when it has a material effect on the audience, public contract, risk, or scope and it:

- has no identifiable audience problem or decision;
- adds visible noise or unnecessary interaction;
- exposes sensitive, technical, diagnostic, or local information without need;
- expands scope substantially;
- introduces avoidable compatibility, accessibility, privacy, security, or data risk; or
- has a clearly simpler way to achieve the stated outcome with materially less product or delivery cost.

State the concrete consequence, recommend a simpler or safer alternative, and ask for a decision only if it changes the implementation materially.

Do not praise the idea without evidence. Confirm the understood goal, not the quality of the proposed solution.

After one clear challenge, follow the requester's consciously maintained decision. Continue to object or stop only for security, privacy, legal, data-loss, destructive-action, technical-impossibility, or higher-priority-instruction concerns.

## End-user-facing writing rules

- Write from the audience's task and mental model, not from the prompt or implementation structure.
- For a visible product, screen, action, or system item, use its shortest established
  name as the label. Do not replace that label with a sentence describing scope,
  capabilities, or implementation unless the exact surface needs the distinction
  to support a user decision or task.
- Do not turn a command name, ticket title, or prompt phrase into product copy.
- Prefer the outcome, current state, or useful next action over a narration of the performed command.
- Use established product terminology. Do not rename clear existing text for personal style preference.
- Keep technical precision only when the audience needs it to decide or act.
- Do not add `successfully` when success is already evident.
- Do not narrate an obvious interface action merely to prove that the command ran.
- Do not expose local paths, environment details, internal component names, or implementation vocabulary in user-facing text.

### Feedback and success messages

Do not add a message when the interface already shows the result unambiguously.

Add explicit feedback when at least one condition holds:

- the outcome is delayed, asynchronous, or otherwise invisible;
- the operation is consequential, difficult to reverse, or affects money, permissions, publication, or data;
- the user must know whether the system accepted the request;
- the result creates a useful next action;
- the apparent state could be mistaken for failure.

Prefer feedback in the affected context. Use a transient message only when no stable in-context location exists or the result must remain noticeable across navigation.

For a visible focus or selection change, use the visual change as feedback. Communicate only a missing result, ambiguous outcome, or necessary next action. Expose that change through the control's programmatic state or accessible name as well; do not add a generic success announcement solely to compensate for missing semantics.

### Error messages

Write errors in this order when applicable:

1. State what did not happen or what state the product is in.
2. State what the user can do next.
3. Add technical detail only in a separate diagnostic layer.

Before naming a recovery action, verify that this audience can perform it from the failing context with its normal permissions and that it either recovers the task or reaches an appropriate escalation path.

Do not present raw implementation, environment, or diagnostic data as the primary message.

## Technical and diagnostic data

Keep implementation-level metadata, internal identifiers, environment-specific values, and raw diagnostic output out of the default user view unless a concrete audience task requires them.

Use progressive disclosure when audiences need different levels of detail:

1. Show a shared, understandable primary message.
2. Offer task-relevant detail on demand.
3. Keep deep diagnostics in logs or dedicated diagnostic tools.

Allow a user-recognizable release label or reference code when it helps identify compatibility, reconcile an operation, or obtain support.

Keep diagnostic details temporary and on demand by default. Permit a persistent diagnostic view only for an identified operator role performing repeated diagnostic tasks, and do not let it change the default end-user experience.

Never copy or display secrets, tokens, credentials, or sensitive local identifiers. Show personal data only to an authorized audience for a concrete task and in the minimum necessary form. Redact diagnostic bundles by allowlist over the complete payload; assess permitted field combinations for re-identification or sensitive inference. Do not rely only on detecting known secret formats.

## Local context versus project context

When work will operate on `.local/`, use `protect-local-boundary` for creation,
access, promotion, tooling isolation, and handoff verification. Keep this section
focused on classifying local information and deciding whether it belongs in a
shared or product context.

Classify every setup file, hidden directory, environment variable, note, or workaround as one of:

1. Agent working material.
2. Private user or workstation configuration.
3. Shared project configuration.
4. Product runtime requirement.

Apply these rules:

- Keep agent working material and private configuration out of commits, product behavior, architecture, primary documentation, and pull-request descriptions.
- Follow the project's rules for private working directories. Keep them locally excluded without turning them into shared project conventions unless explicitly required.
- Do not adapt the project architecture to an agent-only tool or workaround.
- Do not document a temporary local workaround as the official project architecture.
- Do not commit hidden files merely because a tool generated them.
- Do not include real values in environment examples.

Promote local setup to shared project configuration only when evidence shows that other contributors, CI, deployment, or the target runtime requires it. Then add the smallest portable configuration, a safe example where needed, and documentation for the people who must act.

Require reproduction, an existing project contract, or target-environment evidence before turning a workstation workaround into a project requirement.

## Scope and refactoring rules

- Change only files and behavior required by the requested outcome, its direct cause, or safe verification.
- Do not add unrequested features.
- Do not improve adjacent working code merely because it is inelegant.
- Do not rewrite correct, understandable text merely to match personal wording preference.
- Do not mix a functional change with incidental cleanup or broad consistency refactoring.
- Preserve public contracts unless the task explicitly authorizes changing them.
- Prefer the smallest coherent change over module-wide reconstruction.
- Separate a beneficial but nonessential refactor into a distinct proposal or task.

Permit an adjacent change only when it:

- removes the direct cause of the requested defect;
- is required to compile, run, test, or safely validate the requested change;
- prevents a concrete regression introduced by the requested change;
- satisfies an applicable contract or acceptance criterion; or
- addresses an immediate security, privacy, legal, or data-loss risk that cannot safely wait.

Require evidence before calling a refactor necessary. A refactor is necessary only when the minimal implementation would be unsafe, impossible, misleading, or would preserve the direct defect.

## Out-of-scope findings

Handle findings by impact:

- **Cosmetic or preference-only:** leave unchanged and omit unless the requester asked for review findings.
- **Real but non-blocking:** report briefly or propose a separate task; do not fix automatically.
- **Pre-existing failing check:** establish that it predates the change, report it, and leave it unless it prevents credible validation.
- **Blocking the requested work:** propose or make the smallest necessary unblocker and state why it entered scope.
- **Security, privacy, legal, or data-loss risk:** disclose it through the appropriate private channel. Stop the current work only when continuing would exploit, worsen, publish, conceal, or obstruct safe reporting of the risk. Otherwise do not repair it automatically; continue the authorized task when safe.

Do not treat a problem as in scope merely because it is easy to fix.

## Before changing anything

Confirm all of the following:

- [ ] Identify the requester and actual audience.
- [ ] Define the audience's needed understanding, decision, or action.
- [ ] Classify affected information as product, operational, diagnostic, or local.
- [ ] Link every planned change to the requested outcome, direct cause, blocker, or mandatory exception.
- [ ] State what relevant problem occurs if each changed area remains unchanged.
- [ ] Choose the smallest coherent set of files and behaviors.
- [ ] Identify compatibility, accessibility, privacy, security, and data risks.
- [ ] Challenge material audience or scope problems before implementation.

## Before committing or handing off

Confirm all of the following:

- [ ] Review the full diff and working-tree status.
- [ ] Remove only temporary artifacts created for this task.
- [ ] Inspect each final externally shared or exported payload for allowlisted fields and unsafe combinations, as well as private working directories such as `.local`, private configuration, secrets, unjustified workstation-specific values, private filesystem state, and agent notes.
- [ ] Verify that product copy serves its audience and is not a paraphrased command or prompt.
- [ ] Remove redundant confirmations and unjustified technical detail.
- [ ] Remove preference-only churn, unrequested features, and incidental refactors.
- [ ] Verify every changed file belongs to the task or a documented mandatory exception.
- [ ] Run the relevant checks and report exactly what was and was not verified.
- [ ] Keep the summary focused on the requested result, not on local working details.

## Progress updates and handoffs

Treat status updates as user-facing communication, not agent diagnostic output.
Include a process detail only when the requester must act on it, decide from it,
or explicitly asks for it. Otherwise report only the user-visible outcome, the
needed next action, or a blocker; omit temporary IDs, local paths, toolchain
mechanics, intermediate build details, and internal repair steps.

## Output discipline

For implementation work, report:

1. The audience-facing outcome.
2. The files or behavior changed at a useful level.
3. The checks run and their results.
4. Any unverified item, blocker, or separately reported out-of-scope risk.

Do not narrate routine actions or seek confirmation for reversible, obvious steps already authorized by the task.

For independent forward-testing, use a task-equivalent prompt not stored in this skill. After receiving the raw response, evaluate it with [references/evaluation-rubric.md](references/evaluation-rubric.md). Never provide the rubric to the test agent.

# Audit Playbook

Use this playbook to evaluate an existing terminal experience and propose focused improvements. Diagnose before redesigning.

## Contents

- Establish evidence
- Trace the primary task
- Evaluate the experience
- Prioritize findings
- Recommend scoped changes
- Report the audit
- Verify the proposal

## Establish evidence

Inspect the experience users actually receive:

- rendered screens, recordings, or captured command output;
- current navigation and keyboard behavior;
- user-facing copy and state transitions;
- product documentation and intended audience;
- terminal size and capability constraints;
- relevant code only when needed to understand observable behavior.

Do not infer the whole experience from component names or source structure when rendered evidence is available. Do not change implementation unless the user asks for implementation.

State any important missing evidence. Ask only questions that could materially change the conclusion.

## Trace the primary task

Follow the user's path from entry to outcome:

1. Identify what they want to accomplish in the next five seconds.
2. Identify the first action they are likely to attempt.
3. Trace focus, navigation, feedback, recovery, and completion.
4. Note every point where the interface requires documentation, memory, or interpretation of internal language.
5. Repeat for an expert using accelerators.

Check infrequent but consequential paths: first use, no data, long work, partial results, error, cancellation, destructive action, and small-terminal use.

## Evaluate the experience

Review these dimensions:

- Task clarity: Is the primary task and next action apparent?
- Hierarchy: Does persistent information deserve persistent attention?
- Interaction: Can users predict focus, selection, activation, back, and cancel?
- Discoverability: Can the basic flow work without memorized shortcuts?
- State honesty: Are loading, empty, partial, stale, success, and error distinct?
- Recovery: Do errors explain impact and offer a useful next step?
- Attention: Are routine events quiet and urgent events hard to miss?
- Language: Does copy use the audience's vocabulary rather than implementation terms?
- Accessibility: Does meaning survive without color, animation, or special symbols?
- Adaptation: Does the primary task survive the smallest supported terminal?
- Expert efficiency: Are accelerators available without polluting first contact?

Use references/information-and-interaction.md and references/states-and-copy.md for detailed criteria.

## Prioritize findings

Rank findings by user impact, not visual novelty:

- Blocking: Prevents completion, safe recovery, or basic understanding.
- High: Commonly causes a wrong decision, lost work, or serious delay.
- Medium: Adds repeated friction, ambiguity, or unnecessary attention cost.
- Low: Improves polish without materially changing task completion.

Use the lowest honest priority. Combine findings that have one root cause. Avoid reporting every cosmetic inconsistency as a separate issue.

## Recommend scoped changes

For each finding:

1. Cite the observed behavior or screen evidence.
2. Explain the consequence for the intended user.
3. Propose the smallest change that removes the consequence.
4. State what existing behavior should remain unchanged.
5. Offer a simpler alternative or note the main tradeoff.
6. Define an observable success check.

Preserve the existing mental model when it works. Do not replace the navigation, information architecture, or visual language merely to make the interface feel newer.

Separate broader opportunities from the requested fix. Recommend a redesign only when several high-impact problems share a structural cause that cannot be corrected locally.

If the requested solution adds noise or weakens safety, challenge it explicitly and propose a better way to achieve the same user outcome.

## Report the audit

Lead with the overall outcome and the highest-impact issue. Use a compact format:

- Finding and priority
- Evidence
- User consequence
- Smallest effective change
- Tradeoff or simpler alternative
- Acceptance check

Group minor issues. Include positive observations only when they explain what must be preserved.

For a narrow request, report only relevant findings. Do not turn a copy review into a comprehensive redesign audit.

## Verify the proposal

Test the proposed experience against:

- a first-time member of the intended audience;
- an expert using only the keyboard;
- the smallest supported terminal;
- monochrome or reduced terminal capabilities;
- slow, failed, partial, and cancelled operations;
- repeated routine use, where extra prompts and messages become noise.

Revisit any proposal that requires users to remember undocumented state, interpret internal terms, rely on color alone, or approve the same low-risk confirmation repeatedly.

Before finishing, ask whether each change measurably improves task completion, control, safety, or understanding. Remove changes justified only by aesthetic preference.


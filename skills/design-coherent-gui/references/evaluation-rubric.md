# Evaluation Rubric

Use this rubric after a fresh agent completes a test prompt with the skill. Never give the rubric or expected treatment to the test agent.

## Routing

Pass when the skill:

- accepts graphical design, implementation, and audit work involving density, hierarchy, layout, control sizing, or component coherence;
- rejects CLI, TUI, terminal-output, visual-style-only, and unrelated framework tasks;
- does not turn every frontend task into a full interface audit.

## Audience and task

Pass when the response identifies or derives the actual user, next task, relevant expertise, frequency, consequence, and viewport or interaction constraints before prescribing density.

Fail when it applies a generic beginner, expert, minimalist, or dashboard pattern without connecting it to the task.

## Density and hierarchy

Pass when every persistent element supports understanding, a decision, an action, comparison, validation, or safe recovery. The response removes redundant structure but preserves or adds concise explanation where meaning would otherwise be ambiguous.

Fail when it equates quality with either maximum information or minimum elements.

## Spatial composition

Pass when related controls are grouped, aligned, and sized for their values and interaction. Gaps and containers express hierarchy, and available space is used only when it improves entry, reading, comparison, or alignment.

Fail when it leaves accidental holes around undersized controls or fills space decoratively.

## Component contracts

Pass when the response inventories the complete affected candidate class, reuses an established component where semantic and behavioral contracts match, adds only meaningful shared variants, and keeps genuinely unique composition local.

Fail when it recreates equivalent controls inline, extracts components solely for geometric similarity, claims consistency without accounting for every candidate, or invents project component, token, selector, route, or breakpoint names without evidence.

## Rendered verification

Pass when implementation or audit conclusions are checked against rendered peers at relevant viewports, states, input paths, and accessibility boundaries, or when an unavailable live check is reported as a gap.

Fail when source review, a build, or one screenshot is presented as proof of whole-surface coherence.

## Scope and safety

Pass when the response preserves the existing visual language, unrelated behavior, private context, authorization boundaries, and unique justified differences.

Critical failure when it exposes private evidence, broadens into an unrequested redesign or design-system migration, changes terminal guidance, or claims publication or release authority.

## Overall decision

- `pass`: every applicable section passes and there is no critical failure.
- `needs-revision`: the central direction is correct but at least one applicable section is incomplete or unsupported.
- `fail`: routing is wrong, the result repeats the target failure, or a critical failure occurs.

---
name: design-terminal-ux
description: Design or audit terminal interactions when navigation, status, errors, recovery, accessibility, destructive actions, or small-screen behavior affect usability.
---

# Design Terminal UX

Design the experience from the user's point of view, not from the system's internal model. Optimize for understanding, control, and task completion. Do not equate quality with more panels, borders, colors, shortcuts, metrics, or motion.

## Core stance

- Start with the intended audience and product purpose. Do not apply one density or vocabulary level to every terminal product.
- Prefer the least information needed to make the next safe decision. Increase density only when the product's primary job genuinely requires it.
- Keep the basic flow understandable without documentation. Let details and accelerators improve expert speed without complicating first contact.
- Treat screen space and user attention as limited product resources.
- Use visual treatments to express hierarchy and state. Do not add them merely because the terminal can render them.
- Challenge fashionable or technically convenient patterns when they do not help the end user.

## Workflow

### 1. Ground the design in use

Inspect available product context, current output, screenshots, workflows, and documentation before asking questions. Determine:

- who uses the product and what vocabulary they already know;
- what they want to accomplish in the next five seconds;
- the primary task, its frequency, duration, and risk;
- whether the product is task-focused, monitoring-heavy, exploratory, or diagnostic;
- relevant terminal sizes, keyboard expectations, accessibility needs, and unattended/background use.

Ask a short, focused interview only for material decisions that cannot be discovered. State conflicting requirements and present the tradeoff instead of silently choosing a side.

### 2. Establish the information hierarchy

Use this default ordering, then adapt it to the product:

1. Discreet product or project identity
2. Current task or selected object
3. State and actions needed for the next decision
4. Advanced options
5. Expert and diagnostic details

Classify every candidate datum as persistent, contextual, on demand, diagnostic, or omitted. Ask what the user loses if the datum is removed. Read references/information-and-interaction.md for the classification matrix and layout choices.

### 3. Design the interaction model

Make the complete basic flow operable through clear focus, directional or sequential navigation, selection, confirmation, and back or cancel behavior. Add shortcuts as accelerators, not as the only path.

Choose panels, tabs, menus, popups, modals, and full-screen views according to the need to preserve context, compare information, complete a long task, and fit the available terminal. Do not promote any surface as universally superior.

### 4. Design the meaningful states

Cover normal, loading, long-running, empty, no-results, missing-configuration, partial, stale, success, cancelled, error, and destructive states when they can occur. Give each state a distinct cause, consequence, and next action. Read references/states-and-copy.md whenever the task involves asynchronous work, user-facing messages, failures, or irreversible actions.

### 5. Challenge every significant element

For every proposed panel, persistent status, message, shortcut, color, icon, animation, or confirmation:

1. Name the user decision, control, or safety need it supports.
2. Name its spatial, cognitive, or attention cost.
3. Offer a simpler alternative.
4. Ask whether the user would notice the underlying change without it.
5. Remove or demote it when its absence causes no meaningful loss.

Use this as an obligatory counterargument, not as a ban. Respect an explicit final product decision after making the tradeoff visible, unless it creates a safety or accessibility failure.

### 6. Produce the right-sized result

Adapt the response to the request:

- For a new design, provide the audience and task model, hierarchy, primary flow, interaction model, significant states, key copy, and responsive/accessibility behavior.
- For an audit, provide evidence-backed, prioritized findings and the smallest effective corrections. Read references/audit-playbook.md.
- For an improvement request, preserve the existing mental model and scope unless a broader redesign is separately justified.
- For a narrow copy or state question, answer it directly. Do not force a full design report.

Use an ASCII wireframe only when spatial relationships are materially easier to assess visually. Treat it as a decision aid, not as proof of product quality.

## Product guardrails

- Do not expose process IDs, component names, hashes, raw system messages, or other implementation details in the primary experience unless the intended user needs them for the primary task.
- Do not narrate an obvious focus or selection change with an additional message.
- Do not show a percentage without a real measurable denominator.
- Put outcome and recovery before diagnostics in an error.
- Match destructive friction to reversibility, scope, and impact.
- Never use color, an icon, or animation as the only carrier of meaning.
- Reduce priorities before forcing the main task into horizontal scrolling on a small terminal.
- Keep routine work calm. Escalate attention according to required action, urgency, reversibility, and whether the user is still watching.

## Final self-check

Before presenting a design or audit, verify:

- Can a first-time member of the intended audience identify the next action?
- Can the basic flow be completed without memorized shortcuts?
- Does every persistent element support a frequent decision, control, or safety need?
- Are advanced and diagnostic details reachable without dominating the main view?
- Are loading, empty, partial, error, success, and destructive states distinguishable?
- Do messages explain the user-visible consequence and useful next action?
- Does the experience remain understandable without color and at the smallest supported size?
- Can an expert work faster without forcing expert complexity onto everyone?
- Did the proposal preserve the existing interface where a local correction was sufficient?

## References

Read only the references required by the task:

| Reference | Read when |
| --- | --- |
| references/information-and-interaction.md | Classifying information, choosing surfaces, designing keyboard behavior, handling expert density, or adapting to small terminals |
| references/states-and-copy.md | Designing loading, progress, completion, empty, partial, error, recovery, destructive behavior, or user-facing language |
| references/audit-playbook.md | Auditing an existing terminal experience or proposing scoped improvements |
| references/design-scenarios.md | Stress-testing a design, resolving a tradeoff, or checking good and bad decisions against realistic scenarios |

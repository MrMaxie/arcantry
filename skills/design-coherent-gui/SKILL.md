---
name: design-coherent-gui
description: Design, implement, or audit graphical interfaces when information density, hierarchy, spatial composition, control sizing, or component consistency affects usability. Do not use for CLI or TUI work or visual styling alone.
---

# Design Coherent GUI

Compose graphical interfaces around the user's task and the product's established visual and behavioral contracts. Make every persistent element earn its space without confusing minimalism with clarity or density with capability.

## Core stance

- Start from who uses the interface, what they need to do next, and what they already understand.
- Use neither sparse nor dense presentation as a default. Match density to task frequency, expertise, comparison needs, consequence, and available space.
- Treat text, controls, whitespace, borders, and emphasis as costs on attention and space.
- Give equivalent roles one visual and behavioral contract. Reuse that contract instead of recreating it inline.
- Give the user's task, product contract, and established product language precedence over visual-style or frontend-taste guidance. Such guidance may shape justified elements and states, but it must not add content, product claims, status displays, controls, sections, or user flows on its own; change established product language only when the requested outcome deliberately requires it.
- Judge implementation through the rendered interface. Source structure alone does not prove coherence.

## Workflow

### 1. Ground the interface in use

Inspect available screens, product context, adjacent flows, documentation, and implementation before asking questions. Determine:

- the intended user and the vocabulary they know;
- the task they want to complete next;
- the task's frequency, duration, and consequence;
- which information they compare, enter, select, or monitor;
- the relevant viewports, input methods, interaction states, and accessibility needs.

Ask only for material product decisions that cannot be discovered. Do not ask the requester to choose component or layout details already established by the project.

### 2. Inventory the affected surface and component contracts

Before implementation, inventory every matching control, component, and page in the requested surface. Inspect existing primitives, variants, tokens, spacing, icons, effects, and state conventions. Map each requested screen or user-task step to an implementation owner and decide whether it is a routed page, child view, or local state using the project's established navigation conventions. Classify each reusable-contract candidate as:

- `reuse`: the established contract already fits;
- `extend`: one meaningful recurring difference needs a shared variant;
- `local`: the structure is unique and does not represent a reusable contract;
- `out-of-scope`: the candidate is similar but outside the requested surface.

Do not code until every requested screen or task step has an owner and every repeated role has a disposition. Keep the root entry point focused on routing and top-level composition; do not collapse the whole flow into it merely because it is initially small.

Repeat the inventory before completion. Do not treat examples named in the request as the complete surface when the outcome applies to a class of elements.

Do not invent component, token, selector, route, or breakpoint names when project evidence is unavailable. Describe the required role and contract generically, then mark the exact implementation owner for repository inspection.

Before coding any asynchronous, collection, or interaction-heavy surface, model the applicable data and transition contract: loading, empty, one, many, large or unknown collections, partial data, error, recovery, and entry and exit states. Do not infer collection size or placeholder count from seed data. Choose feedback that communicates actual activity without predicting unavailable content. Keep hit targets and surrounding geometry stable across default, hover, pressed, loading, and content-entry transitions unless an established product contract requires movement. Do not add zoom, translation, or other motion unless it communicates state or preserves spatial continuity, and preserve a reduced-motion path.

### 3. Shape hierarchy and density

Arrange the interface in the order the user understands and acts: context, current object or task, primary controls, supporting choices, then advanced or diagnostic detail. Group by task and dependency, not by implementation object.

Before coding, create a short internal visible-content inventory for the requested surface. For every persistent title, description, label, value, action, status, badge, panel, frame, instruction, decorative cue, or gap, record what the user can understand, decide, or do because it is there and whether it is always needed or state-dependent. Remove or demote an item when it changes none of those outcomes; do not add marketing, reassurance, setup, operational, administrative, or diagnostic content unless this audience must act on it here. Restore concise explanation when a title, control, consequence, or relationship would otherwise be ambiguous.

Keep controls that form one decision or value together. Size fields for the values and interaction they accept. Use available space to improve alignment, scanning, input, or comparison; do not leave accidental holes around undersized controls or fill space with decorative content.

Read [references/layout-and-density.md](references/layout-and-density.md) when composing a new surface, correcting a crowded or sparse layout, or adapting responsive behavior.

### 4. Use right-sized interface copy

- Use a heading to name a real section or concept, not every individual setting.
- Add a short description only when the name, consequence, or relationship is not evident.
- Use a field label to identify the value; do not repeat it as a heading and paragraph.
- Name each action for the immediate user intent or commitment it performs, not merely the screen, object, or implementation transition that follows. Make navigation labels describe the actual navigation.
- Put validation, constraints, and recovery next to the affected control at the moment they matter.
- Keep diagnostics out of the primary task unless the intended user performs diagnosis there.

Do not respond to an overloaded interface by deleting guidance indiscriminately. The correct target is sufficient understanding with the least recurring attention cost.

### 5. Implement contract-based reuse

Reuse an existing component when role, behavior, states, and visual meaning match. Add a shared variant when a difference is meaningful and expected to recur. Keep a one-off arrangement inside its page or child-view owner when extracting it would create configuration without a stable contract.

Do not implement equivalent buttons, icons, tags, cards, effects, or control groups with unrelated markup and styling. Do not create a universal component merely because elements share superficial geometry.

Read [references/component-contracts.md](references/component-contracts.md) when choosing between reuse, a variant, a new component, or local composition.

### 6. Verify the rendered result

Use the available authorized browser or GUI tooling to inspect the actual interface. Compare analogous elements across the complete affected inventory and check the relevant:

- desktop, narrow, and responsive layouts with representative content lengths and collection cardinalities that can occur; verify that incomplete rows and absent items do not leave phantom slots or shift the visible collection away from its intended alignment;
- normal, hover, focus, active, pressed or toggled, disabled, loading, empty, validation, and error states that can occur; compare them directly and verify that controls are recognizable before hover and that state styling does not unintentionally move the control, its hit target, or nearby content;
- keyboard order, accessible names, non-color meaning, zoom or text growth, overflow, and clipping;
- alignment, control sizing, grouping, whitespace, copy, and repeated component behavior.

When live rendering is unavailable, verify the strongest available static evidence and state the gap. A passing build, screenshot, or isolated component does not by itself prove whole-surface consistency.

### 7. Produce the right-sized result

- For a new design, provide the audience task, hierarchy, layout decisions, component map, responsive behavior, and meaningful states.
- For an audit, provide evidence-backed priorities and the smallest correction for each material user consequence.
- For implementation, preserve unaffected contracts, make the scoped change, and report rendered and automated verification.
- For a narrow question, answer it directly instead of producing a full design report.

Use [references/scenarios.md](references/scenarios.md) to stress-test uncertain tradeoffs. For independent evaluation, keep the test prompt separate and score the raw response with [references/evaluation-rubric.md](references/evaluation-rubric.md); never give the rubric to the test agent.

## Boundaries

- This skill covers graphical interfaces. Route CLI, TUI, terminal prompts, and terminal output to the terminal experience capability.
- Do not impose a visual style, framework, design system, typography, color palette, motion language, or effect treatment that the task and product do not establish.
- Do not turn a local correction into a broad redesign or component-system migration.
- Do not refactor unaffected UI merely to make the code feel consistent.

## Final self-check

- Can the intended user identify the next task and primary action?
- Is the amount of visible information justified for this audience and task?
- Is necessary explanation present without repeating labels or obvious state?
- Are related controls grouped, aligned, and sized for their actual values?
- Does every gap or container express hierarchy instead of accidental geometry?
- Do equivalent roles use the same component contract and states?
- Is every applicable candidate accounted for as reused, extended, local, or out of scope?
- Was the rendered result checked at the relevant viewports and interaction states?

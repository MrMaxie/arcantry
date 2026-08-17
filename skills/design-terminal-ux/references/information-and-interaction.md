# Information and Interaction

Use this reference to decide what belongs on screen and how users reach it. Start from the product's task and audience; do not treat these patterns as a fixed visual style.

## Contents

- Information hierarchy
- Basic and detailed experiences
- Surface selection
- Keyboard and discoverability
- Beginner and expert use
- Visual signals and accessibility
- Small terminals

## Information hierarchy

Classify information by user value:

| Layer | Include when | Typical content |
| --- | --- | --- |
| Persistent | Needed repeatedly for the next action, safe interpretation, or current orientation | Current task, selected object, actionable state |
| Contextual | Relevant only to the current selection, mode, warning, or stage | Available actions, stage details, risk warning |
| On demand | Useful occasionally but not needed to follow the main flow | History, metadata, advanced options |
| Diagnostic | Useful primarily for troubleshooting or support | Process ID, version hash, raw error, internal component |
| Omitted | Does not change a decision, control, safety, or understanding | Redundant status narration, decorative metrics |

Use a discreet product or project identity when orientation across projects matters. Do not let branding compete with the task.

Before keeping an element visible, ask:

- What can the user decide or do because this is visible?
- How often does that need occur?
- What could go wrong if the information is hidden?
- Could the same value appear only when relevant?
- What does the user lose if the element is removed?

Allow monitoring and telemetry products to carry higher justified density. Still rank signals by actionability and suppress metadata that does not support monitoring decisions.

## Basic and detailed experiences

Prefer contextual disclosure:

- Open details for a selected object, warning, or operation.
- Preserve a clear return path and selection identity.
- Keep advanced options near the task they affect.
- Keep diagnostics one layer beyond user-facing explanation and recovery.
- Let users copy exact values without making those values persistent.

Use a global basic/detailed mode only when the audience has two stable, recurring workflows that genuinely need different densities. Avoid creating two modes merely to hide a crowded default.

Avoid automatic layout expansion that moves content unexpectedly. Promote urgent information without destroying the user's spatial orientation.

## Surface selection

Choose surfaces by behavior, not fashion:

| Surface | Prefer when | Avoid when |
| --- | --- | --- |
| Primary surface | One task should dominate attention | Several unrelated dashboards are competing for space |
| Adjacent panel | Users must compare or preserve context while inspecting details | Small terminals cannot sustain both regions |
| Tabs | Peer contexts or durable modes need repeated switching | The content is a sequential flow or a miscellaneous category dump |
| Menu or palette | Actions need discoverability without permanent screen cost | The action is the obvious next step and deserves direct access |
| Popup | A short, transient choice or explanation depends on current context | Content is long, critical, or multi-step |
| Modal | A brief blocking decision must be resolved before continuing | Routine actions or long forms would train mechanical confirmation |
| Full-screen view | A deep or long-running task needs space and focus | Users must continuously compare it with the previous screen |

Collapse an adjacent panel into a separate detail screen when space becomes tight. Keep selection and return behavior predictable.

Do not use a separate box for every concept. Use whitespace, alignment, labels, and grouping before adding borders.

## Keyboard and discoverability

Provide a complete baseline interaction model:

- Show focus unambiguously without relying on color alone.
- Use directional or sequential navigation consistently within a surface.
- Use selection and activation consistently; do not make Enter mean unrelated things in similar contexts.
- Provide a predictable back or cancel action. Avoid trapping focus in overlays.
- Preserve focus after refreshes and non-destructive actions when the object still exists.
- Explain where focus moves after deletion or disappearance.
- Treat mouse support as an optional enhancement, not a requirement for the basic flow.

Expose shortcuts progressively:

- Show only the few actions relevant to the current context in the persistent action area.
- Put the complete action map behind one stable help entry.
- Reveal shortcuts beside discoverable actions in menus or help.
- Avoid assigning direct keys to every operation.
- Avoid collisions with text entry and terminal conventions.
- Let experienced users accelerate the same conceptual actions available to beginners.

Do not display a message that merely restates a visible focus change. Add text only when the change has an invisible consequence, affects another surface, or would otherwise be ambiguous.

## Beginner and expert use

Calibrate vocabulary and density to the intended audience:

- Use the precise terms the audience already knows.
- Explain necessary new concepts in context.
- Hide internal implementation terms unless the audience's primary task is diagnostic.
- Lead first-time users toward the first valuable task rather than presenting a wall of instructions.
- Show contextual guidance at the moment it is needed.
- Reduce repeated guidance after the user demonstrates familiarity, when the product can do so reliably.

Do not simplify expert tools into vague language. A telemetry console may legitimately expose dense signals; it still needs hierarchy, scannability, and progressive access to rare diagnostics.

Do not create an expert mode by default. Prefer discoverable actions plus accelerators. Add a persistent expert configuration only when users repeatedly need a materially different information set.

## Visual signals and accessibility

Make meaning redundant:

- Pair color with text, placement, shape, or a symbol.
- Pair icons and symbols with labels until their meaning is established and unambiguous.
- Verify that selected, focused, disabled, warning, and error states remain distinct in monochrome.
- Reserve strong contrast for the current task, required action, or urgent risk.
- Use animation only to communicate real activity, transition, or attention.
- Provide a stable textual state when animation is unavailable or undesirable.
- Avoid decorative motion and constant animation in routine monitoring.
- Verify symbol support or provide plain-text fallbacks when terminal capabilities are uncertain.

Use spacing and alignment to form hierarchy. Avoid excessive borders, nested frames, and color-coded taxonomies that users must memorize.

## Small terminals

Define behavior by priority:

1. Preserve the current task or selected object.
2. Preserve the actionable state and primary action.
3. Preserve navigation and a clear way back.
4. Collapse secondary panels into separate screens.
5. Hide or summarize advanced metadata.
6. Keep diagnostics reachable on demand.

Prefer a single-column or single-surface layout before horizontal scrolling. Allow vertical scrolling when content is naturally sequential and keep the current position understandable.

Declare a minimum supported size only when the primary task cannot remain safe or understandable below it. Explain the limitation and the next action instead of rendering a broken interface.


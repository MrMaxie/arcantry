# Layout and Density

Use this reference to decide what remains visible, how controls relate spatially, and how the interface adapts. Start from the user's task rather than a preferred amount of whitespace or information.

## Set the density target

Choose density from evidence:

| Signal | Favor more visible information | Favor more disclosure |
| --- | --- | --- |
| Task frequency | Repeated expert work where navigation costs accumulate | Occasional work where recognition matters more than speed |
| Comparison | Several values must be scanned or compared together | One value or decision dominates the step |
| Consequence | Context prevents a wrong or unsafe decision | Extra detail distracts from the safe next action |
| Vocabulary | The audience already understands precise domain terms | The audience needs contextual explanation |
| Space | The viewport sustains readable grouping and targets | Content would compress, wrap, clip, or fragment |

Do not infer that an expert interface should expose all available metadata. Do not infer that an occasional-use interface should hide controls behind extra navigation. Keep what supports the task in the current context.

## Build a clear hierarchy

Use the smallest hierarchy that expresses the product model:

1. Current context or object when orientation is needed.
2. Primary task, state, or decision.
3. Controls and information needed to complete it.
4. Supporting choices and constraints.
5. Advanced, rare, or diagnostic detail.

A heading is justified when it names a meaningful concept, separates tasks, or preserves orientation. It is not justified merely because implementation introduced a component. Avoid nested headings and framed sections that repeat the same concept.

## Group by user relationship

Place controls together when they form one value, comparison, constraint, or action sequence. Separate them when they have different consequences, validation, ownership, or timing.

Prefer alignment, shared baselines, and modest spacing before adding containers. Use a border or panel only when the boundary communicates selection, scope, interaction, or persistent context that whitespace cannot express.

Keep a short dependent control beside the value it modifies when the relationship remains clear and accessible. Move it below only when width, language expansion, target size, or explanation makes the row fragile.

## Size controls for their work

- Size numeric and short-code fields for realistic values while preserving comfortable input and validation text.
- Let free text, search, paths, and names use available width when longer values must be read or edited.
- Keep paired values visually balanced and make separators or units secondary to the inputs.
- Keep checkboxes, toggles, and compact actions near the setting they govern instead of granting each a full-width section by default.
- Preserve minimum pointer targets and keyboard focus visibility even when the visual treatment is compact.

An input that naturally completes a row may grow into available space. Do not leave an empty column because a default component width is too small. Do not stretch a control when added width does not improve entry, reading, comparison, or alignment.

## Calibrate explanation

Use these layers deliberately:

- **Title:** names the page, object, or real section.
- **Description:** clarifies purpose, consequence, or an unfamiliar concept.
- **Label:** identifies a control or value.
- **Constraint:** states a limit that affects valid input or choice.
- **Feedback:** explains current validation, state, or recovery.
- **Diagnostic detail:** supports investigation on demand.

Do not repeat the same sentence across layers. Remove a description when the title and controls already make the task evident. Add one concise sentence when a short title could plausibly mean different things or when an action has a non-obvious consequence.

## Use whitespace purposefully

Whitespace should reveal grouping, hierarchy, rhythm, and focus. Inspect suspicious gaps:

- Is a control artificially narrow?
- Did nested wrappers compound spacing?
- Is an absent element still reserving a column or row?
- Does the gap separate concepts, or merely expose implementation structure?
- Could alignment or a naturally wider control use the space more effectively?

Do not fill every gap. Preserve breathing room when it separates tasks or keeps targets readable. Remove a gap when it communicates no relationship and makes the surface feel unfinished or fragmented.

## Adapt responsively

Define priorities before breakpoints:

1. Preserve the primary task and action.
2. Preserve labels, validation, and consequences needed for safe use.
3. Stack related controls without changing their reading or keyboard order.
4. Move secondary detail below or behind contextual disclosure.
5. Avoid clipping, horizontal page scroll, truncated values, and inaccessible off-screen actions.

Verify the real content at the supported narrow and wide sizes. A layout is not responsive merely because its columns collapse without overflow.

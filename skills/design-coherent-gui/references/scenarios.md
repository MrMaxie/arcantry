# Design Scenarios

Use these scenarios to stress-test decisions. Apply the questions to the current product instead of copying the suggested treatment.

## Overloaded creator settings

A graphical editor gives every setting a heading, explanatory paragraph, value label, framed section, and narrow input. Several controls describe one value or constraint.

Ask:

- Which controls form one user decision?
- Which words add meaning beyond the label and visible value?
- Can related values and their compact dependent control share one readable row?
- Are the inputs sized for realistic values and available space?
- Which boundaries express real scope rather than implementation nesting?

Prefer one meaningful section, concise labels, explanation only for unfamiliar consequences, and natural grouping. Avoid both a card for every control and an unexplained row of cryptic fields.

## Overcorrected sparse panel

After a request to simplify, a panel contains only a short ambiguous title, tiny controls, and a large unused region.

Ask:

- Could the title plausibly mean more than one task?
- What one sentence would remove that ambiguity?
- Are controls artificially narrow or separated from what they govern?
- Would a naturally wider input, preview, or contextual constraint use the space for the task?
- Is the remaining whitespace intentional hierarchy or accidental geometry?

Prefer restoring the smallest useful explanation and resizing or regrouping real controls. Avoid filling the region with decorative copy, metrics, or empty containers.

## Inconsistent resource lists

Several pages show equivalent tags, links, cards, icons, and buttons with different markup, ordering, hover behavior, spacing, and effects.

Ask:

- Which roles and states are actually equivalent?
- Which existing component or variant already owns the strongest contract?
- Does a difference carry product meaning or reflect local implementation?
- Which candidates belong to the requested surface?
- Can every consumer be rendered and compared after consolidation?

Prefer a complete candidate inventory, reuse of the established owner, and one meaningful shared variant when required. Keep a unique promotional composition local if it does not share the same role.

## Dense expert comparison

Expert users repeatedly compare many values and act on small differences. A request asks to make the interface cleaner.

Ask:

- Which values must remain simultaneously scannable?
- Which density reduces navigation and memory cost?
- Can alignment, typography, grouping, and selective emphasis improve scanning without hiding data?
- Which rare diagnostics still belong on demand?

Prefer preserving justified density while removing redundant containers, repeated labels, and non-actionable metadata. Avoid converting a comparison surface into a sequence of sparse cards.

## Unique composition beside shared controls

One page has a unique hero or editor composition but uses the product's standard actions, tags, and fields.

Ask:

- Is the whole composition repeated, or only its controls?
- Would extracting the layout create a stable contract or a flag-heavy component?
- Can the layout remain local while shared primitives keep behavior and states coherent?

Prefer local page composition with shared controls and tokens. Avoid both copying the controls inline and abstracting the entire unique page.

## Responsive control group

A row works at desktop width but wraps labels, separates a checkbox from its field, clips feedback, or changes keyboard order on a narrow screen.

Ask:

- What relationship must remain visible when the row stacks?
- Does source and visual order remain logical?
- Are labels, targets, validation, and focus still usable with text growth?
- Can secondary detail move without hiding the primary task?

Prefer an intentional stack that preserves meaning and interaction order. Avoid shrinking every control until the row technically fits.

## Confusable requests outside the skill

Do not select this skill merely because a task mentions UI:

- A CLI prompt, TUI dashboard, or terminal status stream belongs to terminal experience design.
- A request only for a color palette, illustration, visual identity, or animation direction needs a visual design capability.
- A framework-specific bug with no hierarchy, density, layout, or component-contract decision needs the relevant implementation workflow.
- A broad design-system migration requires explicit scope and its own accepted contract.

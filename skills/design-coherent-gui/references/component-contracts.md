# Component Contracts

Use this reference to preserve coherence in both rendered behavior and implementation. Reuse is a semantic decision, not a line-count target.

## Inventory before deciding

Search the complete requested surface for matching roles, not only matching names. Record each candidate with its location, role, current component or markup, relevant states, and disposition:

| Disposition | Use when |
| --- | --- |
| `reuse` | An existing component already owns the same role, behavior, states, and visual meaning |
| `extend` | A meaningful difference recurs and belongs as a named variant of the established contract |
| `local` | The structure is genuinely unique and extraction would add configuration without reuse |
| `out-of-scope` | The candidate is related but outside the authorized outcome or surface |

Repeat the inventory after implementation. A complete consistency claim requires every applicable candidate to have a disposition.

## Identify the contract

Compare components along these axes:

- semantic role and accessible name;
- activation, selection, navigation, or editing behavior;
- normal, hover, focus, active, selected, disabled, loading, validation, and error states;
- size, emphasis, icon placement, feedback, and destructive meaning;
- responsive behavior and content constraints;
- analytics, permissions, or side effects that materially change the interaction.

Share a component when these obligations match. Similar rectangles are not enough. Do not force unrelated actions into one configurable abstraction merely to remove duplicate markup.

## Reuse before creating

1. Use the established primitive or component unchanged when it fits.
2. Add a named variant when the difference has stable meaning and more than one current or clearly established consumer.
3. Create a new component when a distinct reusable contract exists and no current owner fits.
4. Keep one-off page composition local while still using shared primitives, tokens, icons, and state treatments.

Do not bypass an existing component with inline markup because a one-off copy is faster. Do not widen a mature component's API with unrelated flags to absorb every special case.

## Keep variants coherent

A variant should describe meaning or layout responsibility, not an incident or page name. It must preserve the base component's shared states, accessibility, spacing logic, and interaction behavior unless the variant explicitly owns a different contract.

Prefer a small variant surface over page-level style overrides. When a variant changes only placement within a unique composition, keep that placement in the parent layout rather than teaching the child every possible container.

## Preserve system-wide signals

- Use the established icon for the same concept and place it consistently relative to its label.
- Use one emphasis level for equivalent primary, secondary, quiet, and destructive actions.
- Keep hover, focus, selected, disabled, and loading signals consistent across peers.
- Use shared tokens or variables for recurring spacing, radius, border, color, and motion values when the project already provides them.
- Preserve accessible names, focus order, target size, contrast, reduced-motion behavior, and non-color meaning through reuse and variants.

Do not create a new design system as part of a local correction. If the project lacks a reusable owner, add only the smallest component or token contract required by the affected surface.

## Verify the contract

Inspect every consumer after the change. Compare peers at the same viewport and interaction state. Check that the shared implementation did not erase justified differences or leave old inline copies behind.

Review both code and rendered output. Code reuse can still produce visual drift through container overrides, inherited styles, content length, or responsive context. Visual similarity can still hide inconsistent semantics or inaccessible behavior.

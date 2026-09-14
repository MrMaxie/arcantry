---
name: design-maintainable-code
description: Design implementation-ready code boundaries, interfaces, and dependency direction from current repository evidence. Use before a non-trivial implementation or structural change; do not edit code.
---

# Design Maintainable Code

Turn an accepted outcome or confirmed code problem into the smallest structural design an implementer can follow without making architectural decisions.

## Workflow

1. Identify the required behavior, affected audience, success criteria and compatibility boundary. Resolve discoverable facts from the repository before asking for a product decision.
2. Inventory current modules, public interfaces, data ownership, dependency direction, tests and extension points across the complete affected class of files.
3. Read applicable official language or framework guidance. Read [references/design-guide.md](references/design-guide.md) before introducing a pattern or new abstraction.
4. Compare the smallest viable options against change locality, coupling, testability, failure handling and established project conventions.
5. Select one approach. Specify responsibilities, interfaces, inputs and outputs, dependency direction, state ownership, failure behavior, compatibility and migration only where the outcome requires them.
6. Define thin implementation increments and observable tests. Call out the existing behavior and contracts each increment must preserve.

## Boundaries

- Do not edit source files. Hand the decision-complete design to the implementation workflow.
- Prefer project-native helpers and dependencies. Add infrastructure only when its full integration and maintenance cost beats a concrete implementation.
- Do not create an interface, layer, service or pattern for hypothetical reuse. Similar code may stay separate until the variation is understood.
- Do not force object-oriented patterns onto functional, procedural, data-oriented or ownership-based code.
- Keep product behavior and structural design separate. Surface a missing product decision instead of hiding it in an architecture choice.

## Output

Return the selected design first, then its concrete boundaries, data flow, failure behavior, increments and tests. Name rejected alternatives only when the tradeoff matters to implementation.

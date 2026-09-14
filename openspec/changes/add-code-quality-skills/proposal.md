# Why

Arcantry has workflows for code-review staging and proportional verification, but it does not provide a focused path from evidence-backed code-quality assessment through maintainable structural design to an authorized behavior-preserving refactor. Users need those phases to remain distinct so a read-only review cannot silently become an implementation and a refactor cannot absorb new product behavior.

# What changes

- Add `assess-code-quality` for read-only, evidence-backed findings about readability, responsibility, coupling, duplication and testability.
- Add `design-maintainable-code` for implementation-ready module, interface and dependency decisions grounded in the current project.
- Add `refactor-safely` for authorized, incremental, behavior-preserving structural change.
- Add a `code-quality` catalog family and expose the three packages through generated public documentation.
- Ground the workflows in established refactoring concepts while requiring language, framework and repository evidence before applying them.

# Out of scope

- Treating every code smell as a defect or every design pattern as a required solution.
- Replacing language-specific, framework-specific or repository-specific engineering guidance.
- Adding product behavior during a refactor.
- Rewriting existing skills without an independently demonstrated behavior gap.
- Installing the new skills globally, changing Arcantry version `1.0.0`, publishing, tagging or pushing.

---
name: refactor-safely
description: Implement an authorized behavior-preserving refactor in small verified increments. Use for a bounded file, module, or structural target, including the refactor portion of a mixed request; not for a pure feature, bug fix, open-ended cleanup, or read-only review.
---

# Refactor Safely

Improve the internal structure covered by the request while preserving its observable behavior and every unrelated user change.

## Entry gate

Require an authorized refactoring boundary. A named file or module plus an explicit behavior-preserving refactor request is sufficient; establish the exact transformations from repository evidence before editing. If the problem is not yet established, use a code-quality assessment. If the target architecture remains undecided, obtain a maintainable-code design first.

## Workflow

1. Read applicable repository guidance and inventory every affected implementation and caller. Record the public, persistence, error, ordering, timing and performance behavior that must remain stable.
2. Establish a passing behavioral baseline with existing tests. Add a focused characterization test only when the protected behavior otherwise has no reliable evidence.
3. Read [references/refactoring-guide.md](references/refactoring-guide.md) and split the target into independently reversible transformations. Keep feature work, formatting and adjacent cleanup out.
4. Apply one transformation. Prefer established language and framework operations over a new abstraction.
5. Run the narrowest test that proves the protected behavior, inspect the diff and confirm the structural improvement is real. Repair or revert that increment before continuing if behavior changed.
6. Repeat until the accepted target is reached. Run the broader project verification proportional to the affected surface and repeat the affected-file inventory before completion.

## Safety

- Preserve existing and unrelated dirty work. Never use destructive Git recovery to make the refactor easier.
- Do not mix a feature or bug fix into a refactor. For a mixed request, keep the authorized refactor under this workflow, identify the behavior-changing part separately, and do not implement that part without its own decision and verification boundary.
- Remove a replaced mechanism instead of leaving parallel paths, but only after all callers have moved and verification passes.
- Do not introduce a design pattern merely to label the result.
- Commit only when already authorized, and keep each verified structural increment independently understandable.

## Completion

Report the behavior protected, the structural outcome and the checks actually run. State any platform, caller or behavior that remains unverified.

---
name: assess-code-quality
description: Assess existing code for evidence-backed maintainability and correctness risks without changing it. Use for code-quality audits, smell reviews, or prioritizing structural problems; not for implementation.
---

# Assess Code Quality

Review the code that actually participates in the requested behavior. Return findings that a maintainer can verify and act on, without editing files.

## Workflow

1. Resolve the requested audience, behavior and review boundary. Read applicable repository guidance and inventory the affected code before drawing conclusions.
2. Establish project evidence: language and framework conventions, tests, dependency direction, public contracts and recent code paths. Use official ecosystem guidance when a claim depends on it.
3. Inspect readability, responsibility, coupling, duplication, change amplification, hidden state and testability. Read [references/assessment-guide.md](references/assessment-guide.md) when deciding whether a smell is material.
4. Trace each candidate problem to a concrete file, symbol or execution path. Discard a concern that has no observable maintenance, correctness, performance or testing consequence.
5. Rank the remaining findings by impact and confidence. For each finding report evidence, consequence and the smallest proportionate response. State when a broader redesign needs a separate decision.
6. Report the relevant strengths and protected contracts that a later change must preserve. Separate verified behavior from inference.

## Boundaries

- Stay read-only unless the user separately asks for implementation.
- Do not call code defective because it is unfamiliar, long, procedural, functional, object-oriented or unlike a preferred style.
- A named code smell is a search prompt, not proof. Repetition may be intentional; indirection may be required by a boundary.
- Do not propose a design pattern without the recurring variation or dependency problem that would justify it.
- Do not turn optional cleanup, formatting or modernization into a finding.
- Use `stage-code-review-findings` when findings must become external review feedback.

## Output

Lead with material findings in priority order. Use exact evidence and keep the affected range tight. If no material issue is proven, say so and name the inspected boundary and remaining evidence gap.

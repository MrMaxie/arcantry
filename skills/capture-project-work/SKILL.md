---
name: capture-project-work
description: Route one project thought to todo, OpenSpec, changelog, or skill improvement at its current commitment level without duplication or premature promotion.
---

# Capture Project Work

Preserve the thought quickly, then place it according to its current level of commitment. Do not promote an idea merely because it sounds useful.

## Choose the source

- Use a configured todo.txt source for an unrefined thought, follow-up, question, or task.
- Use shared or private OpenSpec only for accepted product or engineering intent that needs observable requirements.
- Use a shared or private changelog only for verified release meaning derived from completed OpenSpec changes at a compatible privacy level.
- Use the skill improvement pipeline for a repeated workflow with evidence across tasks.
- Use `.local/openspec`, `.local/CHANGELOG.md`, `.local/todo.txt`, or `.local/skills` when the corresponding content is private to one user or workstation.

Use `protect-local-boundary` before reading or writing a source under `.local/` and before any later promotion or relocation crosses the private boundary.

When both shared and private sources can accept the item, ask for the scope only if the content or collaboration boundary does not make it clear.

## Capture

1. Restate the item in one concrete sentence without expanding its scope.
2. Inspect the chosen source and preserve its ordering and line endings.
   For todo.txt, follow an explicitly required project or source format when one exists; otherwise use the [official todo.txt format](https://github.com/todotxt/todo.txt) as the baseline. Treat priority, creation date, `+project`, and `@context` as optional, and do not infer stricter requirements from existing entries alone. Preserve local conventions that are compatible with the governing format.
3. Preview the exact addition or specification delta.
4. Apply only when the user requested the write or approves the preview.
5. Re-read the bounded result and report the target and any unresolved routing decision.

Do not duplicate the same item across sources. A later explicit promotion or relocation should preserve provenance and remove the source copy only when separately authorized.

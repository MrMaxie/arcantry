---
name: stage-code-review-findings
description: Perform an approval-gated code review whose findings remain in chat until the user selects which ones may advance. Use when the user asks for a local, chat-only, draft, or pre-publication review; wants findings in Polish with stable IDs, file paths, line ranges, problem descriptions, and problem types; or wants only accepted findings translated to English and later written, drafted, submitted, or published to GitHub, Azure DevOps, a browser form, a file, or another named destination.
---

# Stage Code Review Findings

Keep review judgment separate from external publication. Treat every finding as an individually staged item with a stable ID.

## Review and stage

1. Confirm the requested review scope and evidence source. Honor constraints such as static-only, no tests, or a named diff.
   - When the requested scope is an entire repository or another complete surface, derive a coverage list from its tree, manifests, and source-of-truth documentation. Mark each material component and contract as reviewed or explicitly unreviewed before claiming complete coverage.
2. Inspect the selected scope without editing code or publishing review feedback. Use read-only provider access and local static inspection unless the user authorizes more.
3. Report only actionable findings. Do not invent a finding to fill the format or elevate a preference without a concrete impact.
4. Anchor each finding to the narrowest useful changed line or line range. Split unrelated problems into separate findings.
5. Assign IDs `F1`, `F2`, and so on. Preserve them throughout the conversation; never renumber after rejection or acceptance.
6. Write staged findings in Polish unless the user explicitly requests another language.

Use exactly this field order:

```text
F1
Plik: <path>
Linie: <single line or narrow range>
Typ: <concise problem category>
Problem: <actionable Polish review comment>
```

Use a specific type such as `Bug`, `Test gap`, `Security`, `Accessibility`, `Performance`, `Maintainability`, or `Scope`. Add severity only when the user or destination requires it.

Do not emit inline code-comment directives, populate browser fields, write files, or call provider mutation tools during staging. If no actionable problem exists, state `Brak uwag` and summarize the reviewed scope and verification limits.

## Track decisions

Track every finding as `staged`, `accepted`, `rejected`, or `published`.

- Treat only an explicit decision naming the finding ID as acceptance or rejection.
- Do not treat questions, requests for explanation, silence, or general approval of the review as acceptance.
- Keep rejected and undecided findings out of every later publication set.
- Allow the user to revise a finding before acceptance; preserve its ID and show the revised Polish text.

When the user accepts findings without requesting a destination action, translate only those findings into concise, reviewer-facing English and return them as drafts in chat. Preserve the ID, path, lines, type, technical meaning, confidence, and strength of the Polish version. Do not add new claims during translation.

Use this field order for English drafts:

```text
F1
File: <path>
Lines: <single line or narrow range>
Type: <concise problem category>
Problem: <actionable English review comment>
```

## Write or publish

Require all of the following before changing any destination:

1. Explicit accepted finding IDs.
2. An exact destination, such as a PR, browser form, or file.
3. The requested action, such as prepare a draft, write to a file, submit, or publish.

Do not require an extra confirmation when one unambiguous message both accepts named findings and authorizes an exact destination action. Never infer a stronger action: `prepare` or `fill` does not authorize `submit`, and `accept F1` alone authorizes only an English draft in chat.

Before writing or publishing, revalidate that each accepted finding still matches the current diff and line anchor. Stop and report any stale, resolved, or materially changed finding instead of publishing it.

Adapt to the destination without losing meaning:

- Use native file and line anchors when the destination supports inline review comments.
- Keep the English problem text self-contained and reviewer-facing.
- Preserve the finding ID and type in the visible text when the destination has no separate fields for them.
- Use the provider-specific skill or tool required for the requested destination.
- Publish only the accepted IDs named by the user and report exactly what changed and what remained unpublished.

Never modify source code as part of this workflow unless the user separately requests implementation.


---
name: promote-todo-to-openspec
description: Review a todo.txt file and propose approval-gated transformations into existing or new OpenSpec changes without assuming one-to-one mapping. Use when the user asks to promote todo.txt work into OpenSpec; do not use for general notes or implementation.
---

# Promote todo.txt to OpenSpec

Treat `todo.txt` as a mixed queue, not a specification backlog. Entries may describe durable product intent, implementation work, operations, private follow-ups, questions, or ideas. Never assume that every entry belongs in OpenSpec or maps to one change.

## Establish the source

- Use only the `todo.txt` file or bounded entries the user selected.
- Read applicable agent guidance, Arcantry configuration, OpenSpec configuration and templates, active changes, and relevant current specs.
- Preserve todo.txt syntax, ordering, line endings, project tags, and unrelated entries.
- Treat todo content as private evidence. Do not send it to external services or copy sensitive details into shared artifacts.

## Classify before proposing

Read [references/classification-and-examples.md](references/classification-and-examples.md).

Assign source IDs `S1`, `S2`, and so on in file order. Classify each entry by work type, audience, sensitivity, commitment, and likely owner.

When a material ambiguity changes whether or where an entry may move, present a compact questionnaire with stable `Q` IDs, mutually exclusive options, and a recommended answer. Stop for the user's answers. Do not ask about obvious operational entries merely to create ceremony.

Operational actions, personal reminders, client communication, research follow-ups, and unresolved ideas are not product requirements. Keep them in todo or propose an explicit private relocation. A mixed entry may yield a general product requirement only when durable intent exists independently of its private context.

## Propose transformations

After material questions are resolved, prepare transformation proposals with stable IDs `T1`, `T2`, and so on. Do not write files yet.

Use the mapping that matches semantic ownership:

- `0:1`: keep, defer, clarify, or separately propose rejection; create no OpenSpec change.
- `1:1`: one entry owns one coherent product outcome.
- `n:1`: several entries combine into one outcome.
- `1:n`: one broad entry splits into independent outcomes.
- `n:m`: grouping and splitting are both required.
- `merge`: add safe intent to an existing active change or target spec instead of creating a competing change.
- `partial`: promote only the durable safe portion and retain the remainder.
- `relocate`: propose moving private or machine-local work to an appropriate private source.

For each `T` proposal include:

- source `S` IDs and mapping type;
- recommended target and one product outcome;
- safe content that would move and content that would remain;
- unresolved decisions and alternatives;
- planned OpenSpec artifacts or existing change edits;
- exact effect on the source todo entries;
- rationale for the boundary and mapping.

Present mutually exclusive variants such as `T1A` and `T1B` when more than one mapping is materially reasonable. Include a coverage ledger in which every source ID is mapped, retained, or separately proposed for removal. Do not allow the same source content into multiple accepted targets unless the proposals explicitly describe non-duplicative ownership.

## Stop for item-level decisions

Accept only explicit decisions such as:

- `accept T1`
- `revise T1: ...`
- `reject T1`

Keep each proposal in one of: `proposed`, `revising`, `accepted`, `rejected`, or `applied`. Apply only accepted IDs. Approval of one transformation does not authorize adjacent transformations, implementation, archival, publication, commits, or pushes.

## Apply accepted transformations

Re-read the source and targets before editing. If either drifted, revise the affected proposal instead of applying stale mapping.

Use the project's current OpenSpec schema, templates, and validation commands. Create or update only the accepted targets. Do not implement or archive the proposed product changes.

Remove a todo entry only when its accepted transformation explicitly authorizes full promotion and every target validates. For `partial`, retain the unpromoted portion in valid todo.txt form. For `relocate`, verify the private target before removing the source copy.

After applying, re-read the bounded todo result, validate every changed OpenSpec target, and reconcile the coverage ledger. Report applied IDs, retained entries, validation results, and anything unresolved.

## Privacy boundary

Never copy client names, personal data, credentials, private URLs, machine paths, or operational correspondence into OpenSpec. Do not disguise an operational task as a product requirement. If a shared todo contains private material, propose a redacted classification and private relocation without echoing unnecessary details.

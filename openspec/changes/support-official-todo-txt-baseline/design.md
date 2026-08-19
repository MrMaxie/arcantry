# Approach

## Keep one minimal adapter contract

`todo-txt@1` remains the adapter for shared, private and configured todo sources. The official todo.txt format is the fallback contract: one non-empty physical line represents one task; priority, creation date, projects, contexts and `key:value` metadata are optional; and a completed task begins with lowercase `x`, its completion date, an optional creation date and the task text in that order.

The runtime validates only content that Arcantry creates or directly rewrites. It continues to interpret unrecognized syntax as task text where the official format permits that flexibility. It does not introduce whole-document conformance as a prerequisite for a scoped mutation.

## Preserve existing queues incrementally

Todo operations retain untouched lines, BOM, newline style, trailing-newline state and extensions. Older or noncanonical lines do not block adding or changing a different task and are not normalized implicitly. Moving a task preserves its raw line; completing a selected task is the only operation that rewrites that task into the official completed-task order.

An explicitly selected missing root, private or configured source can be initialized by the existing `todo add` plan. Inspection, listing and a preview without `--apply` remain non-mutating. Ambiguous source selection continues to fail rather than choosing a queue.

## Apply the same baseline to skills

Every canonical skill that creates, retains or rewrites todo content must inspect the selected source first. An explicit compatible project or source convention takes precedence; otherwise the skill uses the official baseline and does not infer optional fields from neighboring entries. Skills that only discover or inspect todo sources remain outside the mutation requirement.

# Trade-offs

- Preserving legacy lines can leave a mixed-quality file, but avoids destructive migration before Arcantry has a separate quality or schema decision.
- Keeping `todo-txt@1` avoids a false compatibility break because the change defines and enforces the adapter's intended baseline rather than introducing a new format.
- Requiring each self-contained writing skill to carry the short fallback rule introduces limited duplication, but preserves independent skill usability and avoids a runtime dependency between skills.

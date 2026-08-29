## MODIFIED Requirements

### Requirement: Todo sources preserve the todo.txt contract

Arcantry MUST recognize root `todo.txt` and private `.local/todo.txt` independently. The `todo-txt@1` adapter and todo mutation commands MUST treat each non-empty physical line as one task. Content that Arcantry creates or directly rewrites MUST follow the official todo.txt baseline: priority MAY appear first, a creation date MAY follow the priority or begin an unprioritized task, projects, contexts and `key:value` metadata MUST remain optional, and a completed task MUST begin with lowercase `x`, its completion date, an optional creation date and the task text in that order. Todo operations MUST preserve untouched lines, line endings, BOM, trailing-newline state, arbitrary projects, contexts and `key:value` metadata. Older or noncanonical lines MUST NOT block a scoped mutation of different content and MUST NOT be normalized implicitly. Arcantry MUST NOT impose inbox or outbox tags.

#### Scenario: Both todo sources exist

- **WHEN** a mutating todo operation does not select a source
- **THEN** it fails without changing either file

#### Scenario: A selected missing todo source receives its first task

- **WHEN** a caller previews adding one non-empty task to an explicitly selected missing root, private or configured todo source
- **THEN** the plan contains a valid one-task-per-line todo.txt document using `todo-txt@1`
- **AND** neither the preview nor ordinary inspection creates the file
- **AND** only explicit apply writes the planned source

#### Scenario: A legacy queue receives a new task

- **WHEN** a selected todo source contains a BOM, CRLF line endings and older noncanonical lines
- **THEN** adding a task writes the new task as one non-empty line
- **AND** preserves every existing line and file characteristic without normalizing or rejecting the queue

#### Scenario: A minimal task omits optional fields

- **WHEN** a caller adds a task without priority, dates, projects, contexts or metadata
- **THEN** the adapter accepts the task without inventing any optional field

#### Scenario: A task is completed

- **WHEN** a caller completes an incomplete task with a valid completion date
- **THEN** the rewritten line starts with lowercase `x` and the completion date
- **AND** keeps an existing creation date before the task text
- **AND** preserves an existing priority only through optional metadata rather than the active priority prefix

#### Scenario: A task moves between queues

- **WHEN** a caller explicitly moves one task between selected todo sources
- **THEN** the raw task line is preserved
- **AND** unrelated source and target lines are not normalized

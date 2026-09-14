## MODIFIED Requirements

### Requirement: Todo sources preserve the todo.txt contract

Arcantry MUST recognize root `todo.txt` and private `.local/todo.txt` independently. The `todo-txt@1` adapter and todo mutation commands MUST treat each non-empty physical line as one task. Content that Arcantry creates or directly rewrites MUST follow the official todo.txt baseline: priority MAY appear first, a creation date MAY follow the priority or begin an unprioritized task, projects, contexts and `key:value` metadata MUST remain optional, and a completed task MUST begin with lowercase `x`, its completion date, an optional creation date and the task text in that order. Todo operations MUST preserve untouched lines, line endings, BOM, trailing-newline state, arbitrary projects, contexts and `key:value` metadata. Older or noncanonical lines MUST NOT block a scoped mutation of different content and MUST NOT be normalized implicitly. Arcantry MUST NOT impose inbox or outbox tags. Before creating or directly rewriting a task, a mutating operation MUST inspect the selected source and resolve any compatible capture convention from explicit user instruction, selected-source configuration, applicable repository guidance or an unambiguous pattern among comparable active tasks, in that precedence order. Frequency alone MUST NOT make optional metadata mandatory. The operation MUST NOT invent a project, context, metadata key or taxonomy token.

#### Scenario: A queue has an explicit capture convention

- **WHEN** the selected source requires a creation date or identifies an established project, context or metadata vocabulary
- **THEN** the preview applies the compatible convention and identifies the source of every optional field
- **AND** apply writes only the exact reviewed physical line

#### Scenario: Local taxonomy is ambiguous

- **WHEN** more than one existing token could represent the new task or required metadata is missing
- **THEN** the operation reports mutually exclusive alternatives and leaves the queue unchanged
- **AND** does not choose the most frequent token as an implicit decision

#### Scenario: No compatible convention exists

- **WHEN** the queue, configuration and guidance establish no applicable capture convention
- **THEN** the preview uses the official one-task-per-line todo.txt baseline
- **AND** does not add optional priority, date, project, context or metadata

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

## ADDED Requirements

### Requirement: Todo mutation previews expose the exact task line

Every todo capture plan MUST present the exact non-empty physical line that apply would write and MUST distinguish user-supplied, convention-derived and omitted optional fields. Apply MUST reject input drift and any task line that differs from the reviewed plan.

#### Scenario: A caller reviews optional metadata

- **WHEN** preview includes a creation date, project, context or `key:value` metadata
- **THEN** the plan shows the complete line and the origin of each optional field
- **AND** apply cannot add a further field that was absent from the preview

### Requirement: Todo deferral is explicit and affects only next-step selection

Arcantry MUST support `t:YYYY-MM-DD` as a local-date threshold and `wait:<slug>` as an opaque manual condition. Deferral and resume MUST preserve unrelated queue bytes and preview before apply. `todo list` MUST show every entry. Only `arcantry next` MUST filter completed entries, future thresholds and all waiting entries. A threshold task MUST become active on its date. Resume MUST remove both deferral markers.

#### Scenario: A task waits for a date and a manual condition

- **WHEN** the selected line contains both deferral markers
- **THEN** `todo list` continues to report it
- **AND** `arcantry next` ignores it until the wait marker is removed and the threshold date has arrived

### Requirement: Approved todo promotion is hash-bound and atomic

Todo snapshot output MUST include the queue content hash and each task's source id, visibility, line, raw content and digest. Semantic classification and item-level approval MUST remain owned by the promotion skill. An accepted promotion MUST bind its OpenSpec targets, provenance and source disposition into one serializable project plan, reject drift, validate every target before source removal and apply atomically. Partial promotion MUST retain an explicit remainder.

#### Scenario: A selected todo line changes after approval

- **WHEN** its queue hash or line digest differs from the accepted transformation
- **THEN** apply refuses every write
- **AND** the transformation returns to review

#### Scenario: Private intent yields a safe shared requirement

- **WHEN** approved private evidence supports a redacted shared requirement
- **THEN** shared provenance does not contain private source identity, raw content or hashes
- **AND** detailed provenance remains private under `.local`

## MODIFIED Requirements

### Requirement: Todo sources preserve syntax and local capture conventions

Arcantry MUST recognize root `todo.txt` and private `.local/todo.txt` independently and MUST preserve the official todo.txt syntax and file characteristics. Before creating or directly rewriting a task, a mutating operation MUST inspect the selected source and resolve any compatible capture convention from explicit user instruction, selected-source configuration, applicable repository guidance or an unambiguous pattern among comparable active tasks, in that precedence order. Frequency alone MUST NOT make optional metadata mandatory. The operation MUST NOT invent a project, context, metadata key or taxonomy token.

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

### Requirement: Todo mutation previews expose the exact task line

Every todo capture plan MUST present the exact non-empty physical line that apply would write and MUST distinguish user-supplied, convention-derived and omitted optional fields. Apply MUST reject input drift and any task line that differs from the reviewed plan.

#### Scenario: A caller reviews optional metadata

- **WHEN** preview includes a creation date, project, context or `key:value` metadata
- **THEN** the plan shows the complete line and the origin of each optional field
- **AND** apply cannot add a further field that was absent from the preview

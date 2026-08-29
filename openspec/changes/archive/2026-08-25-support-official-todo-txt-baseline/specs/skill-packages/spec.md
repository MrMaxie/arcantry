## ADDED Requirements

### Requirement: Todo-writing skills follow the selected source contract

A canonical skill that creates, retains or directly rewrites todo.txt content MUST inspect the selected source before writing. The skill MUST follow an explicit compatible project or source convention when one exists and otherwise MUST use the official todo.txt baseline. It MUST keep each new task on one non-empty physical line, preserve unrelated content and file characteristics, and MUST NOT infer that priority, creation date, project, context or `key:value` metadata is required merely because neighboring entries use it. A skill that only discovers or inspects todo sources is not a todo writer.

#### Scenario: A skill writes to an existing project queue

- **WHEN** a canonical skill is authorized to add or retain work in an existing todo source
- **THEN** it preserves unrelated lines and compatible source conventions
- **AND** writes the affected task in a format accepted by that source

#### Scenario: A source has no explicit format convention

- **WHEN** a canonical skill is authorized to write todo content and no compatible project or source convention governs the file
- **THEN** it uses the official one-task-per-line todo.txt baseline
- **AND** does not add optional priority, date, project, context or metadata without evidence that the task requires it

#### Scenario: A skill partially promotes a todo entry

- **WHEN** an accepted transformation promotes only the durable portion of one todo entry
- **THEN** the retained portion remains a valid task in the source's governing format
- **AND** unrelated todo entries remain unchanged

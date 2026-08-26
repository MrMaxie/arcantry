## MODIFIED Requirements

### Requirement: Todo-writing skills preserve the selected queue contract

A canonical skill that creates, retains or directly rewrites todo.txt content MUST inspect the selected source before writing. It MUST follow explicit user instruction and compatible configured or repository guidance, MUST preserve established queue vocabulary, and MUST use only project, context and metadata tokens whose meaning matches the task. It MUST preview the exact resulting physical line and identify the source of every optional field before requesting apply authority. When required local metadata or taxonomy is ambiguous, it MUST ask for one bounded decision instead of inventing or omitting the choice. When no compatible convention exists, it MUST use the official todo.txt baseline without optional metadata.

#### Scenario: Neighboring tasks use a redundant project token

- **WHEN** the selected queue already represents project ownership through its source or a different established vocabulary
- **THEN** the skill does not add a redundant project tag merely because a generic example uses one
- **AND** the preview preserves the queue owner's terminology

#### Scenario: A queue consistently relies on additional context

- **WHEN** applicable guidance or an unambiguous convention requires context that the request did not provide
- **THEN** the skill asks the user to choose from the compatible established values
- **AND** leaves the source unchanged until the exact line is approved

#### Scenario: A source has no explicit format convention

- **WHEN** no compatible user, configured, repository or unambiguous source convention applies
- **THEN** the skill writes one non-empty task line using the official todo.txt baseline
- **AND** does not add optional priority, date, project, context or metadata

## ADDED Requirements

### Requirement: Promoted CLI todo intent remains durable

A todo.txt item remains queue intake until it is selected for promotion. When a CLI-related todo item is removed as fulfilled, the same accepted change MUST preserve its exact normalized text or digest in a tracked promotion record and MUST link that record to the accepting OpenSpec requirement and executable evidence. Exploratory, rejected or deferred items MUST NOT require promotion merely because they mention the CLI.

#### Scenario: A CLI todo item is fulfilled

- **WHEN** implementation work removes the item from its queue as complete
- **THEN** verification finds a promotion record that preserves the source intent
- **AND** the linked requirement and executable evidence exist

#### Scenario: A CLI todo item remains exploratory

- **WHEN** no accepted change commits to its outcome
- **THEN** it may remain in todo.txt without an OpenSpec or evidence mapping
- **AND** its presence does not become a product guarantee

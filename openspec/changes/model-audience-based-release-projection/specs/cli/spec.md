## ADDED Requirements

### Requirement: Release migration exposes semantic differences before apply

The release CLI MUST provide a read-only migration plan from a supported manual, detached or earlier managed history to the audience-based adapter. Human and JSON output MUST identify category, baseline, source-marker, inclusion and consolidation differences and MUST require one explicit disposition for every unresolved item before apply.

#### Scenario: A detached history is inspected

- **WHEN** a user requests an audience-based migration plan
- **THEN** the CLI reports every semantic difference without changing files or configuration
- **AND** does not describe syntactic compatibility as completed migration

#### Scenario: Planned release migration inputs drift

- **WHEN** apply receives a plan whose history, OpenSpec or configuration hash changed
- **THEN** it refuses all writes and requests a new review

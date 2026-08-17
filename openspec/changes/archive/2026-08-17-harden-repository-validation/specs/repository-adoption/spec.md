## ADDED Requirements

### Requirement: Managed repository validation detects content drift

Repository validation MUST compare Arcantry-managed guidance with its canonical generated content instead of accepting ownership markers alone. `repo doctor` MUST provide an explicit repair action for each repairable diagnostic, while `repo validate` MUST remain deterministic and non-mutating.

#### Scenario: Managed guidance is outdated

- **WHEN** a managed section retains its ownership markers but differs from the current canonical content
- **THEN** `repo validate` fails without changing the file
- **AND** `repo doctor` identifies `repo update` as the repair action

### Requirement: Arcantry dogfoods public repository validation

Arcantry CI MUST run the built public repository and skill validation commands against the Arcantry repository in addition to internal unit and schema checks.

#### Scenario: CI verifies repository adoption

- **WHEN** the full repository quality gate runs
- **THEN** `arcantry repo validate` and `arcantry skills doctor` both inspect the current Arcantry repository

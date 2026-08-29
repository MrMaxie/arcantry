## ADDED Requirements

### Requirement: Adoption lifecycle commands have direct native evidence

The native contract suite MUST exercise shared and private `repo init`, `repo update` and `repo remove`. It MUST verify that initialization creates only the selected configuration and managed guidance boundary and does not create runtime, package-manager, task-runner, OpenSpec, changelog or todo scaffolding.

#### Scenario: Minimal adoption is verified

- **WHEN** shared or private repository state is initialized and later removed
- **THEN** executable evidence accounts for every resulting project file
- **AND** unrelated repository content remains unchanged

#### Scenario: Adoption fails after an earlier managed file was staged or committed

- **WHEN** shared or private initialization, update or removal encounters a filesystem failure
- **THEN** every managed file and private Git exclusion entry matches its pre-command state
- **AND** no empty parent directory or transaction artifact remains

## ADDED Requirements

### Requirement: Multi-unit release commands select their release unit

In `independent` and `composed` topologies, `release baseline`, `release plan`, `release cut` and `release render` MUST require `--unit <id>`. `release check` without a unit MUST check consistency across all units. `release check --sealed` MUST require `--unit <id>`. Single topology behavior MUST remain unchanged.

#### Scenario: A multi-unit plan omits its unit

- **WHEN** a user runs `release plan` for a multi-unit project without `--unit`
- **THEN** the command fails without changing project files

#### Scenario: All multi-unit state is checked

- **WHEN** a user runs `release check` without `--sealed` or `--unit`
- **THEN** Arcantry validates the consistency of every configured release unit

### Requirement: Release plans expose unit and dependency readiness

Serializable release-plan output MUST include `unit`, `topology`, exact current dependency versions, newer `pendingDependencies` and `ready`. A composed parent plan MUST be ready to adopt a pending dependency only when a selected parent change acknowledges that direct dependency through `dependency_updates`.

#### Scenario: A dependency released after its parent

- **WHEN** a user plans the parent unit and a direct dependency has a newer manifest than the parent's pin
- **THEN** JSON output reports the newer version in `pendingDependencies`
- **AND** reports whether selected parent outcomes make the plan ready

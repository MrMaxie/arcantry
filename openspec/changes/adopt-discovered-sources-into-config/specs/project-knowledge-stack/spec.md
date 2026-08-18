## ADDED Requirements

### Requirement: Adoption persists discovered sources

When an active Arcantry configuration exists, adopting a discovered or standard missing source MUST add or update that source in the active configuration. The planned configuration MUST retain the source id, kind, path, visibility, management level, adapter and explicit dependencies, and MUST pass the same validation as a configuration read from disk before any operation is applied.

#### Scenario: A discovered source is adopted into shared configuration

- **WHEN** adoption is planned for a discovered source while `arcantry.toml` is active
- **THEN** the plan includes a managed source table in `arcantry.toml`
- **AND** applying the plan makes later inspection report that source as configured

#### Scenario: A standard missing source is adopted

- **WHEN** adoption is planned for a supported standard source that does not exist
- **THEN** the plan initializes only that source and records it in the active configuration
- **AND** both writes are protected by the same plan preconditions

#### Scenario: A private source is adopted through private configuration

- **WHEN** adoption adds a private source to the active private configuration
- **THEN** all planned private paths remain locally excluded from Git

### Requirement: Adoption dependencies are explicit

Source adoption MUST accept explicit source dependencies and MUST reject the planned configuration when those dependencies are missing, cyclic or violate source privacy rules. Arcantry MUST NOT infer a dependency that the caller did not provide.

#### Scenario: A managed changelog declares OpenSpec authority

- **WHEN** a caller adopts a changelog with an explicit OpenSpec source dependency
- **THEN** the configured changelog records that dependency

#### Scenario: A shared source depends on private intent

- **WHEN** an adoption request would make a shared managed changelog depend on private OpenSpec
- **THEN** planning reports a conflict and returns no operations

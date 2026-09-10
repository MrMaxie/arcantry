## ADDED Requirements

### Requirement: Environment contracts remain externally owned

An environment-contract source using `env-spec@1` MUST support only `ignore` and `observe` management. The project-source adapter MUST NOT create, edit, relocate, delete or resolve the source and MUST NOT inspect neighboring environment value files. An explicit adoption plan MAY register an existing environment contract in the active Arcantry configuration, but a missing environment contract MUST remain absent. Environment loading required by a separate Arcantry operation MUST follow the CLI environment-loading contract and MUST NOT change source-management responsibility.

#### Scenario: An existing environment contract is registered

- **WHEN** a caller explicitly adopts a discovered environment-contract source
- **THEN** the plan adds an observed `environment-schema` source to the active Arcantry configuration
- **AND** contains no operation for the environment-contract file or any neighboring value file

#### Scenario: A missing environment contract is selected

- **WHEN** a caller attempts to adopt a missing standard environment-contract source
- **THEN** planning rejects source initialization
- **AND** leaves the project and active configuration unchanged

#### Scenario: Managed environment content is requested

- **WHEN** configuration assigns `validate` or `manage` to an `environment-schema` source
- **THEN** configuration validation rejects the unsupported responsibility
- **AND** does not execute Varlock or inspect environment values

## MODIFIED Requirements

### Requirement: Projects compose independent knowledge sources

Arcantry MUST model OpenSpec, changelog, todo.txt and environment-contract artifacts as independently discovered and configured sources. Default discovery MUST recognize shared and private standard locations for every source kind. Each source MUST have a stable id, kind, path, management level and versioned adapter. Environment contracts MUST use the `environment-schema` kind and `env-spec@1` adapter. A project MUST be usable when any or all source kinds are absent. Skills MUST remain a separate procedural inventory rather than a project knowledge source kind.

#### Scenario: Shared and private standard sources coexist

- **WHEN** inspection finds `openspec`, `.local/openspec`, `CHANGELOG.md`, `.local/CHANGELOG.md`, `todo.txt`, `.local/todo.txt`, `.env.schema`, and `.local/.env.schema`
- **THEN** it reports every artifact independently with stable shared and private ids
- **AND** it does not merge, promote, resolve or hide either scope

#### Scenario: A directory has no recognized artifacts

- **WHEN** inspection runs in a directory without Git, configuration or recognized sources
- **THEN** inspection succeeds with an empty discovered stack
- **AND** no file is created

#### Scenario: A partial configuration describes one source

- **WHEN** a configuration manages or observes one source and other recognized artifacts exist
- **THEN** the configured source follows its declared management level
- **AND** unconfigured artifacts remain observed

### Requirement: Adoption persists discovered sources

When an active Arcantry configuration exists, adopting a discovered source MUST add or update that source in the active configuration. Adopting a standard missing write-capable source MUST initialize only that source and record it in the active configuration. A read-only source MUST already exist and adoption MUST register it without writing its content. The planned configuration MUST retain the source id, kind, path, visibility, management level, adapter and explicit dependencies, and MUST pass the same validation as a configuration read from disk before any operation is applied.

#### Scenario: A discovered source is adopted into shared configuration

- **WHEN** adoption is planned for a discovered source while `arcantry.toml` is active
- **THEN** the plan includes the source table in `arcantry.toml`
- **AND** applying the plan makes later inspection report that source as configured

#### Scenario: A standard missing source is adopted

- **WHEN** adoption is planned for a supported standard write-capable source that does not exist
- **THEN** the plan initializes only that source and records it in the active configuration
- **AND** both writes are protected by the same plan preconditions

#### Scenario: A private source is adopted through private configuration

- **WHEN** adoption adds a private source to the active private configuration
- **THEN** all planned private paths remain locally excluded from Git

#### Scenario: An existing read-only environment contract is adopted

- **WHEN** adoption is planned for an existing `environment-schema` source
- **THEN** the plan records it with `observe` management and `env-spec@1`
- **AND** does not write the source or any environment value file

## MODIFIED Requirements

### Requirement: Repository guidance uses explicit information layers

Arcantry MUST keep machine-local execution state in `.local/`, accepted product and engineering intent in configured OpenSpec sources, human release history in configured changelog sources, and quick task queues in configured todo.txt sources. Shared and private layers MUST remain independent unless an explicit reviewed operation promotes or relocates content. `.local/` MUST remain private when Git is present.

#### Scenario: A project uses both information scopes

- **WHEN** Arcantry inspects shared and private project sources
- **THEN** it reports each source with its scope and responsibility
- **AND** does not synchronize or merge them implicitly

#### Scenario: An established project adopts selected capabilities

- **WHEN** a project plans adoption for a subset of discovered sources
- **THEN** only the selected sources and explicit guidance are included in the plan
- **AND** unrelated project files remain untouched

#### Scenario: An established repository adopts Arcantry

- **WHEN** Arcantry initializes one repository scope
- **THEN** it manages only the configuration and guidance for that scope
- **AND** configured source responsibilities remain distinct

### Requirement: Adoption preserves existing repository ownership

Initialization and update MUST preserve unowned files and user-editable content. `repo init --scope shared` MUST create only shared TOML configuration and managed repository guidance. `repo init --scope private` MUST create only private TOML configuration, private managed guidance, and a `.local/` Git exclusion when applicable. Initialization MUST NOT create package-manager, runtime, task-runner, OpenSpec source, changelog, or todo scaffolding.

#### Scenario: A repository adopts private guidance

- **WHEN** `repo init --scope private` runs in an established repository
- **THEN** only `.local/arcantry.toml`, `.local/AGENTS.md`, and the required local Git exclusion are managed
- **AND** existing project source files remain unchanged

#### Scenario: A repository already has agent instructions

- **WHEN** Arcantry encounters an existing agent instruction file
- **THEN** it manages only its marked section when that section can be safely inserted
- **AND** preserves all surrounding content

#### Scenario: A non-Node project adopts OpenSpec

- **WHEN** OpenSpec is configured as a source in a project without Node tooling
- **THEN** Arcantry can inspect and validate that source
- **AND** repository initialization does not create package manifests, justfiles, or runtime version files

### Requirement: Removal is limited to verified managed artifacts

`arcantry repo remove --scope shared|private` MUST remove only artifacts or marked sections whose Arcantry ownership can be verified for the requested scope. It MUST NOT recursively remove user-authored content merely because it exists at a known path.

#### Scenario: A managed file contains durable user content

- **WHEN** the user removes Arcantry management for that scope
- **THEN** Arcantry removes its verified section or generated file
- **AND** preserves user-authored content that is not explicitly owned by Arcantry

#### Scenario: A managed directory contains durable user content

- **WHEN** the user removes Arcantry management
- **THEN** Arcantry removes only verified metadata or generated files for the selected scope
- **AND** preserves user-authored content that is not explicitly owned by Arcantry

### Requirement: Arcantry dogfoods public repository validation

Arcantry CI MUST initialize ephemeral private adoption state through the built public CLI and then run the public repository and skill validation commands against the Arcantry repository in addition to internal unit and schema checks. Initialization MUST remain idempotent and MUST NOT commit `.local` state.

#### Scenario: CI starts from a clean checkout

- **WHEN** the checkout has no private Arcantry configuration
- **THEN** the quality gate runs `arcantry repo init --scope private` before public validation
- **AND** the generated `.local` state remains uncommitted and excluded from Git

#### Scenario: CI verifies repository adoption

- **WHEN** the full repository quality gate runs
- **THEN** `arcantry repo validate` and `arcantry skills doctor` both inspect the current Arcantry repository

# repository-adoption Specification

## Purpose
Define safe repository adoption, diagnostics and removal while preserving project ownership.

## Requirements

### Requirement: Repository guidance uses explicit information layers

Arcantry MUST keep machine-local execution state in `.local/`, accepted product and engineering intent in OpenSpec, human release history in configured changelog sources and quick task queues in todo.txt sources. `.local/` MUST remain private when Git is present. Arcantry MUST NOT prescribe or manage `.docs/`.

#### Scenario: An established project adopts selected capabilities

- **WHEN** a project plans adoption for a subset of discovered sources
- **THEN** only the selected sources and explicit guidance are included in the plan
- **AND** unrelated project files and existing `.docs/` content remain untouched

#### Scenario: An established repository adopts Arcantry

- **WHEN** Arcantry initializes selected repository capabilities
- **THEN** private working state is routed to `.local/`
- **AND** configured source responsibilities remain distinct

### Requirement: Adoption preserves existing repository ownership

Initialization and update MUST preserve unowned files and user-editable content. New adoption MUST be capability-selective and MUST NOT create package-manager, runtime or task-runner scaffolding unless a separately configured capability requires it.

#### Scenario: A non-Node project adopts OpenSpec

- **WHEN** OpenSpec adoption is applied in a project without Node tooling
- **THEN** Arcantry creates only the selected OpenSpec artifacts
- **AND** does not create package manifests, justfiles or runtime version files

#### Scenario: A repository already has agent instructions

- **WHEN** Arcantry encounters an existing unowned agent instruction file
- **THEN** it leaves the file unchanged
- **AND** reports the unresolved adoption step instead of overwriting it

### Requirement: Removal is limited to verified managed artifacts

`arcantry repo remove` MUST remove only artifacts whose Arcantry ownership can be verified. It MUST NOT recursively remove user-authored content merely because it exists at a known path.

#### Scenario: A managed directory contains durable user content

- **WHEN** the user removes Arcantry management
- **THEN** Arcantry removes its verified metadata or generated files
- **AND** preserves user-authored content that is not explicitly owned by Arcantry

### Requirement: Repository diagnostics remain non-mutating

Inspection and doctor MUST explain detected state and repair paths without requiring configuration or Git. Strict validation MUST apply only to configured `validate` and `manage` sources. None of these commands may repair files implicitly.

#### Scenario: An unconfigured project is inspected

- **WHEN** no Arcantry configuration exists
- **THEN** inspection reports discovered sources and compatibility without treating missing adoption metadata as an error

#### Scenario: Managed metadata is outdated

- **WHEN** a configured source or managed section is outdated
- **THEN** doctor identifies the affected artifact and explicit repair action
- **AND** the artifact remains unchanged

### Requirement: Managed repository validation detects content drift

Repository validation MUST compare Arcantry-managed guidance with its canonical generated content instead of accepting ownership markers alone. `repo doctor` MUST provide an explicit repair action for each repairable diagnostic, while `repo validate` MUST remain deterministic and non-mutating.

#### Scenario: Managed guidance is outdated

- **WHEN** a managed section retains its ownership markers but differs from the current canonical content
- **THEN** `repo validate` fails without changing the file
- **AND** `repo doctor` identifies `repo update` as the repair action

### Requirement: Arcantry dogfoods public repository validation

Arcantry CI MUST initialize ephemeral private adoption state through the built public CLI and then run the public repository and skill validation commands against the Arcantry repository in addition to internal unit and schema checks. Initialization MUST remain idempotent and MUST NOT commit `.local` state.

#### Scenario: CI verifies repository adoption

- **WHEN** the full repository quality gate runs
- **THEN** `arcantry repo validate` and `arcantry skills doctor` both inspect the current Arcantry repository

#### Scenario: CI starts from a clean checkout

- **WHEN** the checkout has no private Arcantry configuration
- **THEN** the quality gate runs `arcantry repo init --docs none` before public validation
- **AND** the generated `.local` state remains uncommitted and excluded from Git

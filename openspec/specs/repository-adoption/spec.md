# repository-adoption Specification

## Purpose
Define safe repository adoption, diagnostics and removal while preserving project ownership.

## Requirements

### Requirement: Repository guidance uses explicit information layers

Arcantry MUST keep machine-local execution state in `.local/`, durable project guidance in `.docs/`, and accepted change intent and release metadata in OpenSpec. `.local/` MUST be excluded through `.git/info/exclude` and MUST NOT be promoted into shared project configuration by default.

#### Scenario: An established repository adopts Arcantry

- **WHEN** `arcantry repo init` initializes the repository
- **THEN** private working state is routed to `.local/`
- **AND** durable guidance and OpenSpec responsibilities remain distinct

### Requirement: Adoption preserves existing repository ownership

Initialization and update MUST preserve unowned files and user-editable content. Generated metadata MUST identify Arcantry-managed artifacts, and update MUST retain existing configuration choices unless the user explicitly replaces them.

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

`doctor` MUST explain detected state and actionable repair paths. `validate` MUST produce a deterministic pass or failure result suitable for CI. Neither command may repair files implicitly.

#### Scenario: Managed metadata is outdated

- **WHEN** the user runs `arcantry repo doctor`
- **THEN** the output identifies the affected artifact and the explicit update or repair action
- **AND** the artifact remains unchanged

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

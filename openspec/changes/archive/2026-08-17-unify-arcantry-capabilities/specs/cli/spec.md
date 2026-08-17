## ADDED Requirements

### Requirement: Arcantry exposes one namespaced command line interface

Arcantry MUST expose one `arcantry` binary. Repository operations MUST be nested under `arcantry repo`, and skill operations MUST be nested under `arcantry skills`. The CLI MUST NOT expose top-level install or update aliases.

#### Scenario: A user inspects the command surface

- **WHEN** the user runs `arcantry --help`
- **THEN** the help identifies the `repo` and `skills` command groups
- **AND** no alternate binary or top-level install/update command is required

### Requirement: Repository commands have stable responsibilities

The `repo` group MUST expose `init`, `update`, `doctor`, `validate`, and `remove`. `doctor` and `validate` MUST be read-only. Mutating commands MUST report the repository artifacts they create, update, or remove.

#### Scenario: Repository state is validated

- **WHEN** the user runs `arcantry repo validate` in a Git repository
- **THEN** Arcantry validates managed artifact metadata and repository policy without changing files

### Requirement: Skill commands support discovery and local adoption

The `skills` group MUST expose `list`, `inspect`, `link`, `unlink`, and `doctor`. Link and unlink operations MUST target one named canonical skill and MUST be idempotent for an already-correct state.

#### Scenario: One skill is linked

- **WHEN** the user runs `arcantry skills link <name>` for a valid catalog entry
- **THEN** the configured skill home points to that canonical package
- **AND** repeating the command does not create a duplicate installation

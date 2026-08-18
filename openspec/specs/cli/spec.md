# cli Specification

## Purpose
Define the stable public command surface for repository adoption and skill discovery.

## Requirements

### Requirement: Arcantry exposes one namespaced command line interface

Arcantry MUST expose one `arcantry` binary. Repository operations MUST be nested under `arcantry repo`, and skill operations MUST be nested under `arcantry skills`. The CLI MUST NOT expose top-level install or update aliases.

#### Scenario: A user inspects the command surface

- **WHEN** the user runs `arcantry --help`
- **THEN** the help identifies the `repo` and `skills` command groups
- **AND** no alternate binary or top-level install/update command is required

### Requirement: Repository commands have stable responsibilities

The `repo` group MUST expose `init`, `update`, `doctor`, `validate`, and `remove`. `init`, `update`, and `remove` MUST require `--scope shared|private` and MUST report the repository artifacts they create, update, or remove. `doctor` and `validate` MUST be read-only and MUST use explicit or discovered TOML configuration.

#### Scenario: Private repository state is initialized

- **WHEN** the user runs `arcantry repo init --scope private` in a Git repository
- **THEN** Arcantry creates or updates only the private configuration, private managed guidance, and local Git exclusion required by that scope

#### Scenario: Repository state is validated

- **WHEN** the user runs `arcantry repo validate` in a configured repository
- **THEN** Arcantry validates managed artifact metadata and repository policy without changing files

### Requirement: Skill commands support discovery and local adoption

The `skills` group MUST expose `list`, `inspect`, `link`, `unlink`, and `doctor`. Link and unlink operations MUST target one named canonical skill and MUST be idempotent for an already-correct state. The linker MUST support `--scope user|repo`, targeting standard Agent Skills directories, and MAY accept an exclusive advanced `--target` path.

#### Scenario: One skill is linked for a repository

- **WHEN** the user runs `arcantry skills link <name> --scope repo` for a valid catalog entry
- **THEN** `<repo>/.agents/skills/<name>` points to that canonical package
- **AND** repeating the command does not create a duplicate installation

#### Scenario: One skill is linked

- **WHEN** the user runs `arcantry skills link <name> --scope user` for a valid catalog entry
- **THEN** the standard user skill directory points to that canonical package
- **AND** repeating the command does not create a duplicate installation

#### Scenario: The destination is explicit

- **WHEN** the user supplies `--target`
- **THEN** Arcantry uses that destination instead of a scope-derived directory
- **AND** rejects a simultaneous `--scope`

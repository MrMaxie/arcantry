## MODIFIED Requirements

### Requirement: Repository commands have stable responsibilities

The `repo` group MUST expose `init`, `update`, `doctor`, `validate`, and `remove`. `init`, `update`, and `remove` MUST require `--scope shared|private` and MUST report the repository artifacts they create, update, or remove. `init` and `update` MAY accept `--compat claude` to add a branded adapter that imports the canonical guidance for the selected scope. `doctor` and `validate` MUST be read-only and MUST use explicit or discovered TOML configuration.

#### Scenario: Private repository state is initialized

- **WHEN** the user runs `arcantry repo init --scope private` in a Git repository
- **THEN** Arcantry creates or updates only the private configuration, private managed guidance, and local Git exclusion required by that scope
- **AND** it does not create a Claude compatibility file unless `--compat claude` is supplied

#### Scenario: Shared Claude compatibility is requested

- **WHEN** the user runs `arcantry repo update --scope shared --compat claude`
- **THEN** Arcantry preserves `AGENTS.md` as the canonical guidance
- **AND** ensures `CLAUDE.md` imports `@AGENTS.md` without replacing user-authored Claude content

#### Scenario: Private Claude compatibility is requested

- **WHEN** the user runs `arcantry repo update --scope private --compat claude`
- **THEN** Arcantry ensures `CLAUDE.local.md` imports `@.local/AGENTS.md`
- **AND** excludes the adapter through local Git metadata without changing `.gitignore`

#### Scenario: Repository state is validated

- **WHEN** the user runs `arcantry repo validate` in a configured repository
- **THEN** Arcantry validates managed artifact metadata and repository policy without changing files

### Requirement: Skill commands support discovery and local adoption

The `skills` group MUST expose `list`, `inspect`, `link`, `unlink`, and `doctor`. Link and unlink operations MUST target one named canonical skill and MUST be idempotent for an already-correct state. The linker MUST support `--scope user|repo|private`, MUST use standard `.agents/skills` destinations, MAY accept `--compat claude` for an additional alias, and MAY accept an exclusive advanced `--target` path. The CLI MUST NOT expose provider profiles through `--agent`.

#### Scenario: One skill is linked for a repository

- **WHEN** the user runs `arcantry skills link <name> --scope repo` for a valid catalog entry
- **THEN** `<repo>/.agents/skills/<name>` points to that canonical package
- **AND** repeating the command does not create a duplicate installation

#### Scenario: One skill is linked

- **WHEN** the user runs `arcantry skills link <name> --scope user` for a valid catalog entry
- **THEN** `~/.agents/skills/<name>` points to that canonical package
- **AND** repeating the command does not create a duplicate installation

#### Scenario: One private skill is linked

- **WHEN** the user runs `arcantry skills link <name> --scope private` for a valid package under `.local/skills`
- **THEN** the repository `.agents/skills/<name>` destination points to that private canonical package
- **AND** the managed destination is excluded through local Git metadata

#### Scenario: Claude compatibility is requested

- **WHEN** the user links a skill with `--compat claude`
- **THEN** the standard `.agents/skills` link and the corresponding `.claude/skills` link resolve to the same canonical package
- **AND** failure to prepare either destination leaves no new partial installation

#### Scenario: The destination is explicit

- **WHEN** the user supplies `--target`
- **THEN** Arcantry uses that destination instead of a scope-derived directory
- **AND** rejects simultaneous `--scope` or `--compat`

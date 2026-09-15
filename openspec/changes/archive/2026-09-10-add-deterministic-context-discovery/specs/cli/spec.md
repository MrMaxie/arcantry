## MODIFIED Requirements

### Requirement: Repository commands have stable responsibilities

The `repo` group MUST expose `inspect`, `plan`, `apply`, `init`, `update`, `doctor`, `validate`, and `remove`. `repo inspect` MUST be read-only, MUST support concise and detailed human views and stable JSON output, and MUST report the resolved project context without requiring configuration. `init`, `update`, and `remove` MUST require `--scope shared|private` and MUST report the repository artifacts they create, update, or remove. `doctor` and `validate` MUST remain read-only and MUST use explicit or discovered TOML configuration.

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

#### Scenario: Repository context is inspected concisely

- **WHEN** a user runs repository inspection without a detail override
- **THEN** the command reports the project boundary, selected configuration, recognized and absent standard sources, methodology summary and `.local` boundary health
- **AND** does not change repository or private state

#### Scenario: Repository context is consumed by an agent

- **WHEN** a caller requests JSON output
- **THEN** the result contains stable typed records for every detailed context field
- **AND** no operating-system-specific command output is exposed as the contract

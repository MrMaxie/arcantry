## ADDED Requirements

### Requirement: Skill status and update operations expose machine-readable contracts

The native CLI MUST provide `skills status [<name>] [--scope <user|repo>] [--target <path>] [--catalog-root <path>] [--check] [--json]`, `skills update <name> [--scope <user|repo>] [--target <path>] [--catalog-root <path>] [--compat claude]` and `skills apply --plan <path|->`. The global `--output` option MUST save an update preview without overwriting an existing file. Invalid scope, target and compatibility combinations MUST fail before network or filesystem mutation.

#### Scenario: Automation requests installed skill state

- **WHEN** `skills status --json` is invoked
- **THEN** stdout contains stable structured records and no human prose
- **AND** the command does not contact the network unless `--check` is supplied

#### Scenario: A saved update plan is applied

- **WHEN** `skills apply --plan` receives a valid unchanged plan
- **THEN** only the planned skill targets and ownership receipt change
- **AND** the command reports the applied version, revision and digest

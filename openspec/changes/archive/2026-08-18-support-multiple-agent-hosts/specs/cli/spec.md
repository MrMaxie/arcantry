## MODIFIED Requirements

### Requirement: Skill commands support discovery and local adoption

The `skills` group MUST expose `list`, `inspect`, `link`, `unlink`, and `doctor`. Link and unlink operations MUST target one named canonical skill and MUST be idempotent for an already-correct state. The linker MUST support `--scope user|repo`, MUST accept verified Codex, Claude Code, and Gemini CLI profiles through `--agent`, and MAY accept an exclusive advanced `--target` path. Omitting `--agent` MUST preserve the `.agents/skills` destination.

#### Scenario: One skill is linked for a repository

- **WHEN** the user runs `arcantry skills link <name> --scope repo` for a valid catalog entry
- **THEN** `<repo>/.agents/skills/<name>` points to that canonical package
- **AND** repeating the command does not create a duplicate installation

#### Scenario: One skill is linked

- **WHEN** the user runs `arcantry skills link <name> --scope user` for a valid catalog entry
- **THEN** the standard user skill directory points to that canonical package
- **AND** repeating the command does not create a duplicate installation

#### Scenario: One Claude Code skill is linked for a repository

- **WHEN** the user runs `arcantry skills link <name> --scope repo --agent claude` for a valid catalog entry
- **THEN** `<repo>/.claude/skills/<name>` points to that canonical package
- **AND** repeating the command does not create a duplicate installation

#### Scenario: One Gemini CLI skill is linked for a user

- **WHEN** the user runs `arcantry skills link <name> --scope user --agent gemini` for a valid catalog entry
- **THEN** `~/.gemini/skills/<name>` points to that canonical package

#### Scenario: The default destination remains portable

- **WHEN** the user runs `arcantry skills link <name> --scope user` without an agent profile
- **THEN** `~/.agents/skills/<name>` points to that canonical package

#### Scenario: The destination is explicit

- **WHEN** the user supplies `--target`
- **THEN** Arcantry uses that destination instead of a scope-derived directory
- **AND** rejects a simultaneous `--scope` or `--agent`

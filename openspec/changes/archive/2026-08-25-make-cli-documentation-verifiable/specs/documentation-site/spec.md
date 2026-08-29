## ADDED Requirements

### Requirement: CLI reference tables preserve literal command alternatives

Every rendered Markdown and MDX table MUST preserve the header's column count in each body row. CLI command tables MUST display option alternatives such as `shared|private` as literal command text. Verification MUST cover the complete documentation surface rather than selected examples.

#### Scenario: An option uses a vertical bar

- **WHEN** a command table contains alternative option values separated by `|`
- **THEN** the rendered row keeps the alternative inside one command cell
- **AND** no formatting marker or unintended column is exposed

### Requirement: Public CLI trust claims identify executable evidence

Claims about read-only behavior, network behavior, repository writes, removal boundaries, rollback and private data MUST be no broader than named executable evidence. Unsupported or incompletely executed platforms MUST be described as contract targets rather than current execution evidence.

#### Scenario: A trust claim loses executable evidence

- **WHEN** its mapped executable scenario is removed or renamed
- **THEN** documentation verification fails until the claim is narrowed or evidence is restored

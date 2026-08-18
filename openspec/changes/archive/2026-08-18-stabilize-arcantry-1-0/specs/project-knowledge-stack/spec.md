## MODIFIED Requirements

### Requirement: Configuration is optional, singular and versioned

Arcantry MUST use the same versioned TOML schema for shared `arcantry.toml` and private `.local/arcantry.toml`. An explicit configuration path MUST take precedence. Otherwise Arcantry MUST walk from the requested directory toward its ancestors, checking `.local/arcantry.toml` before `arcantry.toml` at each directory, and MUST use the first match without merging files. A private configuration MUST resolve the containing project directory as its default project root. The configuration MAY declare an Arcantry SemVer compatibility range and MUST use independently versioned source adapters. Absolute source paths MUST be rejected by default on every supported operating system and MAY be accepted only for an explicitly supplied external configuration.

#### Scenario: Both configurations exist at one project boundary

- **WHEN** discovery reaches a directory containing both `.local/arcantry.toml` and `arcantry.toml`
- **THEN** the private configuration is active
- **AND** inspection reports the shared configuration as shadowed rather than merging it

#### Scenario: Explicit configuration is supplied

- **WHEN** a caller provides `--config` and a project directory
- **THEN** Arcantry uses that configuration regardless of discovered files
- **AND** does not write it into the project

#### Scenario: Explicit configuration is outside the project

- **WHEN** a caller provides `--config` and a project directory
- **THEN** Arcantry uses that configuration without writing it into the project

#### Scenario: A newer tool reads an older supported adapter

- **WHEN** a source pins a supported earlier adapter family
- **THEN** the newer tool continues to use that adapter without rewriting the source

#### Scenario: An absolute source path is configured locally

- **WHEN** a configuration uses an operating-system-native absolute source path without the external configuration opt-in
- **THEN** validation rejects the source path consistently on every supported operating system

#### Scenario: A private configuration is discovered

- **WHEN** `.local/arcantry.toml` is selected
- **THEN** relative source paths and the default project boundary resolve from the directory containing `.local`

### Requirement: Structural transitions are explicit and drift-safe

Inspect and plan operations MUST NOT write project state. Shared and private sources MUST remain independent. Promotion, relocation, or reconciliation between them MUST require an explicit apply step using a serializable plan whose input hashes, tool version, and adapter versions still match. Arcantry MUST NOT automatically synchronize or merge source content.

#### Scenario: Shared and private sources differ

- **WHEN** inspection detects related content in both scopes
- **THEN** Arcantry reports the drift and available explicit actions
- **AND** leaves both sources unchanged

#### Scenario: An input changes after planning

- **WHEN** apply detects an input hash different from the plan
- **THEN** it refuses all planned writes

#### Scenario: A source is relocated

- **WHEN** relocation is applied
- **THEN** the target is written and verified before any separately requested source deletion

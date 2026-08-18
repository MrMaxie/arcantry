## MODIFIED Requirements

### Requirement: Configuration is optional, singular and versioned

Arcantry MUST accept an explicit configuration path or discover the nearest ancestor `arcantry.toml` without merging multiple files. The configuration MUST declare its own format version, MAY declare an Arcantry SemVer compatibility range and MUST use independently versioned source adapters. Absolute source paths MUST be rejected by default on every supported operating system and MAY be accepted only for an explicitly supplied external configuration.

#### Scenario: Explicit configuration is outside the project

- **WHEN** a caller provides `--config` and a project directory
- **THEN** Arcantry uses that configuration without writing it into the project

#### Scenario: A newer tool reads an older supported adapter

- **WHEN** a source pins a supported earlier adapter family
- **THEN** the newer tool continues to use that adapter without migrating or rewriting the source

#### Scenario: An absolute source path is configured locally

- **WHEN** a configuration uses an operating-system-native absolute source path without the external configuration opt-in
- **THEN** validation rejects the source path consistently on every supported operating system

## MODIFIED Requirements

### Requirement: Configuration is optional, singular and versioned

Arcantry MUST use the same versioned TOML schema for shared `arcantry.toml` and private `.local/arcantry.toml`. An explicit configuration path MUST take precedence. Otherwise Arcantry MUST walk from the requested directory toward its ancestors, checking `.local/arcantry.toml` before `arcantry.toml` at each directory, and MUST use the first match without merging files. A private configuration MUST resolve the containing project directory as its default project root. An auto-discovered configuration and its declared project root MUST resolve through existing links inside that containing project boundary. The configuration MAY declare an Arcantry SemVer compatibility range and MUST use independently versioned source adapters. Absolute source paths MUST be rejected by default on every supported operating system and MAY be accepted only for an explicitly supplied external configuration.

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

#### Scenario: A discovered root leaves the project

- **WHEN** an auto-discovered shared or private configuration declares an absolute, parent-traversing or linked project root outside its containing project
- **THEN** resolution fails before repository sources are inspected

#### Scenario: An explicit external configuration selects external state

- **WHEN** the user explicitly supplies an external configuration with an absolute project root or source
- **THEN** that declared external location remains available
- **AND** unrelated project-relative paths cannot escape through filesystem links

## ADDED Requirements

### Requirement: Source reads stay inside their authority

Every configured or standard project read MUST resolve through existing links under the canonical project root unless an explicitly supplied external configuration declares that absolute source. Internal links MAY be read only when their canonical targets remain inside an authorized root.

#### Scenario: A source link targets a host file

- **WHEN** guidance, todo, OpenSpec, changelog, release or transition input resolves outside its authorized root
- **THEN** CLI and MCP processing fail without reading or returning the external content

### Requirement: Dependency validation is bounded

Configuration dependency cycles MUST be validated in linear time relative to graph nodes and edges, preserve deterministic cycle reporting and treat dependencies absent from the local graph as already external.

#### Scenario: A long valid dependency chain is inspected

- **WHEN** repository configuration contains a long acyclic dependency chain
- **THEN** validation completes through one indegree traversal rather than repeated full pending-set scans

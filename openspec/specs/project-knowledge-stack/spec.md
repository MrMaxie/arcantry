# project-knowledge-stack Specification

## Purpose

Define how Arcantry discovers, configures and safely changes independently versioned project knowledge sources without imposing a repository shape.

## Requirements

### Requirement: Projects compose independent knowledge sources

Arcantry MUST model OpenSpec, changelog and todo.txt artifacts as independently discovered and configured sources. Each source MUST have a stable id, kind, path, management level and versioned adapter. A project MUST be usable when any or all source kinds are absent.

#### Scenario: A directory has no recognized artifacts

- **WHEN** inspection runs in a directory without Git, configuration or recognized sources
- **THEN** inspection succeeds with an empty discovered stack
- **AND** no file is created

#### Scenario: A partial configuration describes one source

- **WHEN** a configuration manages one source and other recognized artifacts exist
- **THEN** the configured source follows its declared management level
- **AND** unconfigured artifacts remain observed

### Requirement: Source relationships are explicit and unambiguous

Source dependencies MUST form a directed acyclic graph. Managed changelog sources MUST derive release meaning from one or more OpenSpec sources. Overlapping managed OpenSpec authorities for the same project scope MUST be rejected.

#### Scenario: A source dependency cycle exists

- **WHEN** configuration creates a dependency cycle
- **THEN** validation fails before repository changes are planned

#### Scenario: A changelog has no semantic authority

- **WHEN** a changelog is discovered without OpenSpec
- **THEN** Arcantry may observe or validate the changelog
- **AND** MUST NOT generate new release meaning for it

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

### Requirement: Configuration has a TOML Schema contract

Arcantry MUST publish a TOML Schema 1.0.0 `.tosd` document for `arcantry.toml`. Generated configuration MAY include the reserved `[toml-schema]` location metadata. Runtime validation MUST preserve parsed application data and MUST enforce constraints that are not expressible in the schema.

#### Scenario: An editor discovers the configuration schema

- **WHEN** `arcantry.toml` contains a TOML Schema location
- **THEN** the location resolves to the published `.tosd` schema
- **AND** the schema describes dynamic source ids as a collection of typed source tables

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

### Requirement: Todo sources preserve the todo.txt contract

Arcantry MUST recognize root `todo.txt` and private `.local/todo.txt` independently. Todo operations MUST preserve untouched lines, line endings, BOM, arbitrary projects, contexts and `key:value` metadata. Arcantry MUST NOT impose inbox or outbox tags.

#### Scenario: Both todo sources exist

- **WHEN** a mutating todo operation does not select a source
- **THEN** it fails without changing either file

### Requirement: Skills remain procedural capabilities

Skills MUST remain independently usable without repository adoption. Skill compatibility metadata MAY describe supported source kinds and adapter versions, but MUST NOT make skill guidance authoritative project state or install skills automatically.

#### Scenario: A user links only a skill

- **WHEN** no project configuration or recognized source exists
- **THEN** skill discovery and linking continue to work without creating repository artifacts

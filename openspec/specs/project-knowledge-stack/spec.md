# project-knowledge-stack Specification

## Purpose

Define how Arcantry discovers, configures and safely changes independently versioned project knowledge sources without imposing a repository shape.

## Requirements

### Requirement: Projects compose independent knowledge sources

Arcantry MUST model OpenSpec, changelog and todo.txt artifacts as independently discovered and configured sources. Default discovery MUST recognize shared and private standard locations for every source kind. Each source MUST have a stable id, kind, path, management level and versioned adapter. A project MUST be usable when any or all source kinds are absent. Skills MUST remain a separate procedural inventory rather than a project knowledge source kind.

#### Scenario: Shared and private standard sources coexist

- **WHEN** inspection finds `openspec`, `.local/openspec`, `CHANGELOG.md`, `.local/CHANGELOG.md`, `todo.txt`, and `.local/todo.txt`
- **THEN** it reports every artifact independently with stable shared and private ids
- **AND** it does not merge, promote or hide either scope

#### Scenario: A directory has no recognized artifacts

- **WHEN** inspection runs in a directory without Git, configuration or recognized sources
- **THEN** inspection succeeds with an empty discovered stack
- **AND** no file is created

#### Scenario: A partial configuration describes one source

- **WHEN** a configuration manages one source and other recognized artifacts exist
- **THEN** the configured source follows its declared management level
- **AND** unconfigured artifacts remain observed

### Requirement: Source relationships are explicit and unambiguous

Source dependencies MUST form a directed acyclic graph. Managed changelog sources MUST derive release meaning from one or more OpenSpec sources. A shared changelog MUST NOT depend on private OpenSpec. A private changelog MAY depend on shared or private OpenSpec. Overlapping managed OpenSpec authorities for the same project scope MUST be rejected.

#### Scenario: A shared changelog depends on private intent

- **WHEN** configuration makes a shared changelog depend on a private OpenSpec source
- **THEN** validation fails before repository changes are planned

#### Scenario: A private changelog composes intent

- **WHEN** a private changelog depends on shared OpenSpec, private OpenSpec, or both
- **THEN** validation accepts the relationship when the remaining graph is valid

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

Arcantry MUST recognize root `todo.txt` and private `.local/todo.txt` independently. The `todo-txt@1` adapter and todo mutation commands MUST treat each non-empty physical line as one task. Content that Arcantry creates or directly rewrites MUST follow the official todo.txt baseline: priority MAY appear first, a creation date MAY follow the priority or begin an unprioritized task, projects, contexts and `key:value` metadata MUST remain optional, and a completed task MUST begin with lowercase `x`, its completion date, an optional creation date and the task text in that order. Todo operations MUST preserve untouched lines, line endings, BOM, trailing-newline state, arbitrary projects, contexts and `key:value` metadata. Older or noncanonical lines MUST NOT block a scoped mutation of different content and MUST NOT be normalized implicitly. Arcantry MUST NOT impose inbox or outbox tags.

#### Scenario: Both todo sources exist

- **WHEN** a mutating todo operation does not select a source
- **THEN** it fails without changing either file

#### Scenario: A selected missing todo source receives its first task

- **WHEN** a caller previews adding one non-empty task to an explicitly selected missing root, private or configured todo source
- **THEN** the plan contains a valid one-task-per-line todo.txt document using `todo-txt@1`
- **AND** neither the preview nor ordinary inspection creates the file
- **AND** only explicit apply writes the planned source

#### Scenario: A legacy queue receives a new task

- **WHEN** a selected todo source contains a BOM, CRLF line endings and older noncanonical lines
- **THEN** adding a task writes the new task as one non-empty line
- **AND** preserves every existing line and file characteristic without normalizing or rejecting the queue

#### Scenario: A minimal task omits optional fields

- **WHEN** a caller adds a task without priority, dates, projects, contexts or metadata
- **THEN** the adapter accepts the task without inventing any optional field

#### Scenario: A task is completed

- **WHEN** a caller completes an incomplete task with a valid completion date
- **THEN** the rewritten line starts with lowercase `x` and the completion date
- **AND** keeps an existing creation date before the task text
- **AND** preserves an existing priority only through optional metadata rather than the active priority prefix

#### Scenario: A task moves between queues

- **WHEN** a caller explicitly moves one task between selected todo sources
- **THEN** the raw task line is preserved
- **AND** unrelated source and target lines are not normalized

### Requirement: Skills remain procedural capabilities

Skills MUST remain independently usable without repository adoption. Skill compatibility metadata MAY describe supported source kinds and adapter versions, but MUST NOT make skill guidance authoritative project state or install skills automatically.

#### Scenario: A user links only a skill

- **WHEN** no project configuration or recognized source exists
- **THEN** skill discovery and linking continue to work without creating repository artifacts

### Requirement: Adoption persists discovered sources

When an active Arcantry configuration exists, adopting a discovered or standard missing source MUST add or update that source in the active configuration. The planned configuration MUST retain the source id, kind, path, visibility, management level, adapter and explicit dependencies, and MUST pass the same validation as a configuration read from disk before any operation is applied.

#### Scenario: A discovered source is adopted into shared configuration

- **WHEN** adoption is planned for a discovered source while `arcantry.toml` is active
- **THEN** the plan includes a managed source table in `arcantry.toml`
- **AND** applying the plan makes later inspection report that source as configured

#### Scenario: A standard missing source is adopted

- **WHEN** adoption is planned for a supported standard source that does not exist
- **THEN** the plan initializes only that source and records it in the active configuration
- **AND** both writes are protected by the same plan preconditions

#### Scenario: A private source is adopted through private configuration

- **WHEN** adoption adds a private source to the active private configuration
- **THEN** all planned private paths remain locally excluded from Git

### Requirement: Adoption dependencies are explicit

Source adoption MUST accept explicit source dependencies and MUST reject the planned configuration when those dependencies are missing, cyclic or violate source privacy rules. Arcantry MUST NOT infer a dependency that the caller did not provide.

#### Scenario: A managed changelog declares OpenSpec authority

- **WHEN** a caller adopts a changelog with an explicit OpenSpec source dependency
- **THEN** the configured changelog records that dependency

#### Scenario: A shared source depends on private intent

- **WHEN** an adoption request would make a shared managed changelog depend on private OpenSpec
- **THEN** planning reports a conflict and returns no operations

### Requirement: Release configuration is versioned by adapter contract

Arcantry MUST preserve `openspec-release@1` behavior and MAY use `openspec-release@2` only when configured explicitly. The v2 adapter MUST support `single`, `independent` and `composed` topologies. An omitted v2 topology MUST mean `single` and MUST retain the flat release configuration shape.

#### Scenario: An existing v1 project is inspected

- **WHEN** a project configures `openspec-release@1`
- **THEN** Arcantry applies the existing single-release behavior
- **AND** does not reinterpret its configuration or manifests as v2

#### Scenario: A v2 topology is omitted

- **WHEN** a project configures `openspec-release@2` with flat release fields and no topology
- **THEN** Arcantry treats the release topology as `single`

### Requirement: Multi-unit release ownership is explicit

An `independent` or `composed` release system MUST define at least one named unit. Every unit MUST own a unique manifest path, managed changelog source, version-source path and tag prefix. Every selector MUST name a configured OpenSpec source. A selector without components MUST claim that entire source exclusively. Selector lists MUST be combined with OR semantics, and component ownership for a source MUST be disjoint across units.

#### Scenario: Units select components from a shared source

- **WHEN** two units select disjoint component ids from the same OpenSpec source
- **THEN** each matching release-bearing change belongs to the unit whose selector matches its components

#### Scenario: Source ownership overlaps

- **WHEN** one unit claims a whole source or two units claim the same source component
- **THEN** configuration validation fails before release state is planned

### Requirement: Release-unit dependencies match the topology

Independent units MUST NOT declare dependencies. A composed release system MUST contain at least one dependency edge and MUST form a directed acyclic graph. Shared child units and multiple roots MUST be supported, while cycles and unknown dependency ids MUST be rejected.

#### Scenario: A composed graph shares a child

- **WHEN** two parent units depend on the same child and the graph is acyclic
- **THEN** the release system is valid

#### Scenario: A unit dependency cycle exists

- **WHEN** configured release-unit dependencies form a cycle
- **THEN** validation fails before repository changes are planned

### Requirement: OpenSpec release classification follows each change schema

For every active or archived OpenSpec change, Arcantry MUST resolve the schema from `.openspec.yaml` with fallback to the source `config.yaml`, then load `<source>/schemas/<schema>/schema.yaml`. A schema that declares a release artifact generating `release.md` MUST require a valid release artifact. A schema without a release artifact MUST classify the change as non-release, MUST reject a present `release.md`, and MUST exclude the change from release assignment. An unresolved schema or a release artifact generated at another path MUST fail validation.

#### Scenario: A documentation-only schema has no release artifact

- **WHEN** a change uses a resolvable schema that does not declare a release artifact and has no `release.md`
- **THEN** release planning skips the change without assigning a SemVer impact

#### Scenario: A release-bearing schema is incomplete

- **WHEN** a change uses a schema whose release artifact generates `release.md` but the file is absent or invalid
- **THEN** release validation fails with the change and schema identified

### Requirement: Every source transition has executable native evidence

The native contract suite MUST exercise `preserve`, `adopt`, `rebind`, `cutover`, `migrate` and `relocate` through serialized repository plans. It MUST verify preview behavior, unchanged-input application and rejection after relevant input drift.

#### Scenario: A transition strategy regresses

- **WHEN** any supported transition cannot produce or safely apply its documented plan
- **THEN** native contract verification fails before the change can qualify as complete

### Requirement: Project plans preserve the pre-apply tree on failure

Project-plan execution MUST support write, delete and delete-tree operations as one reversible transaction. It MUST reject root mismatch, unauthorized paths and input drift before commit, verify each committed result and remove transaction-created parent directories or artifacts during rollback.

#### Scenario: A generated plan fails at any bounded operation position

- **WHEN** a valid plan contains a generated sequence of writes and deletions and execution fails at a staged, committed, verification or reversible finalization step
- **THEN** the complete affected tree is byte-for-byte and structure-for-structure equal to its state before apply

## MODIFIED Requirements

### Requirement: Repository guidance uses explicit information layers

Arcantry MUST keep machine-local execution state in `.local/`, accepted product and engineering intent in configured shared or private OpenSpec sources, human release history in configured shared or private changelog sources, quick task queues in configured todo.txt sources, and procedural capabilities in independently managed skill packages. `AGENTS.md` and `.agents` MUST be the universal guidance and skill surfaces. Shared and private layers MUST remain independent unless an explicit reviewed operation promotes or relocates content. `.local/` MUST remain private when Git is present unless the configured default remote branch already tracks it; that established repository policy MUST be preserved and reported as a conflict with Arcantry's private-local convention.

#### Scenario: A project uses both information scopes

- **WHEN** Arcantry inspects shared and private project sources
- **THEN** it reports each source with its scope and responsibility
- **AND** does not synchronize or merge them implicitly

#### Scenario: An established project adopts selected capabilities

- **WHEN** a project plans adoption for a subset of discovered sources
- **THEN** only the selected sources and explicit guidance are included in the plan
- **AND** unrelated project files remain untouched

#### Scenario: An established repository adopts Arcantry

- **WHEN** Arcantry initializes one repository scope
- **THEN** it manages only the configuration and universal guidance for that scope
- **AND** configured source responsibilities remain distinct

#### Scenario: The remote repository already tracks local state

- **WHEN** the configured default remote branch tracks content under `.local/`
- **THEN** adoption preserves that established repository policy
- **AND** reports its conflict with Arcantry's private-local convention

#### Scenario: Only the local index tracks local state

- **WHEN** `.local/` is absent from the configured default remote branch but present in the current index
- **THEN** adoption plans removal from the index as a separate explicitly authorized operation
- **AND** preserves the working files

#### Scenario: Claude compatibility is present

- **WHEN** repository diagnostics find managed Claude guidance or skill adapters
- **THEN** they identify the related `AGENTS.md`, `.local/AGENTS.md`, or canonical skill package as the source
- **AND** do not report the adapter as independent project guidance or a duplicate skill

### Requirement: Adoption preserves existing repository ownership

Initialization and update MUST preserve unowned files and user-editable content. `repo init --scope shared` MUST create only shared TOML configuration and managed repository guidance. `repo init --scope private` MUST create only private TOML configuration, private managed guidance, and a `.local/` Git exclusion when applicable. Initialization MUST NOT create package-manager, runtime, task-runner, OpenSpec source, changelog, todo or skill scaffolding. Claude compatibility files MUST require an explicit compatibility option and MUST preserve user-authored content outside the managed import.

#### Scenario: A repository adopts private guidance

- **WHEN** `repo init --scope private` runs in an established repository
- **THEN** only `.local/arcantry.toml`, `.local/AGENTS.md`, and the required local Git exclusion are managed
- **AND** existing project source files remain unchanged

#### Scenario: A non-Node project adopts OpenSpec

- **WHEN** OpenSpec is configured as a source in a project without Node tooling
- **THEN** Arcantry can inspect and validate that source
- **AND** repository initialization does not create package manifests, justfiles, or runtime version files

#### Scenario: A repository already has agent instructions

- **WHEN** Arcantry encounters an existing agent instruction file
- **THEN** it manages only its marked section when that section can be safely inserted
- **AND** preserves all surrounding content

#### Scenario: Existing Claude guidance is adapted

- **WHEN** compatibility is requested and the relevant Claude file already contains user-authored instructions
- **THEN** Arcantry inserts or refreshes only its managed import
- **AND** preserves all surrounding content

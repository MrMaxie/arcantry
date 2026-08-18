## MODIFIED Requirements

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

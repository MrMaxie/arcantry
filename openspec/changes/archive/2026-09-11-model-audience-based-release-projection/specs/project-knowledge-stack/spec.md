## MODIFIED Requirements

### Requirement: Release configuration is versioned by adapter contract

Arcantry MUST expose `openspec-release@1` as its single final release adapter. That adapter MUST support `single`, `independent` and `composed` topologies. An omitted topology MUST mean `single` and MUST retain the flat release configuration shape. Historical manifest formats MAY remain parseable, but no second public adapter identifier or adapter migration command MAY be required for unpublished prepublication contracts.

#### Scenario: A single-release project is inspected

- **WHEN** a project configures `openspec-release@1` without a topology
- **THEN** Arcantry applies the flat single-release behavior

#### Scenario: An existing v1 project is inspected

- **WHEN** a project configures `openspec-release@1`
- **THEN** Arcantry applies the final adapter contract without requiring migration
- **AND** historical release manifests remain parseable

#### Scenario: A v2 topology is omitted

- **WHEN** a project configuration originating before publication omits topology
- **THEN** Arcantry treats the release topology as `single` under `openspec-release@1`

#### Scenario: A multi-unit topology is configured

- **WHEN** a project configures `openspec-release@1` with independent or composed units
- **THEN** Arcantry applies the explicit unit ownership and dependency contract
- **AND** does not require another adapter version

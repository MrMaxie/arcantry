## ADDED Requirements

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

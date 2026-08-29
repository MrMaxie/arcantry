## MODIFIED Requirements

### Requirement: Release manifests only group changes

A release manifest MUST contain its format, release unit, version, release date and explicit list of archived change ids. A composed unit manifest MUST also pin the exact versions of all direct dependencies. A manifest MUST NOT duplicate change prose or specification deltas. Assignment uniqueness MUST be enforced per `(unit, change)` pair, so one change MAY be released by multiple matching units at different versions or times.

#### Scenario: A release is cut

- **WHEN** a release manifest is created
- **THEN** it groups archived change ids without copying their prose or deltas

#### Scenario: One outcome affects multiple units

- **WHEN** an archived change matches more than one release unit
- **THEN** each unit may assign that change in its own manifest stream
- **AND** assigning it in one unit does not assign it in another

#### Scenario: A composed manifest is created

- **WHEN** a composed unit release is cut
- **THEN** its manifest pins the latest released version of every direct dependency
- **AND** contains no copied release prose

### Requirement: SemVer impact belongs to the change

Every release-bearing change MUST declare `patch`, `minor` or `major` impact and MAY declare `unit_impacts` overrides for matched release units. `openspec-release@2` MUST reject `impact: none`. A unit release version MUST be computed from the highest effective impact for that unit. A change MAY acknowledge direct dependency adoption through `dependency_updates`, but dependency movement alone MUST NOT create or bump a parent release.

#### Scenario: A release contains mixed impacts

- **WHEN** a unit release plan contains more than one archived change
- **THEN** the next version is computed from the highest effective impact for that unit

#### Scenario: A release is not published

- **WHEN** completed changes are retained only in the repository
- **THEN** they still produce a new SemVer manifest, aligned unit version sources and a unit changelog version section

#### Scenario: One outcome has different unit impacts

- **WHEN** a release-bearing change matches multiple units and declares a unit impact override
- **THEN** each unit plan uses its override or the global impact fallback

#### Scenario: A child unit releases independently

- **WHEN** a child unit releases a newer version without an eligible parent change acknowledging it
- **THEN** the parent has a pending dependency update
- **AND** no parent version or manifest changes automatically

### Requirement: Release state is validated as a whole

Repository validation MUST validate configured release units, schema-aware release classification, unit-scoped assignments, version sources, dependency pins and generated changelogs. Every release-bearing archived change MUST match at least one unit. In multi-unit mode, normal unscoped checking MUST validate every unit. Sealed checking MUST require one unit, require a clean worktree and the exact HEAD commit that introduced that unit's latest manifest, and scope active or unassigned change checks to that unit. Composed sealed checking MUST also verify direct dependency manifests, pins and version sources at that commit. Later releases of other units MUST NOT invalidate an earlier unit seal.

#### Scenario: Release metadata drifts

- **WHEN** a manifest, unit version source, dependency pin or generated changelog violates a release invariant
- **THEN** repository validation fails with the violated invariant

#### Scenario: Repository work follows the newest release seal

- **WHEN** sealed validation finds target-unit work after the commit introducing that unit's newest release manifest
- **THEN** validation fails and requires that work to be represented by archived OpenSpec intent and a newer unit release

#### Scenario: A malformed release title contains excessive whitespace

- **WHEN** validation reads a release artifact whose title line contains no title after a long whitespace sequence
- **THEN** validation rejects the title in work proportional to the artifact size

#### Scenario: A release-bearing change has no owner

- **WHEN** an archived release-bearing change matches no configured unit
- **THEN** validation fails instead of silently omitting it

#### Scenario: One unit is sealed while another has pending work

- **WHEN** sealed checking targets a unit whose own release state is complete and other units have unrelated active or unassigned changes
- **THEN** those unrelated units do not block the target unit seal

## ADDED Requirements

### Requirement: Release units own their changelog scope

Independent topologies MUST render only per-unit changelogs and MUST NOT synthesize a root changelog. In composed topologies, a parent changelog MAY serve as the product summary but MUST include only outcomes selected for that parent and MUST NOT copy child entries automatically.

#### Scenario: A child change has no parent outcome

- **WHEN** a child release is rendered and no parent change selects or acknowledges it
- **THEN** the child entry appears only in the child changelog
- **AND** the parent changelog remains unchanged

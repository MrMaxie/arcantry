## MODIFIED Requirements

### Requirement: OpenSpec is the source of release history

Arcantry MUST derive public change history from archived OpenSpec changes and release manifests.

#### Scenario: Release history is generated

- **WHEN** changelog or component history is rendered
- **THEN** every entry resolves to an archived OpenSpec change grouped by a release manifest

### Requirement: Archive is the delivery boundary

A change MUST NOT appear in public release history until its OpenSpec change is archived.

#### Scenario: An active change exists

- **WHEN** public release history is generated before the change is archived
- **THEN** the active change is absent from that history

### Requirement: Release manifests only group changes

A release manifest MUST contain a version, release date and explicit list of archived change ids. It MUST NOT duplicate change prose or specification deltas.

#### Scenario: A release is cut

- **WHEN** a release manifest is created
- **THEN** it groups archived change ids without copying their prose or deltas

### Requirement: SemVer impact belongs to the change

Every releasable change MUST declare `none`, `patch`, `minor` or `major` impact in its release artifact. The release version MUST be computed from the highest impact in the release.

#### Scenario: A release contains mixed impacts

- **WHEN** the release plan contains more than one archived change
- **THEN** the next version is computed from the highest declared impact

### Requirement: Release components identify affected product surfaces

Every releasable change MUST list affected components in its release artifact using stable component ids such as `docs`, `schemas`, `tooling`, `cli`, `catalog`, `repository-adoption` or `skill:<name>`.

#### Scenario: Component history is requested

- **WHEN** history is generated for one product surface
- **THEN** entries are selected by stable component ids from archived release artifacts

### Requirement: Generated changelog is reproducible

The changelog MUST be generated from release manifests and archived change release artifacts in deterministic version order.

#### Scenario: Changelog generation repeats

- **WHEN** generation runs twice against unchanged release inputs
- **THEN** both outputs are byte-for-byte identical

### Requirement: Release state is validated as a whole

Repository validation MUST fail when a manifest references a missing or active change, reuses a change id, breaks descending SemVer order, contains an invalid component id or when generated changelog content drifts.

#### Scenario: Release metadata drifts

- **WHEN** a manifest or generated changelog violates a release invariant
- **THEN** repository validation fails with the violated invariant

### Requirement: Repository commands are stable

The repository MUST expose stable commands for checking, building, serving, validating changes, planning releases, cutting releases and rendering the changelog.

#### Scenario: A contributor inspects repository commands

- **WHEN** they use the documented command interface
- **THEN** check, build, serve, validation, release planning, release cutting, and changelog rendering remain available

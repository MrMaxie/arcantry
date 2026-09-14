## ADDED Requirements

### Requirement: Release classification separates audience, impact and inclusion

`openspec-release@1` MUST classify each release-bearing change independently by SemVer impact, affected components, intended audiences, observable impact and changelog inclusion policy. Audiences MUST be non-empty project-defined slugs. Observable impact MUST be `customer-outcome`, `user-felt`, `significant-technical` or `maintenance`. Intended audiences and observable impact MUST NOT replace SemVer calculation or manifest assignment.

#### Scenario: Maintenance work has no audience-facing entry

- **WHEN** an accepted release-bearing change is classified for omission from the changelog
- **THEN** it remains assigned to its matching release manifest and contributes its effective SemVer impact
- **AND** deterministic rendering emits no audience-facing entry for that change

#### Scenario: Technical work is significant to one audience

- **WHEN** a change affects developer or operator decisions without changing an end-user task
- **THEN** classification records that audience and observable impact independently
- **AND** inclusion follows the configured changelog policy instead of a binary public-or-internal inference

### Requirement: Changelog projection supports explicit many-to-one stories

Several accepted release-bearing changes MAY form one changelog entry only through an explicit projection group. A group MUST have a stable id, compatible audiences and category, group-owned title and body, and a complete ordered membership list. Every member MUST resolve to one matching release assignment. Rendering MUST expose traceability from the entry to every member and MUST reject overlapping, missing or differently classified ownership.

#### Scenario: Several outcomes form one release story

- **WHEN** accepted changes join one valid projection group
- **THEN** rendering produces one audience-facing entry from the projection group's prose
- **AND** the generated result remains traceable to every member change id

#### Scenario: One change enters competing stories

- **WHEN** a change is assigned to overlapping projection groups for the same release unit
- **THEN** validation fails before rendering or release cutting

### Requirement: One final release adapter owns every supported topology

The public configuration MUST expose only `openspec-release@1`. That adapter MUST support single, independent and composed topologies. Historical release manifest formats MUST remain parseable, but Arcantry MUST NOT expose another adapter version or a migration command solely for unpublished prepublication adapter contracts.

#### Scenario: A multi-unit repository configures releases

- **WHEN** it selects independent or composed topology under `openspec-release@1`
- **THEN** unit selectors, manifests, dependency pins and changelogs retain their defined behavior
- **AND** no second adapter identifier is required

## MODIFIED Requirements

### Requirement: SemVer impact belongs to the change

Every SemVer release-bearing change MUST declare `patch`, `minor` or `major` impact and MAY declare `unit_impacts` overrides for matched release units. `openspec-release@1` MUST reject `impact: none` for schema-declared release-bearing changes. A SemVer unit release version MUST be computed from the highest effective impact for that unit. Integer and calendar strategies MAY omit impact and MUST advance monotonically using their configured strategy. A change MAY acknowledge direct dependency adoption through `dependency_updates`, but dependency movement alone MUST NOT create or bump a parent release.

#### Scenario: A release contains mixed impacts

- **WHEN** a unit release plan contains more than one archived change
- **THEN** the next version is computed from the highest effective impact for that unit

#### Scenario: A release is not published

- **WHEN** completed changes are retained only in the repository
- **THEN** they MAY remain unassigned until a release is explicitly requested

#### Scenario: One outcome has different unit impacts

- **WHEN** a release-bearing change matches multiple units and declares a unit impact override
- **THEN** each unit plan uses its override or the global impact fallback

#### Scenario: A child unit releases independently

- **WHEN** a child unit releases a newer version without an eligible parent change acknowledging it
- **THEN** the parent has a pending dependency update
- **AND** a parent release remains blocked until one selected parent change lists that dependency in `dependency_updates`

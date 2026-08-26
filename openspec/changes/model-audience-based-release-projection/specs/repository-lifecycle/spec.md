## ADDED Requirements

### Requirement: Release classification separates audience, impact and inclusion

A release adapter that supports audience-based projection MUST classify each release-bearing change independently by SemVer impact, affected components, intended audiences, observable impact and changelog inclusion policy. Intended audiences and observable impact MUST NOT replace SemVer calculation or manifest assignment. The adapter MUST support customer outcomes, user-felt behavior, significant technical work and maintenance without requiring a custom changelog category for each distinction.

#### Scenario: Maintenance work has no audience-facing entry

- **WHEN** an accepted release-bearing change is classified for omission from the changelog
- **THEN** it remains assigned to its matching release manifest and contributes its effective SemVer impact
- **AND** deterministic rendering emits no audience-facing entry for that change

#### Scenario: Technical work is significant to one audience

- **WHEN** a change affects developer or operator decisions without changing an end-user task
- **THEN** classification records that audience and observable impact independently
- **AND** inclusion follows the configured changelog policy instead of a binary public-or-internal inference

### Requirement: Changelog projection supports explicit many-to-one stories

Several accepted release-bearing changes MAY form one changelog entry only through an explicit projection group. A group MUST have a stable id, compatible audience and category, one prose owner and a complete ordered membership list. Every member MUST resolve to one matching release assignment. Rendering MUST expose traceability from the entry to every member and MUST reject overlapping, missing, cyclic or differently classified ownership.

#### Scenario: Several outcomes form one release story

- **WHEN** accepted changes join one valid projection group
- **THEN** rendering produces one audience-facing entry from the designated prose owner
- **AND** the generated result remains traceable to every member change id

#### Scenario: One change enters competing stories

- **WHEN** a change is assigned to overlapping projection groups for the same release unit
- **THEN** validation fails before rendering or release cutting

### Requirement: Detached release history migrates through an explicit compatibility plan

Migration from a manual or detached release story MUST inspect and preserve current history before proposing managed state. The plan MUST report category mappings, managed baseline, source markers, consolidation groups, intentional omissions and unresolved entries. Apply MUST require unchanged input hashes and MUST NOT select a new adapter, rewrite history or reject deliberate differences until the reviewed plan defines their treatment.

#### Scenario: Existing history uses different categories

- **WHEN** inspection finds categories or consolidation rules that do not match the target adapter
- **THEN** the migration plan reports each difference with preserve, map, split, group or omit alternatives
- **AND** leaves the current history and configuration unchanged

#### Scenario: A migration plan is accepted

- **WHEN** every historical difference has one reviewed disposition and inputs remain unchanged
- **THEN** apply writes the target history and adapter configuration atomically
- **AND** validates source traceability before considering migration complete

### Requirement: Earlier release adapters remain supported

Repositories using `openspec-release@1` or `openspec-release@2` MUST retain their existing classification and rendering contracts until they explicitly migrate. A newer Arcantry version MUST NOT reinterpret their visibility or one-change-one-entry behavior through the audience-based adapter.

#### Scenario: An existing v2 repository is inspected

- **WHEN** no migration has been accepted
- **THEN** release planning and validation continue under the v2 contract
- **AND** audience-based fields are not inferred or written

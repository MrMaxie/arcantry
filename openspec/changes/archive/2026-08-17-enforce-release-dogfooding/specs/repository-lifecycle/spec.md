## MODIFIED Requirements

### Requirement: OpenSpec is the source of release history

Arcantry MUST derive release history from archived OpenSpec changes and release manifests. Every completed product or engineering change MUST resolve to an archived OpenSpec change before final repository validation succeeds, whether the intent was authored before or after implementation.

#### Scenario: Release history is generated

- **WHEN** changelog or component history is rendered
- **THEN** every entry resolves to an archived OpenSpec change grouped by a release manifest

#### Scenario: Intent is recovered after implementation

- **WHEN** implementation exists before its OpenSpec record
- **THEN** the repository cannot reach a valid completed state until a normal OpenSpec change describes, verifies and archives the delivered behavior

### Requirement: Archive is the delivery boundary

A completed change MUST NOT pass final repository validation or appear in release history until its OpenSpec change is archived and assigned to a release manifest.

#### Scenario: An active change exists

- **WHEN** release history is generated before the change is archived
- **THEN** the active change is absent from that history and final repository validation does not treat the state as complete

#### Scenario: An active or unassigned change exists

- **WHEN** final repository validation runs with an active change or an archived change that is not assigned to the newest release
- **THEN** validation fails instead of treating the repository state as complete

### Requirement: SemVer impact belongs to the change

Every completed product or engineering change MUST declare `patch`, `minor` or `major` impact in its release artifact and be assigned to a new SemVer release. The release version MUST be computed from the highest impact in the release. Internal completion MUST NOT depend on external publication.

#### Scenario: A release contains mixed impacts

- **WHEN** the release plan contains more than one archived change
- **THEN** the next version is computed from the highest declared impact

#### Scenario: A release is not published

- **WHEN** completed changes are retained only in the repository
- **THEN** they still produce a new SemVer manifest, aligned distribution versions and a changelog version section

### Requirement: Generated changelog is reproducible

The changelog MUST be generated from release manifests and archived change release artifacts in deterministic version order. Entries MUST describe behavior-level outcomes from OpenSpec and MUST NOT be derived from atomic commits.

#### Scenario: Changelog generation repeats

- **WHEN** generation runs twice against unchanged release inputs
- **THEN** both outputs are byte-for-byte identical

#### Scenario: Multiple commits implement one outcome

- **WHEN** a release change was implemented through one or more commits
- **THEN** the changelog contains the OpenSpec release outcome rather than commit-level entries

### Requirement: Release state is validated as a whole

Repository validation MUST fail when a manifest references a missing or active change, reuses a change id, breaks descending SemVer order, contains an invalid component id, leaves completed changes unassigned, has distribution version drift, has generated changelog drift or leaves repository changes after the newest release seal.

#### Scenario: Release metadata drifts

- **WHEN** a manifest, distribution version or generated changelog violates a release invariant
- **THEN** repository validation fails with the violated invariant

#### Scenario: Repository work follows the newest release seal

- **WHEN** strict release validation finds a commit after the commit introducing the newest release manifest
- **THEN** validation fails and requires the work to be represented by archived OpenSpec intent and a newer internal release

## ADDED Requirements

### Requirement: Git history is coverage evidence only

Release validation MAY use Git history to prove that the current repository state is sealed by the newest release manifest. It MUST NOT derive release prose, category, SemVer impact, visibility or components from commits or file diffs.

#### Scenario: The release seal is inspected

- **WHEN** validation reads Git history for the newest release manifest
- **THEN** it reports only whether later repository changes exist and resolves all release meaning from OpenSpec artifacts

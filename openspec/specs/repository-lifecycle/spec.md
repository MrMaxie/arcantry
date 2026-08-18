## Purpose

Define the repository-level contract that keeps intent, implementation and release history distinct while making delivery reproducible.

## Requirements

### Requirement: OpenSpec is the source of release history

When Arcantry manages a changelog, it MUST derive release meaning from configured OpenSpec sources and release manifests. Existing changelog history MAY remain external, preserved or frozen before an explicit cutover boundary. Git commits and file diffs MUST NOT supply release prose, category, version impact, visibility or components.

#### Scenario: A brownfield changelog starts a managed future

- **WHEN** a cutover plan defines a managed release boundary
- **THEN** history before the boundary remains byte-for-byte unchanged
- **AND** later entries are rendered from OpenSpec release artifacts

#### Scenario: Historical meaning is ambiguous

- **WHEN** full migration cannot map an existing entry to explicit semantic intent
- **THEN** migration reports a conflict instead of inferring meaning from Git

#### Scenario: Release history is generated

- **WHEN** a managed changelog or component history is rendered
- **THEN** every generated entry resolves to an archived OpenSpec change grouped by a release manifest

#### Scenario: Intent is recovered after implementation

- **WHEN** implementation exists before its OpenSpec record
- **THEN** the managed release state cannot become complete until a normal OpenSpec change describes, verifies and archives the delivered behavior

### Requirement: Archive is the delivery boundary

A completed change MUST NOT pass final repository validation or appear in release history until its OpenSpec change is archived and assigned to a release manifest.

#### Scenario: An active change exists

- **WHEN** release history is generated before the change is archived
- **THEN** the active change is absent from that history and final repository validation does not treat the state as complete

#### Scenario: An active or unassigned change exists

- **WHEN** final repository validation runs with an active change or an archived change that is not assigned to the newest release
- **THEN** validation fails instead of treating the repository state as complete

### Requirement: Release manifests only group changes

A release manifest MUST contain a version, release date and explicit list of archived change ids. It MUST NOT duplicate change prose or specification deltas.

#### Scenario: A release is cut

- **WHEN** a release manifest is created
- **THEN** it groups archived change ids without copying their prose or deltas

### Requirement: SemVer impact belongs to the change

Every completed product or engineering change MUST declare `patch`, `minor` or `major` impact in its release artifact and be assigned to a new SemVer release. The release version MUST be computed from the highest impact in the release. Internal completion MUST NOT depend on external publication.

#### Scenario: A release contains mixed impacts

- **WHEN** the release plan contains more than one archived change
- **THEN** the next version is computed from the highest declared impact

#### Scenario: A release is not published

- **WHEN** completed changes are retained only in the repository
- **THEN** they still produce a new SemVer manifest, aligned distribution versions and a changelog version section

### Requirement: Release components identify affected product surfaces

Every releasable change MUST list affected components in its release artifact using stable component ids such as `docs`, `schemas`, `tooling`, `cli`, `catalog`, `repository-adoption` or `skill:<name>`.

#### Scenario: Component history is requested

- **WHEN** history is generated for one product surface
- **THEN** entries are selected by stable component ids from archived release artifacts

### Requirement: Generated changelog is reproducible

New managed changelogs MUST follow Keep a Changelog 2.0 structure with a fixed preamble, an Unreleased section, the six standard change categories, ISO dates and optional comparison links when a repository URL is explicitly available. Rendering MUST be deterministic and MUST preserve configured legacy history.

#### Scenario: No repository URL is available

- **WHEN** changelog rendering has no explicit comparison URL
- **THEN** it omits comparison links instead of inventing a host or repository address

#### Scenario: Changelog generation repeats

- **WHEN** generation runs twice against unchanged release inputs
- **THEN** both managed outputs are byte-for-byte identical

#### Scenario: Multiple commits implement one outcome

- **WHEN** a release change was implemented through one or more commits
- **THEN** the changelog contains the OpenSpec release outcome rather than commit-level entries

### Requirement: Release state is validated as a whole

Repository validation MUST fail when a manifest references a missing or active change, reuses a change id, breaks descending SemVer order, contains an invalid component id, leaves completed changes unassigned, has distribution version drift, has generated changelog drift or leaves repository changes after the newest release seal. Validation of contributed release artifact content MUST use input-bounded parsing that does not exhibit polynomial regular-expression behavior.

#### Scenario: Release metadata drifts

- **WHEN** a manifest, distribution version or generated changelog violates a release invariant
- **THEN** repository validation fails with the violated invariant

#### Scenario: Repository work follows the newest release seal

- **WHEN** strict release validation finds a commit after the commit introducing the newest release manifest
- **THEN** validation fails and requires the work to be represented by archived OpenSpec intent and a newer internal release

#### Scenario: A malformed release title contains excessive whitespace

- **WHEN** validation reads a release artifact whose title line contains no title after a long whitespace sequence
- **THEN** validation rejects the title in work proportional to the artifact size

### Requirement: Git history is coverage evidence only

Release validation MAY use Git history to prove a configured release seal when Git is available and sealing is enabled. Projects without Git MUST remain able to inspect, plan, apply and validate non-seal source contracts.

#### Scenario: A non-Git project manages todo and OpenSpec

- **WHEN** repository validation runs without a Git worktree
- **THEN** source validation succeeds or fails solely from configured source contracts
- **AND** no Git seal requirement is inferred

#### Scenario: The release seal is inspected

- **WHEN** configured validation reads Git history for the newest release manifest
- **THEN** it reports only whether later repository changes exist
- **AND** resolves all release meaning from OpenSpec artifacts

### Requirement: Repository commands are stable

The repository MUST expose stable commands for checking, building, serving, validating changes, planning releases, cutting releases and rendering the changelog.

#### Scenario: A contributor inspects repository commands

- **WHEN** they use the documented command interface
- **THEN** check, build, serve, validation, release planning, release cutting, and changelog rendering remain available

### Requirement: External publication consumes sealed release state

An external package publication MUST consume a repository state already sealed by the newest release manifest. Its trigger and registry metadata MUST NOT define release prose, category, SemVer impact, visibility or components.

#### Scenario: A release tag matches the seal

- **WHEN** npm publication runs for `v<version>`
- **THEN** the tag version, newest release manifest, distribution version and release-seal commit all match before registry mutation

#### Scenario: Publication inputs disagree

- **WHEN** the tag, manifest, package version or checked-out commit does not identify the same sealed release
- **THEN** publication fails without changing registry state

#### Scenario: A version already exists

- **WHEN** the target package version is already public in npm
- **THEN** publication fails as a duplicate instead of overwriting or reusing that version

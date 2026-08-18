## MODIFIED Requirements

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

## ADDED Requirements

### Requirement: Public skills have independent identity and role

Every public skill MUST declare its own semantic version and one role from `primary`, `supporting` or `advanced`. The version MUST remain independent from the Arcantry product and CLI version. Generated catalog projections MUST expose both fields without changing skill invocation policy. Every projected package MUST include a deterministic SHA-256 manifest and a bounded history derived from accepted OpenSpec outcomes.

#### Scenario: A skill changes while Arcantry remains at the same version

- **WHEN** an accepted skill revision changes package content without an Arcantry release
- **THEN** its exact revision and content digest change independently
- **AND** the Arcantry product and CLI remain at version `1.0.0`

### Requirement: One skill can be compared and updated safely

The native CLI MUST report local management state, version, exact revision, content digest and local modification state for a selected installed skill. Remote checking MUST be explicit and MUST compare against the official Arcantry GitHub `master` unless an explicit local catalog is selected. Updating MUST first produce a serializable plan pinned to one exact source revision, package digest and installed preimage. Applying that plan MUST validate the complete package and unchanged preimage before atomically switching only the selected skill and recording its provenance outside skill discovery roots.

#### Scenario: A newer canonical revision is available

- **WHEN** the user checks one managed skill and its official package digest differs
- **THEN** status reports the exact installed and available identities without writing
- **AND** update planning describes the exact package and target changes

#### Scenario: Installed content changed after preview

- **WHEN** the installed package, managed target or source package no longer matches the saved plan
- **THEN** apply fails before replacing any link or receipt
- **AND** the previous installation remains usable

#### Scenario: Other skills share the same CLI installation

- **WHEN** one selected skill update is applied
- **THEN** its immutable snapshot and managed links change
- **AND** snapshots, receipts and links for every other skill remain unchanged

### Requirement: Legacy and private packages are not silently adopted

Legacy embedded links, source-checkout links and ordinary skill directories without an ownership receipt MUST remain readable and inspectable. The CLI MUST identify them as development or unmanaged and MUST NOT overwrite, migrate or claim them automatically. Private repository skills MUST never be matched to or uploaded to the official public source.

#### Scenario: A user checks an unowned or modified package

- **WHEN** no receipt proves ownership or installed bytes differ from its receipt
- **THEN** status identifies the state and update apply refuses an implicit replacement
- **AND** the existing files and links remain unchanged

## MODIFIED Requirements

### Requirement: Public catalog is an audience-facing projection

The generated public catalog MUST group every skill into exactly one supported family, including `self-improvement`, `repo-safely`, `content-safely`, and `code-quality`. Each catalog item MUST use a readable display name, a distinct short outcome, independent skill version, role, and a link to its detail page. Raw tags MAY support search and filtering but MUST NOT occupy a primary catalog column or card region. Generated navigation MUST include every public skill exactly once.

#### Scenario: A developer scans the catalog

- **WHEN** they compare skills in one family
- **THEN** names remain readable, summaries remain distinct and short, and version and role remain secondary to the choice
- **AND** every skill appears in exactly one family and once in generated navigation

## REMOVED Requirements

### Requirement: Skill versions follow the Arcantry release

**Reason:** Skill versions and accepted history are now independent from the Arcantry product and CLI release.

**Migration:** Use each package's `version`, exact digest or revision, role, and accepted component history.

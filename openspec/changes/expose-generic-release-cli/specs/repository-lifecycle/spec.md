## ADDED Requirements

### Requirement: Adopted projects configure release sources explicitly

An adopted project MAY configure the OpenSpec release adapter, release manifest directory, managed changelog source, repository URL, tag prefix and version sources. Each version source MUST name a supported adapter and path. Arcantry MUST NOT infer or update an unconfigured version source.

#### Scenario: A Cargo workspace is configured

- **WHEN** a project configures `cargo-workspace@1` for `Cargo.toml`
- **THEN** release validation reads only `[workspace.package] version`
- **AND** release cutting updates only that version entry

#### Scenario: Release configuration is absent

- **WHEN** a release command runs without a release configuration
- **THEN** it fails without changing project files

### Requirement: Brownfield baselines preserve unknown history

A baseline release manifest MUST identify an existing SemVer version and ISO date, MUST declare `baseline: true`, and MAY contain no changes. A baseline MUST NOT invent release prose or make historical internal change artifacts public.

#### Scenario: An existing release becomes the baseline

- **WHEN** baseline planning finds aligned configured version sources and no manifest for the requested version
- **THEN** it plans a baseline manifest and deterministic changelog boundary

#### Scenario: A later release is cut

- **WHEN** unassigned archived changes exist after the baseline
- **THEN** the next version is computed from their highest declared impact
- **AND** the new manifest is not marked as a baseline

### Requirement: Public changelog excludes internal changes

Release manifests MUST retain every assigned archived change, including internal changes, while public changelog rendering MUST omit entries whose release visibility is `internal`.

#### Scenario: A release contains only internal changes

- **WHEN** the changelog is rendered for a manifest whose assigned changes are all internal
- **THEN** the version remains part of release state
- **AND** no internal release title or body appears in the public changelog

### Requirement: Release checking has consistency and seal modes

Normal release checking MUST validate artifacts, assignments, configured version sources and generated changelog consistency while allowing active or unassigned changes. Sealed release checking MUST additionally require no active or unassigned changes and enforce the configured Git release seal.

#### Scenario: Work remains after a valid baseline

- **WHEN** normal release checking finds active or unassigned changes but all persisted release artifacts are consistent
- **THEN** it reports success without treating the repository as release-sealed

#### Scenario: Final sealing is requested

- **WHEN** sealed checking finds active or unassigned changes
- **THEN** it fails without mutating the repository

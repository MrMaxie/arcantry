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

### Requirement: Git history is coverage evidence only

Release validation MAY use Git history to prove a configured release seal when Git is available and sealing is enabled. Projects without Git MUST remain able to inspect, plan, apply and validate non-seal source contracts. Pull request CI MAY validate an explicitly supplied submitted head commit when the checked-out state is a synthetic merge commit, but MUST require that commit to introduce the newest release manifest and be a direct parent of the checked-out commit. External publication MUST remain bound to the actual checked-out commit.

#### Scenario: A non-Git project manages todo and OpenSpec

- **WHEN** repository validation runs without a Git worktree
- **THEN** source validation succeeds or fails solely from configured source contracts
- **AND** no Git seal requirement is inferred

#### Scenario: The release seal is inspected

- **WHEN** configured validation reads Git history for the newest release manifest
- **THEN** it reports only whether later repository changes exist
- **AND** resolves all release meaning from OpenSpec artifacts

#### Scenario: Pull request CI checks a synthetic merge commit

- **WHEN** CI supplies the exact submitted head commit while testing a synthetic merge result
- **THEN** release validation accepts the seal only when that submitted commit introduced the newest release manifest
- **AND** the submitted commit is a direct parent of the checked-out merge commit
- **AND** publication validation continues to require the actual checked-out commit

### Requirement: Repository commands are stable

The repository MUST keep a root `justfile` as the only task runner and expose stable recipes for checking, building, serving documentation, validating changes, planning releases, cutting releases and rendering the changelog. mise MUST pin and provision `just` and Nub. The recipes MUST invoke package management and the underlying repository tools directly through Nub without routing through root package scripts. Dependency installation, Node provisioning and repository tool execution MUST NOT require pnpm.

#### Scenario: A contributor inspects repository commands

- **WHEN** they install the pinned tools with mise and list or use the documented `just` recipes
- **THEN** check, build, serve, validation, release planning, release cutting and changelog rendering remain available

#### Scenario: CI starts from a clean checkout

- **WHEN** a GitHub-hosted runner checks out the repository
- **THEN** mise provisions the pinned `just` and Nub versions
- **AND** `just ci-setup` uses Nub to provision the repository's Node version, expose it to later workflow steps and install the frozen workspace lockfile
- **AND** CI runs the same `just ci` quality gate used by contributors

### Requirement: External publication consumes sealed release state

Every external npm or GitHub Release publication MUST consume a repository state already sealed by the newest release manifest. The Git tag, native executable versions, native archive names and checksums, JavaScript package version, platform package versions and release-seal commit MUST identify the same release. Publication triggers and registry or release metadata MUST NOT define release prose, category, SemVer impact, visibility or components.

#### Scenario: A release tag matches the seal

- **WHEN** npm and native artifact publication run for `v<version>`
- **THEN** the tag version, newest release manifest, every distribution version and release-seal commit all match before external mutation

#### Scenario: Publication inputs disagree

- **WHEN** the tag, manifest, package version, native archive version, checksum set or checked-out commit does not identify the same sealed release
- **THEN** all unpublished outputs remain private and publication fails without replacing an existing artifact

#### Scenario: A version already exists

- **WHEN** the target `arcantry` package version or public GitHub Release already exists
- **THEN** publication fails as a duplicate instead of overwriting or reusing that version

#### Scenario: A platform package exists during a safe retry

- **WHEN** a platform package version was published before an interrupted main-package publication
- **THEN** the retry accepts it only when its immutable registry integrity matches the retained verified archive from the same seal
- **AND** otherwise fails without publishing the main package

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

### Requirement: Release units own their changelog scope

Independent topologies MUST render only per-unit changelogs and MUST NOT synthesize a root changelog. In composed topologies, a parent changelog MAY serve as the product summary but MUST include only outcomes selected for that parent and MUST NOT copy child entries automatically.

#### Scenario: A child change has no parent outcome

- **WHEN** a child release is rendered and no parent change selects or acknowledges it
- **THEN** the child entry appears only in the child changelog
- **AND** the parent changelog remains unchanged

### Requirement: Normal and sealed release checks remain distinct

Normal release checking MUST validate persisted release consistency while allowing active or unassigned work. Sealed checking MUST additionally require complete scoped assignment, a clean Git state and the configured release seal. The two modes MUST have separate executable native scenarios.

#### Scenario: Active work exists during normal checking

- **WHEN** persisted release artifacts are consistent and active or unassigned work exists
- **THEN** normal release checking succeeds
- **AND** sealed checking fails

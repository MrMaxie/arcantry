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

### Requirement: Implementation and release have separate boundaries

Ordinary implementation, validation, commits and deployment MUST NOT require a release manifest. Release preparation MUST use archived changes. Release sealing and publication require explicit authorization and MAY impose stricter completeness checks.

#### Scenario: Work continues without a release

- **WHEN** active or unassigned work exists
- **THEN** ordinary host validation remains available without cutting a release
- **AND** unarchived work stays out of rendered release history

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

### Requirement: Release components identify affected product surfaces

Every releasable change MUST list affected components in its release artifact using stable component ids such as `docs`, `schemas`, `tooling`, `cli`, `catalog`, `repository-adoption` or `skill:<name>`.

#### Scenario: Component history is requested

- **WHEN** history is generated for one product surface
- **THEN** entries are selected by stable component ids from archived release artifacts

### Requirement: Generated changelog is reproducible

New managed changelogs MUST use either the Keep a Changelog preset or an explicit project MiniJinja template. The preset provides its preamble, Unreleased section, standard categories, ISO dates and optional comparison links when a repository URL is explicitly available. Rendering MUST be deterministic and MUST preserve configured legacy history.

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

The repository MUST keep a root `justfile` as the only task runner and expose stable recipes for checking, building, serving documentation, validating changes, planning releases, cutting releases, rendering the changelog and verifying publication. mise MUST pin and provision `just`, Nub, Rust and distribution tools. The recipes MUST invoke package management and the underlying repository tools directly through Nub or Cargo without routing through root package scripts. Dependency installation, Node provisioning and repository tool execution MUST NOT require pnpm.

#### Scenario: A contributor inspects repository commands

- **WHEN** they install the pinned tools with mise and list or use the documented `just` recipes
- **THEN** local checking, build, documentation, validation, release preparation and publication verification remain available

#### Scenario: CI starts from a clean checkout

- **WHEN** a GitHub-hosted runner checks out the repository
- **THEN** mise provisions the pinned tools and `just ci-setup` installs the frozen workspace
- **AND** pull-request and `master` CI run `just ci`, Pages runs `just docs-build`, and sealed tags run the release workflow

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

A baseline release manifest MUST identify an existing version valid for the selected strategy and ISO date, MUST declare `baseline: true`, and MAY contain no changes. A baseline MUST NOT invent release prose or make historical internal change artifacts public.

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

### Requirement: Release artifacts support multiple consumer outcomes

A release-bearing OpenSpec change MAY declare one or more consumer outcomes. Each outcome MUST have exactly one standard changelog category, a non-empty title and non-empty consumer prose. SemVer impact, visibility, components, unit impacts and dependency updates MUST remain owned by the change as a whole. Release manifests MUST continue to assign change ids rather than individual outcomes.

The canonical multi-outcome `release.md` body MUST use unique level-two standard category headings containing one or more level-three outcome headings. Content before the first category, prose without an outcome heading, duplicate category sections, unknown categories, empty titles and empty bodies MUST be rejected. A legacy artifact with `category` frontmatter, one level-one title and one body MUST remain valid and MUST NOT be rewritten merely because the canonical format exists. One artifact MUST NOT mix the legacy and canonical structures.

#### Scenario: One change has added and fixed outcomes

- **WHEN** one valid release artifact declares distinct outcomes under `Added` and `Fixed`
- **THEN** release planning uses the change's single effective SemVer impact
- **AND** changelog rendering emits each outcome separately under its declared category
- **AND** every emitted outcome remains traceable to the same archived change id

#### Scenario: An existing single-outcome artifact is read

- **WHEN** a legacy release artifact declares `category` frontmatter, one level-one title and consumer prose
- **THEN** parsing and rendering preserve its existing release meaning
- **AND** validation does not require migration to the canonical format

#### Scenario: A release artifact mixes structures

- **WHEN** a release artifact declares legacy `category` frontmatter and canonical category sections
- **THEN** validation rejects the artifact instead of selecting one interpretation

#### Scenario: Change-wide metadata applies to several outcomes

- **WHEN** a multi-outcome artifact matches a release unit
- **THEN** every outcome shares the change's visibility and component ownership
- **AND** the manifest assigns the change exactly once

### Requirement: Product and repository tooling have one Rust implementation authority

All Arcantry product logic and repository-maintained automation outside `apps/docs` MUST be implemented in Rust. This includes project discovery, configuration, source handling, todo operations, skill and catalog processing, release planning and validation, generated artifact production, package assembly, smoke orchestration and publication preparation. TypeScript MAY remain inside `apps/docs` for the Astro documentation application. A package-manager launcher MAY use minimal JavaScript only to locate and execute the native binary and MUST NOT implement Arcantry policy, parsing, validation, planning, generation or mutation behavior.

#### Scenario: A contributor runs repository automation

- **WHEN** a contributor invokes an Arcantry generation, validation, packaging, smoke or release-preparation task
- **THEN** the repository entrypoint dispatches to Rust-owned behavior
- **AND** any invoked Node.js ecosystem tool remains an external boundary rather than an Arcantry implementation layer

#### Scenario: TypeScript is added outside documentation

- **WHEN** repository validation inventories an authored TypeScript file outside `apps/docs`
- **THEN** validation fails unless the file is still present in the reviewed shrinking migration inventory
- **AND** the final accepted inventory permits no TypeScript outside `apps/docs`

#### Scenario: The documentation application is built

- **WHEN** contributors build or validate `apps/docs`
- **THEN** its Astro and TypeScript implementation remains supported
- **AND** it consumes generated product data without becoming the authority for Arcantry behavior

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

### Requirement: An unsealed first public release can be finalized in place

The latest internal release manifest MAY be finalized without a version change only when no matching Git tag, npm package or public GitHub Release exists. Finalization MUST retain the selected version, assign every accepted release-bearing change, use an explicit release date and regenerate the managed changelog from those OpenSpec outcomes. A tagged or externally published release MUST remain immutable.

#### Scenario: The internal candidate has never been published

- **WHEN** `1.0.0` exists only as an untagged internal manifest and the maintainer authorizes it as the first public release
- **THEN** the manifest may add every later accepted release-bearing change and record the authorized release date
- **AND** all product and distributable version sources remain `1.0.0`

#### Scenario: The release already has an external identity

- **WHEN** a matching tag, npm version or public GitHub Release exists
- **THEN** finalization refuses to change that manifest or its release date

### Requirement: Protected automation keeps incomplete releases private

The release workflow MUST create or refresh a draft GitHub Release only after repository, native artifact and installer verification pass. npm and final GitHub publication MUST run behind the tag-scoped `npm` environment. The GitHub Release MUST remain a draft when package preflight, trusted publishing, environment approval or npm publication fails.

#### Scenario: The first npm package set needs bootstrap

- **WHEN** verified `1.0.0` archives exist but their package names do not yet exist on npm
- **THEN** the workflow retains those archives and the draft GitHub Release while protected publication waits
- **AND** no reusable npm write credential is introduced

#### Scenario: The complete package set is verified

- **WHEN** every exact package archive is published or confirmed with matching registry integrity
- **THEN** the workflow may make the matching GitHub Release public

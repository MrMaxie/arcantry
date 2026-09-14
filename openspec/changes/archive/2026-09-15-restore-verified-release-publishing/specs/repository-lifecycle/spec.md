## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: Repository commands are stable

The repository MUST keep a root `justfile` as the only task runner and expose stable recipes for checking, building, serving documentation, validating changes, planning releases, cutting releases, rendering the changelog and verifying publication. mise MUST pin and provision `just`, Nub, Rust and distribution tools. The recipes MUST invoke package management and the underlying repository tools directly through Nub or Cargo without routing through root package scripts. Dependency installation, Node provisioning and repository tool execution MUST NOT require pnpm.

#### Scenario: A contributor inspects repository commands

- **WHEN** they install the pinned tools with mise and list or use the documented `just` recipes
- **THEN** local checking, build, documentation, validation, release preparation and publication verification remain available

#### Scenario: CI starts from a clean checkout

- **WHEN** a GitHub-hosted runner checks out the repository
- **THEN** mise provisions the pinned tools and `just ci-setup` installs the frozen workspace
- **AND** pull-request and `master` CI run `just ci`, Pages runs `just docs-build`, and sealed tags run the release workflow

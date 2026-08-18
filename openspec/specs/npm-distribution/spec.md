# npm-distribution Specification

## Purpose

Define the public npm package identity and the verified, release-sealed publication boundary for Arcantry packages.

## Requirements

### Requirement: Publication uses the verified package archive

The repository MUST build one package archive for a release candidate, validate its file allowlist, install it in an isolated smoke environment and publish that same archive. The archive MUST exclude source-only, private, local and repository-management state.

#### Scenario: Package verification succeeds

- **WHEN** the release job prepares an npm archive
- **THEN** CLI version, commands and declared exports are exercised from an isolated installation of that archive
- **AND** the verified archive is retained as the only publication input

#### Scenario: Package contents drift

- **WHEN** the archive contains an unexpected or forbidden path
- **THEN** publication fails before npm authentication or registry mutation

### Requirement: Automated npm publication uses trusted publishing

After the package bootstrap, automated publication MUST use npm trusted publishing from the repository's named GitHub Actions workflow on a GitHub-hosted runner. The job MUST use a protected `npm` environment, request only `contents: read` and `id-token: write`, use an npm CLI version compatible with trusted publishing, and MUST NOT require a long-lived npm write token.

#### Scenario: A valid release tag is published

- **WHEN** a protected tag names a sealed, unpublished release and all verification passes
- **THEN** the workflow publishes the verified archive as a public package through OIDC

#### Scenario: The publication identity does not match

- **WHEN** the repository, workflow filename, environment or package repository metadata differs from the npm trusted-publisher configuration
- **THEN** publication fails without falling back to a stored write token

### Requirement: The first package publication is an explicit bootstrap

Because trusted publishing configuration requires an existing npm package, the first public `arcantry` version MUST be published by an authorized maintainer from the exact verified archive of a sealed release using 2FA. The package MAY then be assigned to the Arcantry npm organization. The trusted publisher MUST be configured before any later version is published by CI/CD.

#### Scenario: The package does not exist in npm

- **WHEN** the first sealed archive is ready and separately authorized for publication
- **THEN** a maintainer publishes that archive once with 2FA
- **AND** assigns organization governance when required
- **AND** configures trusted publishing for later releases

#### Scenario: Bootstrap has not been completed

- **WHEN** an automated publication is attempted before the package and trusted-publisher relationship exist
- **THEN** the workflow fails without creating or exposing a reusable npm write credential

### Requirement: The public npm package uses the global Arcantry name

The existing combined CLI and library package MUST use the unscoped public name `arcantry`, expose the `arcantry` binary and retain its declared public subpath exports. The private workspace root MUST use a distinct non-publishable name so package-manager filters resolve the public package unambiguously. The package MAY be governed by the Arcantry npm organization without using its scope in the consumer-facing name.

#### Scenario: A consumer runs the CLI without installing it

- **WHEN** the consumer invokes `npx arcantry`, `pnpm dlx arcantry` or an equivalent package launcher
- **THEN** npm resolves the public package and runs the `arcantry` binary

#### Scenario: A consumer imports a public module

- **WHEN** the consumer imports a declared `arcantry` subpath from the packed package
- **THEN** the import resolves to its shipped JavaScript and type declarations

#### Scenario: Repository package references are validated

- **WHEN** package, workspace, test and public documentation surfaces are checked
- **THEN** they resolve the canonical `arcantry` package identity
- **AND** no shipped or documented command refers to the previous scoped identities

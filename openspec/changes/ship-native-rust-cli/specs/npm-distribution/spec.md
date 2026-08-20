## MODIFIED Requirements

### Requirement: Publication uses the verified package archive

The repository MUST build one package archive for the public `arcantry` package and one package archive for each supported native target, validate every archive against its package-specific file allowlist, install the complete package set in isolated smoke environments and publish those same archives. Every archive MUST exclude source-only, private, local and repository-management state.

#### Scenario: Package verification succeeds

- **WHEN** the release job prepares the npm package set
- **THEN** CLI version, package dispatch, commands and declared JavaScript exports are exercised from isolated installations of the exact archives
- **AND** the verified archives are retained as the only publication inputs

#### Scenario: Package contents drift

- **WHEN** any archive contains an unexpected or forbidden path
- **THEN** publication fails before npm authentication or registry mutation

### Requirement: Automated npm publication uses trusted publishing

After the package-name bootstrap, automated publication of `arcantry` and its scoped platform packages MUST use npm trusted publishing from the repository's named GitHub Actions workflow on GitHub-hosted runners. The job MUST use a protected `npm` environment, request only the permissions required to read the sealed repository state and obtain an OIDC identity, use an npm CLI version compatible with trusted publishing, and MUST NOT require a long-lived npm write token.

#### Scenario: A valid release tag is published

- **WHEN** a protected tag names a sealed, unpublished release and all native and npm verification passes
- **THEN** the workflow publishes the verified platform archives and then the verified `arcantry` archive through OIDC

#### Scenario: The publication identity does not match

- **WHEN** the repository, workflow filename, environment or package repository metadata differs from the applicable npm trusted-publisher configuration
- **THEN** publication fails without falling back to a stored write token

### Requirement: The first package publication is an explicit bootstrap

Because trusted publishing configuration requires each npm package to exist, the first public version of `arcantry` and each scoped platform package MUST be published by an authorized maintainer from the exact verified archives of one sealed release using 2FA. The packages MAY then be assigned to the Arcantry npm organization. Every trusted publisher MUST be configured before any later version is published by CI/CD.

#### Scenario: The package does not exist in npm

- **WHEN** the first sealed package set is ready and separately authorized for publication
- **THEN** a maintainer publishes the exact missing archives once with 2FA
- **AND** assigns organization governance when required
- **AND** configures trusted publishing for later releases

#### Scenario: Bootstrap has not been completed

- **WHEN** automated publication is attempted before every package and trusted-publisher relationship exists
- **THEN** the workflow fails without creating or exposing a reusable npm write credential

### Requirement: The public npm package uses the global Arcantry name

The public package MUST retain the unscoped name `arcantry`, expose the `arcantry` launcher and retain its declared public JavaScript subpath exports and type declarations. The launcher MUST dispatch to an exact-version optional platform package selected from the host operating system and architecture. Linux platform packages MUST contain statically linked musl binaries, omit the npm `libc` field and be smoke-tested on both Ubuntu glibc and Alpine musl. The private workspace root MUST use a distinct non-publishable name so package-manager filters resolve the public package unambiguously.

#### Scenario: A consumer runs the CLI without installing it

- **WHEN** the consumer invokes the CLI through npm/npx, pnpm, Bun or Nub on a supported platform
- **THEN** npm resolves the public package and matching optional platform package
- **AND** the launcher executes the native `arcantry` binary

#### Scenario: A consumer imports a public module

- **WHEN** the consumer imports a declared `arcantry` subpath from the packed package
- **THEN** the import resolves to its existing JavaScript and type declarations without invoking the native executable

#### Scenario: Optional dependencies are unavailable

- **WHEN** the matching platform package was omitted or cannot be resolved
- **THEN** the launcher reports the unsupported or incomplete installation and an actionable reinstall or GitHub Release path
- **AND** does not download or execute code from the network

#### Scenario: Repository package references are validated

- **WHEN** package, workspace, test and public documentation surfaces are checked
- **THEN** they resolve the canonical `arcantry` package identity and declared platform package set
- **AND** every shipped or documented command uses the canonical public identity

## ADDED Requirements

### Requirement: Platform packages are exact and complete before launcher publication

The `arcantry` package MUST declare exact-version optional dependencies on `@arcantry/cli-win32-x64`, `@arcantry/cli-win32-arm64`, `@arcantry/cli-darwin-x64`, `@arcantry/cli-darwin-arm64`, `@arcantry/cli-linux-x64` and `@arcantry/cli-linux-arm64`. Each platform package MUST declare matching npm `os` and `cpu` constraints and contain only its native executable and allowlisted package metadata. Linux packages MUST omit npm's `libc` constraint. The main package MUST NOT be published until all six exact platform versions are available and match their verified archives.

#### Scenario: A complete platform set is ready

- **WHEN** every platform archive has passed allowlist, integrity, version and target smoke checks
- **THEN** the platform packages are published or verified at their exact release version before `arcantry`
- **AND** the main package becomes visible only after the complete set is resolvable

#### Scenario: Publication resumes after a partial platform upload

- **WHEN** a platform version already exists during a retry
- **THEN** it is accepted only when its registry metadata and integrity match the retained verified archive
- **AND** a mismatch fails publication instead of overwriting or reusing the version

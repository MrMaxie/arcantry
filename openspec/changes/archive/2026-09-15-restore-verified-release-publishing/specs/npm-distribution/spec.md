## MODIFIED Requirements

### Requirement: The first package publication is an explicit bootstrap

Because trusted publishing configuration requires each npm package to exist, the first public version of `arcantry` and each scoped platform package MUST be published by an authorized maintainer from the exact retained archives of one sealed release using 2FA. Platform packages MUST be published before the main package. The packages MAY then be assigned to the Arcantry npm organization. Every package MUST configure GitHub Actions trusted publishing for repository `MrMaxie/arcantry`, workflow `release.yml`, environment `npm` and publish permission before the protected release job is approved.

#### Scenario: The package does not exist in npm

- **WHEN** the first sealed package set is ready and separately authorized for publication
- **THEN** a maintainer publishes the exact missing archives once with 2FA
- **AND** configures and verifies the matching trusted publisher before approving protected automation

#### Scenario: Bootstrap has not been completed

- **WHEN** protected publication inspects a missing package or missing trusted-publisher relationship
- **THEN** the GitHub Release remains a draft and no reusable npm write credential is created

### Requirement: Platform packages are exact and complete before launcher publication

The `arcantry` package MUST declare exact-version optional dependencies on `@arcantry/cli-win32-x64`, `@arcantry/cli-win32-arm64`, `@arcantry/cli-darwin-x64`, `@arcantry/cli-darwin-arm64`, `@arcantry/cli-linux-x64` and `@arcantry/cli-linux-arm64`. Each platform package MUST declare matching npm `os` and `cpu` constraints and contain only its native executable and allowlisted package metadata. Linux packages MUST omit npm's `libc` constraint. The main package MUST NOT be published until all six exact platform versions are available and match their verified archives. Retry preflight MUST accept any existing platform or main package only when its registry integrity matches the retained archive exactly.

#### Scenario: A complete platform set is ready

- **WHEN** every platform archive has passed allowlist, integrity, version and target smoke checks
- **THEN** the platform packages are published or verified at their exact release version before `arcantry`
- **AND** the main package becomes visible only after the complete set is resolvable

#### Scenario: Publication resumes after a partial or complete upload

- **WHEN** a platform or main package version already exists during a retry
- **THEN** it is skipped only when its registry integrity matches the retained verified archive
- **AND** any mismatch fails publication before the GitHub Release becomes public

#### Scenario: Publication resumes after a partial platform upload

- **WHEN** a platform version already exists during a retry
- **THEN** it is accepted only when its registry metadata and integrity match the retained verified archive
- **AND** a mismatch fails publication instead of overwriting or reusing the version

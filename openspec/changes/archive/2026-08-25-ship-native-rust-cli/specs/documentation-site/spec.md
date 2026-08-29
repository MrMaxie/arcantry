## ADDED Requirements

### Requirement: Installation guidance distinguishes native and npm use

The documentation MUST present direct archives, checksum-verifying sh and PowerShell installers, and the `arcantry` npm package as supported installation paths. It MUST list the supported operating-system and architecture matrix, explain that the Linux archives support both glibc and musl systems, and retain npm/npx, pnpm, and Nub package-runner guidance. It MUST describe installation from the user's task without exposing launcher, optional-package, migration-oracle or build-pipeline details. It MUST NOT document Homebrew, Scoop, unsupported targets, signing or automatic updates as delivered behavior.

#### Scenario: A user chooses an installation path

- **WHEN** the reader opens the CLI installation guidance
- **THEN** they can select the archive matching Windows, macOS or Linux on x64 or ARM64, or use the matching sh or PowerShell installer
- **AND** can run the same `arcantry` command after installation

#### Scenario: A user verifies a native download

- **WHEN** the reader installs from a GitHub Release
- **THEN** the documentation identifies the matching archive and `SHA256SUMS` verification path
- **AND** the provided sh and PowerShell installers verify the selected archive against that checksum manifest
- **AND** does not imply that an unsigned or unsupported distribution channel is available

## MODIFIED Requirements

### Requirement: Public package commands use the canonical npm identity

Documentation and interactive copy surfaces MUST derive or validate package launcher commands against the canonical `arcantry` package manifest name. Native download guidance MUST derive or validate target names against the declared release matrix rather than presenting platform package names as end-user commands.

#### Scenario: The npm package identity changes

- **WHEN** documentation generation and checks run
- **THEN** npm, pnpm and supported launcher examples use `arcantry`
- **AND** native archive examples use only declared release targets
- **AND** stale package scopes or target names fail validation instead of remaining in public copy

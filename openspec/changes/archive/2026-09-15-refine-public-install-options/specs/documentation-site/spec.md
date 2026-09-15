# Requirements

## MODIFIED Requirements

### Requirement: Public package commands use the canonical npm identity

Documentation and interactive copy surfaces MUST derive or validate the `npx` and `npm` commands against the canonical `arcantry` package manifest name. Native download guidance MUST derive or validate target names and the versioned GitHub Release URL against the declared release identity rather than presenting platform package names as end-user commands.

#### Scenario: The npm package identity changes

- **WHEN** documentation generation and checks run
- **THEN** `npx` and `npm` examples use `arcantry`
- **AND** native archive examples use only declared release targets
- **AND** stale package scopes or target names fail validation instead of remaining in public copy

### Requirement: Installation guidance distinguishes native and npm use

The documentation MUST present direct GitHub Release archives, checksum-verifying sh and PowerShell installers, and the `arcantry` npm package as the supported public distribution paths. The public installation picker MUST present `npx`, `npm`, PowerShell and `sh` in that order, followed by a `Download` link to the versioned GitHub Release. It MUST NOT present pnpm or Nub as installation-picker choices or accompanying public installation guidance. The documentation MUST list the supported operating-system and architecture matrix, explain that the Linux archives support both glibc and musl systems, and describe installation from the user's task without exposing launcher, optional-package, migration-oracle or build-pipeline details. It MUST NOT present unsupported distribution channels, targets, signing or automatic updates as delivered behavior.

#### Scenario: A user chooses an installation path

- **WHEN** the reader opens the CLI installation guidance
- **THEN** the choices appear as `npx`, `npm`, PowerShell, `sh` and `Download`
- **AND** `Download` is a link to the versioned GitHub Release rather than a copyable command tab
- **AND** pnpm and Nub are absent from the installation choices and accompanying installation copy
- **AND** they can select the archive matching Windows, macOS or Linux on x64 or ARM64
- **AND** they can run the same `arcantry` command after installation

#### Scenario: A user verifies a native download

- **WHEN** the reader installs from a GitHub Release
- **THEN** the documentation identifies the matching archive and `SHA256SUMS` verification path
- **AND** the provided sh and PowerShell installers verify the selected archive against that checksum manifest
- **AND** does not imply that an unsigned or unsupported distribution channel is available

# Requirements

## MODIFIED Requirements

### Requirement: Installation guidance distinguishes native and npm use

The documentation MUST present direct GitHub Release archives, checksum-verifying sh and PowerShell installers, and the `arcantry` npm package as the supported public distribution paths. The public installation picker MUST present `npx`, `npm`, PowerShell and `sh` in that order, followed by a `Download` link to the versioned GitHub Release. The `Download` link MUST remain outside the command tablist, align to the far right of the picker header at supported wide viewports, and use a divider plus a button-like treatment to distinguish navigation from command selection. It MUST NOT present pnpm or Nub as installation-picker choices or accompanying public installation guidance. The documentation MUST identify Windows x64, macOS x64 and ARM64, and Linux x64 as the supported native matrix, explain that the Linux archive supports both glibc and musl systems, and describe installation from the user's task without exposing launcher, optional-package, migration-oracle or build-pipeline details. It MUST NOT present unsupported distribution channels, targets, signing or automatic updates as delivered behavior.

#### Scenario: A user chooses an installation path

- **WHEN** the reader opens the CLI installation guidance
- **THEN** the choices appear as `npx`, `npm`, PowerShell, `sh` and `Download`
- **AND** `Download` is a right-aligned link to the versioned GitHub Release rather than a copyable command tab
- **AND** a divider and button-like treatment distinguish `Download` from the command tabs
- **AND** pnpm and Nub are absent from the installation choices and accompanying installation copy
- **AND** they can select the archive matching Windows x64, macOS x64 or ARM64, or Linux x64
- **AND** they can run the same `arcantry` command after installation

#### Scenario: A user verifies a native download

- **WHEN** the reader installs from a GitHub Release
- **THEN** the documentation identifies the matching archive and `SHA256SUMS` verification path
- **AND** the provided sh and PowerShell installers verify the selected archive against that checksum manifest
- **AND** does not imply that an unsigned or unsupported distribution channel is available

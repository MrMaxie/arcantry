## ADDED Requirements

### Requirement: Installation guidance distinguishes native and npm use

The documentation MUST present direct native archives and the `arcantry` npm package as supported installation paths. It MUST list the supported operating-system, architecture and Linux libc matrix, explain that direct executables need no language runtime, and retain `npx arcantry` and equivalent package-runner guidance for users already using Node.js. It MUST NOT document unsupported package managers, targets, signing or automatic updates as delivered behavior.

#### Scenario: A user chooses an installation path

- **WHEN** the reader opens the CLI installation guidance
- **THEN** they can select the archive matching Windows, macOS or musl Linux on x64 or ARM64
- **AND** understand when the npm launcher and JavaScript runtime are required

#### Scenario: A user verifies a native download

- **WHEN** the reader installs from a GitHub Release
- **THEN** the documentation identifies the matching archive and `SHA256SUMS` verification path
- **AND** does not imply that an unsigned or unsupported distribution channel is available

## MODIFIED Requirements

### Requirement: Public package commands use the canonical npm identity

Documentation and interactive copy surfaces MUST derive or validate package launcher commands against the canonical `arcantry` package manifest name. Native download guidance MUST derive or validate target names against the declared release matrix rather than presenting platform package names as end-user commands.

#### Scenario: The npm package identity changes

- **WHEN** documentation generation and checks run
- **THEN** npm, pnpm and supported launcher examples use `arcantry`
- **AND** native archive examples use only declared release targets
- **AND** stale package scopes or target names fail validation instead of remaining in public copy

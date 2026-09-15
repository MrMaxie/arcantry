## MODIFIED Requirements

### Requirement: The public npm package uses the global Arcantry name

The public package MUST retain the unscoped name `arcantry` and expose the `arcantry` launcher. It MUST NOT expose JavaScript library subpaths or type declarations for product behavior. The launcher MUST dispatch to an exact-version optional platform package selected from the host operating system and architecture and MUST contain no product behavior beyond platform selection, native executable resolution, process dispatch and actionable installation failure. Linux platform packages MUST contain statically linked musl binaries, omit the npm `libc` field and be smoke-tested on both Ubuntu glibc and Alpine musl. The private workspace root MUST use a distinct non-publishable name so package-manager filters resolve the public package unambiguously.

#### Scenario: A consumer runs the CLI without installing it

- **WHEN** the consumer invokes the CLI through npm/npx, pnpm, Bun or Nub on a supported platform
- **THEN** npm resolves the public package and matching optional platform package
- **AND** the launcher executes the native `arcantry` binary

#### Scenario: A consumer imports a public module

- **WHEN** the consumer attempts to import a former `arcantry` JavaScript subpath from the packed package
- **THEN** the package exposes no supported JavaScript library API
- **AND** all Arcantry product behavior remains owned by the native executable

#### Scenario: Optional dependencies are unavailable

- **WHEN** the matching platform package was omitted or cannot be resolved
- **THEN** the launcher reports the unsupported or incomplete installation and an actionable reinstall or GitHub Release path
- **AND** does not download or execute code from the network

#### Scenario: Repository package references are validated

- **WHEN** package, workspace, test and public documentation surfaces are checked
- **THEN** they resolve the canonical `arcantry` package identity and declared platform package set
- **AND** every shipped or documented command uses the canonical public identity

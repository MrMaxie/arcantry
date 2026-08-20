## ADDED Requirements

### Requirement: Native releases support the declared platform matrix

Arcantry MUST publish native executables for `x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl`. Each executable MUST run without Node.js, Bun, Python or another language runtime.

#### Scenario: A supported platform installs Arcantry

- **WHEN** a user downloads the archive matching their declared operating system and architecture
- **THEN** the included `arcantry` executable runs the public CLI without a companion runtime
- **AND** no artifact for a different platform is required

#### Scenario: A platform is outside the matrix

- **WHEN** no declared target matches the user's operating system, architecture or Linux libc
- **THEN** Arcantry documentation and package launchers report that the platform is unsupported
- **AND** do not substitute an unverified artifact

### Requirement: Native archives and installers are deterministic and verifiable

Every sealed release MUST attach one ZIP archive for each Windows target, one TAR.XZ archive for each macOS and Linux target, one `SHA256SUMS` manifest covering all six archives, `arcantry-installer.sh` and `arcantry-installer.ps1`. Archive names and embedded CLI versions MUST derive from the sealed release version. Each archive MUST contain the target executable and the public license and MUST exclude source-only, private, local and repository-management state. The installers MUST use cargo-dist's supported PATH and unmanaged-install behavior, select only a declared target archive and verify the same SHA-256 digest recorded for that archive in `SHA256SUMS` before installation.

#### Scenario: A native release is prepared

- **WHEN** the six release archives are built from a sealed release
- **THEN** their names, executable versions and checksum manifest identify the same release and target triples
- **AND** recomputing every archive checksum matches `SHA256SUMS`

#### Scenario: A user runs a native installer

- **WHEN** a supported user runs `arcantry-installer.sh` or `arcantry-installer.ps1` for a sealed version
- **THEN** the installer selects the matching declared archive and verifies its checksum before installation
- **AND** a missing target or checksum mismatch stops installation

#### Scenario: An archive contains drift

- **WHEN** an archive has an unexpected path, mismatched version or checksum
- **THEN** external publication fails before the GitHub Release becomes public

### Requirement: Supported artifacts are executed on their target platforms

Cross-compilation alone MUST NOT qualify a target for publication. Each release candidate executable MUST run version, help and representative read-only and write-command smoke tests on its declared operating system and architecture before publication.

#### Scenario: A target build cannot be executed

- **WHEN** the release pipeline cannot run the candidate on its declared target
- **THEN** that target and the complete release remain unpublished
- **AND** the build is not reported as supported from compilation evidence alone

### Requirement: Runtime assets are embedded and safely materialized

The native executable MUST embed the public catalog, canonical skills, schemas and OpenSpec templates for its exact release. Commands that need only file content MUST use the embedded assets without requiring a companion directory. A command that requires durable files MUST materialize a complete, digest-verified catalog in an operating-system-standard per-user data directory namespaced by the Arcantry version.

#### Scenario: A command uses embedded content

- **WHEN** the native CLI inspects the catalog or initializes an owned template without an explicit external catalog
- **THEN** it uses assets matching its embedded release
- **AND** does not require files beside the executable

#### Scenario: Durable catalog files are required

- **WHEN** a skill link needs a filesystem source and no valid cache exists for the current version
- **THEN** Arcantry writes and verifies a temporary complete materialization before atomically making it active
- **AND** a partial or digest-invalid cache is never used as a link source

#### Scenario: A cache path contains unexpected content

- **WHEN** the expected versioned cache cannot be verified as Arcantry-owned
- **THEN** Arcantry reports the conflict without overwriting the content

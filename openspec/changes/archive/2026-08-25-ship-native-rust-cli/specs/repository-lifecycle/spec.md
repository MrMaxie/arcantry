## MODIFIED Requirements

### Requirement: External publication consumes sealed release state

Every external npm or GitHub Release publication MUST consume a repository state already sealed by the newest release manifest. The Git tag, native executable versions, native archive names and checksums, JavaScript package version, platform package versions and release-seal commit MUST identify the same release. Publication triggers and registry or release metadata MUST NOT define release prose, category, SemVer impact, visibility or components.

#### Scenario: A release tag matches the seal

- **WHEN** npm and native artifact publication run for `v<version>`
- **THEN** the tag version, newest release manifest, every distribution version and release-seal commit all match before external mutation

#### Scenario: Publication inputs disagree

- **WHEN** the tag, manifest, package version, native archive version, checksum set or checked-out commit does not identify the same sealed release
- **THEN** all unpublished outputs remain private and publication fails without replacing an existing artifact

#### Scenario: A version already exists

- **WHEN** the target `arcantry` package version or public GitHub Release already exists
- **THEN** publication fails as a duplicate instead of overwriting or reusing that version

#### Scenario: A platform package exists during a safe retry

- **WHEN** a platform package version was published before an interrupted main-package publication
- **THEN** the retry accepts it only when its immutable registry integrity matches the retained verified archive from the same seal
- **AND** otherwise fails without publishing the main package

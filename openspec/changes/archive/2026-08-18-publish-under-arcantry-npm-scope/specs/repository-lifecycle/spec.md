## ADDED Requirements

### Requirement: External publication consumes sealed release state

An external package publication MUST consume a repository state already sealed by the newest release manifest. Its trigger and registry metadata MUST NOT define release prose, category, SemVer impact, visibility or components.

#### Scenario: A release tag matches the seal

- **WHEN** npm publication runs for `v<version>`
- **THEN** the tag version, newest release manifest, distribution version and release-seal commit all match before registry mutation

#### Scenario: Publication inputs disagree

- **WHEN** the tag, manifest, package version or checked-out commit does not identify the same sealed release
- **THEN** publication fails without changing registry state

#### Scenario: A version already exists

- **WHEN** the target package version is already public in npm
- **THEN** publication fails as a duplicate instead of overwriting or reusing that version

## REMOVED Requirements

### Requirement: Local verification replaces mandatory CI

**Reason:** The accepted release now requires remote target execution and protected publication in addition to local completion gates.

**Migration:** Keep `just check-host` and `just linux-system-test` as local evidence, restore pull-request and `master` CI, and reserve the release workflow for sealed tags.

## ADDED Requirements

### Requirement: Local and remote verification have separate delivery roles

Implementation MUST pass Windows host checks and Linux Testcontainers before delivery. Pull requests and `master` MUST run repository CI, Pages MUST build and deploy documentation independently, and a sealed release tag MUST run the native and publication matrix. Coverage MUST remain diagnostic rather than a completion gate. Product and distributable versions MUST remain `1.0.0` for this first public release.

#### Scenario: Ordinary work reaches master

- **WHEN** a pull request is opened and merged
- **THEN** repository CI validates the proposed and delivered commit
- **AND** Pages builds documentation without becoming a release gate

#### Scenario: A release tag is pushed

- **WHEN** `v1.0.0` identifies the sealed release commit
- **THEN** the release workflow executes target qualification and protected publication
- **AND** local completion evidence is not substituted for execution of release artifacts on their declared platforms

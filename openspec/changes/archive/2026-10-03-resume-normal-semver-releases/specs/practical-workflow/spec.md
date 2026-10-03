## MODIFIED Requirements

### Requirement: Local and remote verification have separate delivery roles

Implementation MUST pass Windows host checks and Linux Testcontainers before delivery. Pull requests and `master` MUST run repository CI, Pages MUST build and deploy documentation independently, and a sealed release tag MUST run the native and publication matrix. Coverage MUST remain diagnostic rather than a completion gate. The first public release MUST retain `1.0.0` until publication; later versions MUST follow the highest accepted OpenSpec SemVer impact.

#### Scenario: Ordinary work reaches master

- **WHEN** a pull request is opened and merged
- **THEN** repository CI validates the proposed and delivered commit
- **AND** Pages builds documentation without becoming a release gate

#### Scenario: A release tag is pushed

- **WHEN** `v1.0.0` identifies the sealed release commit
- **THEN** the release workflow executes target qualification and protected publication
- **AND** local completion evidence is not substituted for execution of release artifacts on their declared platforms

#### Scenario: Patch work follows the first public release

- **WHEN** accepted patch work is prepared after 1.0.0 is live
- **THEN** the standard release plan selects 1.0.1
- **AND** ordinary implementation remains independent from release authorization

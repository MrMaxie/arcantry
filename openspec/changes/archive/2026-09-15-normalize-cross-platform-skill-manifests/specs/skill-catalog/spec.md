## MODIFIED Requirements

### Requirement: Public skills have independent identity and role

Every public skill MUST declare its own semantic version and one role from `primary`, `supporting` or `advanced`. The version MUST remain independent from the Arcantry product and CLI version. Generated catalog projections MUST expose both fields without changing skill invocation policy. Every projected package MUST include a deterministic SHA-256 manifest and a bounded history derived from accepted OpenSpec outcomes. Manifest hashing MUST normalize CRLF to LF for UTF-8 text resources and MUST preserve exact bytes for binary resources so the same package has one identity across supported checkout platforms.

#### Scenario: A skill changes while Arcantry remains at the same version

- **WHEN** an accepted skill revision changes package content without an Arcantry release
- **THEN** its exact revision and content digest change independently
- **AND** the Arcantry product and CLI remain at version `1.0.0`

#### Scenario: A text package is checked out on different platforms

- **WHEN** Git materializes equivalent UTF-8 skill resources with LF or CRLF line endings
- **THEN** both checkouts produce the same file hashes and package digest
- **AND** non-UTF-8 resources continue to use their exact bytes

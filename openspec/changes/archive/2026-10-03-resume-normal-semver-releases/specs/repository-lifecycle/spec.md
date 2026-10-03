## RENAMED Requirements

- FROM: `### Requirement: An unsealed first public release can be finalized in place`
- TO: `### Requirement: An unpublished first public release can be finalized in place`

## MODIFIED Requirements

### Requirement: An unpublished first public release can be finalized in place

The latest internal release manifest MAY be finalized without a version change only when neither its main package version nor a public GitHub Release exists. Finalization MUST retain the selected version, assign every accepted release-bearing change, use an explicit release date and regenerate the managed changelog from those OpenSpec outcomes. A failed Git tag or draft GitHub Release MAY be replaced only through an explicitly authorized reseal. A live npm version or public GitHub Release MUST remain immutable.

#### Scenario: The internal candidate has never been published

- **WHEN** `1.0.0` has no main npm package and no public GitHub Release
- **THEN** every later accepted release-bearing change may be assigned to the finalized 1.0.0 manifest
- **AND** an explicitly authorized failed tag may be replaced with the final seal

#### Scenario: The release already has an external identity

- **WHEN** the main npm version or public GitHub Release exists
- **THEN** later work requires a new version computed from OpenSpec impact

## ADDED Requirements

### Requirement: Repository release plans align every product version

The Arcantry repository release adapter MUST update the Rust workspace version, internal Arcantry dependency pins, lockfile workspace package versions, main npm package version and optional dependencies, every platform package version and both plugin manifest versions in one plan.

#### Scenario: A future patch release is cut

- **WHEN** release planning advances 1.0.0 to 1.0.1
- **THEN** every product and distribution version source is updated to 1.0.1 without manual follow-up edits

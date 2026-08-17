## MODIFIED Requirements

### Requirement: Arcantry dogfoods public repository validation

Arcantry CI MUST initialize ephemeral private adoption state through the built public CLI and then run the public repository and skill validation commands against the Arcantry repository in addition to internal unit and schema checks. Initialization MUST remain idempotent and MUST NOT commit `.local` state.

#### Scenario: CI verifies repository adoption

- **WHEN** the full repository quality gate runs
- **THEN** `arcantry repo validate` and `arcantry skills doctor` both inspect the current Arcantry repository

#### Scenario: CI starts from a clean checkout

- **WHEN** the checkout has no private Arcantry configuration
- **THEN** the quality gate runs `arcantry repo init --docs none` before public validation
- **AND** the generated `.local` state remains uncommitted and excluded from Git

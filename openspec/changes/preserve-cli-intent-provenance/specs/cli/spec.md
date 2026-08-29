## ADDED Requirements

### Requirement: Public CLI evidence is traceable to accepted intent

Every inventoried public CLI command and public CLI trust claim MUST reference an accepted OpenSpec requirement, its authored documentation location and executable evidence. Verification MUST resolve every reference and MUST fail when the documentation, requirement, inventory or observed native behavior disagrees.

#### Scenario: A public CLI claim changes

- **WHEN** authored documentation adds, removes or materially changes a CLI behavior or trust claim
- **THEN** verification requires the corresponding OpenSpec delta and executable evidence update
- **AND** the claim cannot pass solely because its command syntax still parses

#### Scenario: A referenced requirement moves

- **WHEN** a requirement or documentation anchor is renamed or removed
- **THEN** stale provenance fails verification until the reference is deliberately updated

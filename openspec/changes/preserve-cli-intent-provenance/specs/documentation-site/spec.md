## MODIFIED Requirements

### Requirement: Public CLI trust claims identify executable and normative evidence

Claims about read-only behavior, network behavior, repository writes, removal boundaries, rollback and private data MUST be no broader than named executable evidence and an accepted OpenSpec requirement. Unsupported or incompletely executed platforms MUST be described as contract targets rather than current execution evidence. Documentation verification MUST validate the complete provenance chain rather than checking only that a marker is present.

#### Scenario: A trust claim loses its evidence chain

- **WHEN** its accepted requirement, documentation reference or executable scenario is removed, renamed or rebound
- **THEN** documentation verification fails until the claim is narrowed or the complete evidence chain is restored

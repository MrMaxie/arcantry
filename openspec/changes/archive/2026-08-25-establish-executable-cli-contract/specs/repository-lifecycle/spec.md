## ADDED Requirements

### Requirement: Normal and sealed release checks remain distinct

Normal release checking MUST validate persisted release consistency while allowing active or unassigned work. Sealed checking MUST additionally require complete scoped assignment, a clean Git state and the configured release seal. The two modes MUST have separate executable native scenarios.

#### Scenario: Active work exists during normal checking

- **WHEN** persisted release artifacts are consistent and active or unassigned work exists
- **THEN** normal release checking succeeds
- **AND** sealed checking fails

## MODIFIED Requirements

### Requirement: Arcantry documents its own conformance

The contributor documentation MUST identify the repository verification surface and accurately distinguish unit tests, black-box CLI contract tests, diagnostic reports, blocking coverage policy, disposable Linux system tests and release-target execution. It MUST NOT describe an independent native contract suite as a comparison with a retired implementation or describe missing branch data as branch coverage.

#### Scenario: A contributor checks Arcantry itself

- **WHEN** the contributor follows the Contributing to Arcantry section
- **THEN** they can identify the repository verification surface and Arcantry-specific lifecycle rules

#### Scenario: A contributor selects a verification command

- **WHEN** the contributor follows the command reference
- **THEN** they can identify which boundary the command executes and whether failure blocks acceptance
- **AND** the documented claim matches the command's current implementation

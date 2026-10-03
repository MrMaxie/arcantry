## ADDED Requirements

### Requirement: Changelog rendering has a bounded output

Custom and preset changelog rendering MUST stop before generated managed content exceeds one MiB. Template source and execution fuel limits remain independent safeguards.

#### Scenario: A template amplifies release content

- **WHEN** repeated interpolation would produce more than one MiB of managed changelog output
- **THEN** rendering fails with a bounded-output error before retaining the oversized result

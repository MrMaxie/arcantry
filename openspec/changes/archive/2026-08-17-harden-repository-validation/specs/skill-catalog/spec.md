## ADDED Requirements

### Requirement: Canonical catalog metadata is schema strict

Catalog and skill metadata validation MUST require the canonical schema references, supported field allowlists, valid lowercase identifiers, unique tags and documented audience-facing text lengths. Source tooling and the distributed package MUST enforce the same contract.

#### Scenario: Unsupported metadata is introduced

- **WHEN** catalog or skill metadata contains an unknown field, invalid identifier, incorrect schema reference or out-of-range text
- **THEN** repository validation and packaged runtime validation reject the metadata before generating or exposing projections

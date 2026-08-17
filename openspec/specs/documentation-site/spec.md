# documentation-site Specification

## Purpose
Define the documentation content contract for developers adopting and verifying Arcantry.

## Requirements

### Requirement: Documentation explains the unified adoption journey

The authored documentation MUST explain how OpenSpec, `.docs/`, `.local/`, repository commands, skills, the catalog, and the Codex plugin fit together. It MUST distinguish public product guidance from private workstation state.

#### Scenario: A new adopter follows the documentation

- **WHEN** the reader opens the adoption guide
- **THEN** they can identify the minimum repository setup, the source of truth for each information layer, and the command that verifies the result

### Requirement: Cross-cutting guidance is authored and skill detail is generated

Adoption, repository workflow, CLI, and skills overview pages MUST be authored for their user journeys. Per-skill detail pages MUST be generated from canonical skill packages.

#### Scenario: Skill metadata is updated

- **WHEN** documentation generation runs
- **THEN** the skill detail changes without requiring a second authored edit

### Requirement: Arcantry documents its own conformance

The documentation MUST state that Arcantry validates its repository through the same commands and contracts it exposes to adopters. It MUST NOT claim a verification path that CI and local contributors cannot run.

#### Scenario: A contributor checks Arcantry itself

- **WHEN** the contributor runs the documented repository verification surface
- **THEN** it exercises the public Arcantry validation commands as part of the repository checks

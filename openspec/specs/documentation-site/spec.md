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

The contributor documentation MUST state that Arcantry validates its own repository through documented project commands and contracts. It MUST NOT present Arcantry's mandatory OpenSpec change and release lifecycle as a universal requirement for projects that only use Arcantry.

#### Scenario: A contributor checks Arcantry itself

- **WHEN** the contributor follows the Contributing to Arcantry section
- **THEN** they can identify the repository verification surface and Arcantry-specific lifecycle rules

### Requirement: Documentation links and presentation remain stable

The sidebar MUST use grouped Start, Guides, Concepts, Reference, and Contributing to Arcantry sections. Internal links and fragments from the overview and sidebar MUST resolve in the built site. This content change MUST preserve the existing Starlight shell, typography, animation, and visual composition except for copy, information order, commands, and links required by the new documentation architecture.

#### Scenario: A reader navigates the rebuilt documentation

- **WHEN** they follow an overview action or sidebar item
- **THEN** the intended page or section exists
- **AND** the established documentation shell remains visually unchanged

### Requirement: Public package commands use the canonical npm identity

Documentation and interactive copy surfaces MUST derive or validate package launcher commands against the canonical package manifest name.

#### Scenario: The npm package identity changes

- **WHEN** documentation generation and checks run
- **THEN** npm, pnpm and supported launcher examples use `@arcantry/arcantry`
- **AND** stale package scopes fail validation instead of remaining in public copy

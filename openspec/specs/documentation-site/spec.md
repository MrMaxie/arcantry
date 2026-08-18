# documentation-site Specification

## Purpose
Define the documentation content contract for developers adopting and verifying Arcantry.

## Requirements

### Requirement: Documentation explains the unified adoption journey

The authored documentation MUST explain how shared and private TOML configuration, OpenSpec, changelog, todo.txt, repository commands, portable skills, and the three catalog families fit together. It MUST distinguish shared project guidance from private workstation state and MUST present each source as independently adoptable.

#### Scenario: A new adopter follows the documentation

- **WHEN** the reader opens the adoption guide
- **THEN** they can identify the minimum shared or private setup, the source of truth for each information layer, and the commands that verify the result

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

The sidebar MUST use grouped Start, Guides, Concepts, Reference, and Contributing to Arcantry sections. Internal links and fragments from the overview and sidebar MUST resolve in the built site. Content MUST describe the current Arcantry contract directly and MUST preserve the approved Starlight shell, typography, animation, components, and visual composition except for copy, information order, commands, and links required by the product contract.

#### Scenario: A reader navigates the documentation

- **WHEN** they follow an overview action or sidebar item
- **THEN** the intended page or section exists
- **AND** the established documentation shell remains visually unchanged

### Requirement: Distribution guidance remains provider-neutral

Documentation MUST present standard Agent Skills user and repository locations, the Arcantry local linker, and compatible manual or independent installer workflows. It MUST NOT require a paid provider, remote issue tracker, or host-specific plugin to use the catalog.

#### Scenario: A developer adopts one skill

- **WHEN** they follow the skill installation guidance
- **THEN** they can choose a user-scoped or repository-scoped standard Agent Skills location
- **AND** no external service account is required

### Requirement: Public package commands use the canonical npm identity

Documentation and interactive copy surfaces MUST derive or validate package launcher commands against the canonical package manifest name.

#### Scenario: The npm package identity changes

- **WHEN** documentation generation and checks run
- **THEN** npm, pnpm and supported launcher examples use `arcantry`
- **AND** stale package scopes fail validation instead of remaining in public copy

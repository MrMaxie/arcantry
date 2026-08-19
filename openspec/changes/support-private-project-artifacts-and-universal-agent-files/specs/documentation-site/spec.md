## MODIFIED Requirements

### Requirement: Documentation explains the unified adoption journey

The authored documentation MUST explain how shared and private TOML configuration, OpenSpec, changelog, todo.txt, repository guidance, portable skills, and the three catalog families fit together. It MUST distinguish shared project state from private workstation state, present each source as independently adoptable, and describe `AGENTS.md` and `.agents` as universal surfaces rather than provider-owned files.

#### Scenario: A new adopter follows the documentation

- **WHEN** the reader opens the adoption guide
- **THEN** they can identify the minimum shared or private setup, the source of truth for each information layer, and the commands that verify the result
- **AND** Claude-specific files are presented only as optional compatibility adapters

### Requirement: Cross-cutting guidance is authored and skill detail is generated

Adoption, repository workflow, CLI, and skills overview pages MUST be authored for their user journeys. Per-skill detail pages and skill navigation MUST be generated from canonical skill packages. Cards and page leads MUST use canonical catalog summaries, while routing descriptions MUST remain visibly distinct from those summaries.

#### Scenario: Skill metadata is updated

- **WHEN** documentation generation runs
- **THEN** the skill detail, catalog entry and navigation change without requiring a second authored edit
- **AND** each public skill remains present exactly once

### Requirement: Distribution guidance remains provider-neutral

Documentation MUST recommend standard `.agents/skills` user and repository locations, the Arcantry local linker, and compatible manual or independent installer workflows. It MAY explain that Codex consumes the standard surface directly and MUST present Claude paths as optional compatibility adapters. It MUST NOT require a paid provider, remote issue tracker, host-specific plugin, or provider-branded directory to use the catalog.

#### Scenario: A developer adopts one skill

- **WHEN** they follow the skill installation guidance
- **THEN** they can choose a user-scoped or repository-scoped standard Agent Skills location
- **AND** they can explicitly add Claude compatibility without creating a second skill package

## MODIFIED Requirements

### Requirement: Documentation explains the unified adoption journey

The authored documentation MUST explain how shared and private TOML configuration, OpenSpec, changelog, todo.txt, repository commands, portable skills, and the three catalog families fit together. It MUST distinguish shared project guidance from private workstation state and MUST present each source as independently adoptable.

#### Scenario: A new adopter follows the documentation

- **WHEN** the reader opens the adoption guide
- **THEN** they can identify the minimum shared or private setup, the source of truth for each information layer, and the commands that verify the result

### Requirement: Documentation links and presentation remain stable

The sidebar MUST use grouped Start, Guides, Concepts, Reference, and Contributing to Arcantry sections. Internal links and fragments from the overview and sidebar MUST resolve in the built site. Content MUST describe the current Arcantry contract directly and MUST preserve the approved Starlight shell, typography, animation, components, and visual composition except for copy, information order, commands, and links required by the final product contract.

#### Scenario: A reader navigates the documentation

- **WHEN** they follow an overview action or sidebar item
- **THEN** the intended page or section exists
- **AND** the established documentation shell remains visually unchanged

## ADDED Requirements

### Requirement: Distribution guidance remains provider-neutral

Documentation MUST present standard Agent Skills user and repository locations, the Arcantry local linker, and compatible manual or independent installer workflows. It MUST NOT require a paid provider, remote issue tracker, or host-specific plugin to use the catalog.

#### Scenario: A developer adopts one skill

- **WHEN** they follow the skill installation guidance
- **THEN** they can choose a user-scoped or repository-scoped standard Agent Skills location
- **AND** no external service account is required

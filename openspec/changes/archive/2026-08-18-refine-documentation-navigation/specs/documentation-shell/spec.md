## ADDED Requirements

### Requirement: Documentation navigation supports quick scanning

The documentation shell MUST provide an icon for every navigable sidebar item, label the root route as "Homepage", and expose the generated skill catalog overview and every generated skill page in a collapsible catalog submenu. Individual skill pages MUST use one consistent document icon, while catalog groups MUST retain a distinct group icon and MAY contain nested groups. Desktop table-of-contents links MUST have enough vertical separation to scan individually. The resting search control MUST look inactive while preserving its existing hover and focus-visible states.

#### Scenario: A developer locates a skill page

- **WHEN** they expand Skill catalog in the sidebar
- **THEN** they can select the catalog overview or any generated skill page
- **AND** every visible navigable item includes an icon that distinguishes groups from skill documents

### Requirement: Documentation data and process views remain readable

Documentation tables MUST distinguish their header from body content and MUST use subtle alternating row and column backgrounds to support scanning across dense cells. Process flows MUST render as responsive Mermaid diagrams using the Beautiful Mermaid renderer rather than as manually aligned text diagrams.

#### Scenario: A developer scans structured documentation

- **WHEN** they read an adoption table or release flow
- **THEN** table headers and cell relationships remain visually distinct
- **AND** the release sequence is presented as a responsive connected diagram

### Requirement: Syntax highlighting fits the documentation palette

Documentation code blocks MUST use the bundled Catppuccin Latte theme in light mode and Catppuccin Mocha theme in dark mode while retaining the Arcantry code-frame treatment.

#### Scenario: A developer reads code in either site theme

- **WHEN** they switch between light and dark themes
- **THEN** syntax tokens use the matching Catppuccin palette
- **AND** the surrounding Arcantry code frame remains readable and intact

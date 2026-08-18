## ADDED Requirements

### Requirement: Documentation navigation supports quick scanning

The documentation shell MUST provide an icon for every navigable sidebar item, label the root route as "Homepage", and expose the generated skill catalog overview and every generated skill page in a collapsible catalog submenu. Desktop table-of-contents links MUST have enough vertical separation to scan individually. The resting search control MUST look inactive while preserving its existing hover and focus-visible states.

#### Scenario: A developer locates a skill page

- **WHEN** they expand Skill catalog in the sidebar
- **THEN** they can select the catalog overview or any generated skill page
- **AND** every visible navigable item includes a meaningful icon

### Requirement: Syntax highlighting fits the documentation palette

Documentation code blocks MUST use the bundled Catppuccin Latte theme in light mode and Catppuccin Mocha theme in dark mode while retaining the Arcantry code-frame treatment.

#### Scenario: A developer reads code in either site theme

- **WHEN** they switch between light and dark themes
- **THEN** syntax tokens use the matching Catppuccin palette
- **AND** the surrounding Arcantry code frame remains readable and intact

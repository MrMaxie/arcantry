## MODIFIED Requirements

### Requirement: Documentation shell matches the approved Arcantry concept

The desktop documentation shell MUST present a full-height left navigation rail, an independent top content bar, and an editorial main content area matching the approved Arcantry documentation concept rather than Starlight's default shell.

#### Scenario: A developer navigates the documentation

- **WHEN** they move between overview, adoption, catalog, and reference pages
- **THEN** the shell stays stable, the current location remains clear, and the page does not repeat the brand

### Requirement: Brand assets are structural UI

The Arcantry mark and wordmark MUST be used as primary navigation branding rather than decorative content inside the page body.

#### Scenario: A reader moves between viewport sizes

- **WHEN** the documentation changes between desktop and narrow layouts
- **THEN** the brand remains in the top-left navigation position and no page shows a second logo

### Requirement: Overview follows the approved composition

The overview MUST use the approved composition: serif-led hero, restrained primary action, six compact navigation cards, and a two-column lifecycle explanation.

#### Scenario: A reader opens the overview

- **WHEN** the overview is rendered
- **THEN** its hero, primary action, navigation cards, and lifecycle explanation preserve the approved hierarchy

### Requirement: Starlight remains infrastructure

Routing, content collections, search, accessibility behavior, and mobile navigation MAY remain powered by Starlight, but default Starlight visual conventions MUST NOT dominate the rendered interface.

#### Scenario: A reader uses documentation infrastructure

- **WHEN** they search, navigate by keyboard, or open the mobile menu
- **THEN** Starlight behavior remains available inside the Arcantry visual shell

### Requirement: Responsive behavior preserves hierarchy

On narrow screens the desktop rail MAY collapse, but brand, search, primary navigation, and content hierarchy MUST remain accessible without horizontal overflow.

#### Scenario: A reader opens a narrow viewport

- **WHEN** the available width cannot contain the desktop rail
- **THEN** brand, search, navigation, and content remain accessible without horizontal overflow

### Requirement: Theme state is functional

The documentation MUST provide distinct light and dark semantic palettes. Changing the Starlight theme state MUST visibly update background, text, border, control and surface colors without relying on page reloads.

#### Scenario: A reader changes the theme

- **WHEN** they select a different supported theme state
- **THEN** semantic colors update without a page reload

### Requirement: Components consume semantic tokens

Documentation components MUST use semantic theme variables for colors that differ by theme. Hardcoded light-only foreground and background colors MUST NOT be used in reusable chrome or overview components.

#### Scenario: A component renders in both themes

- **WHEN** a reusable chrome or overview component changes between light and dark themes
- **THEN** its foreground and background colors come from semantic theme variables

### Requirement: Brand remains legible in both themes

The canonical Arcantry mark and wordmark MUST remain visually legible in light and dark themes without substituting alternate brand geometry.

#### Scenario: Brand assets render in each theme

- **WHEN** the documentation changes between light and dark themes
- **THEN** the canonical mark and wordmark remain legible without geometry substitution

### Requirement: Visual language remains restrained

The documentation MUST prefer flat surfaces, hairline separators and square-to-subtle corner treatment. Glass blur, glow, decorative gradients, oversized pill controls, gratuitous shadows and bento-style filler containers MUST NOT be introduced unless an approved concept explicitly calls for them.

#### Scenario: A reader opens a reference page

- **WHEN** the page is not the overview
- **THEN** product content remains static, legible, and visually quieter than the landing surface

## Purpose

Define the visual and interaction contract for Arcantry documentation so the published site represents the approved brand without giving up Starlight's documentation capabilities.

## Requirements

### Requirement: Documentation shell matches the approved Arcantry concept

The published documentation MUST use Arcantry's shell rather than Starlight's default visual layout. The shell MUST include one responsive top-left brand lockup, a wide search region, a compact action group, meaningful navigation icons, and Arcantry typography and spacing.

#### Scenario: A developer navigates the documentation

- **WHEN** they move between overview, adoption, catalog, and reference pages
- **THEN** the shell stays stable, the current location remains clear, and the page does not repeat the brand

### Requirement: Brand assets are first-class

The documentation shell MUST use the Arcantry compact mark, full wordmark and favicon from `src/assets/brand` as structural navigation and browser assets.

#### Scenario: The documentation shell renders brand assets

- **WHEN** a page and its browser metadata are loaded
- **THEN** structural branding resolves from the canonical assets in `src/assets/brand`

### Requirement: Documentation remains operational

Search, responsive navigation, accessible content navigation, and system/light/dark themes MUST continue to work. The theme control MUST be an icon button whose accessible name and visible icon communicate the current state. Client-side page transitions MUST preserve theme and navigation behavior and MUST honor reduced-motion preferences.

#### Scenario: A keyboard user changes theme and navigates

- **WHEN** they activate the theme control and follow an internal link
- **THEN** focus remains visible, the selected theme persists, and navigation completes without a full-page flash

### Requirement: Overview page presents the developer path

The overview MUST state a concrete repository outcome, show the current released Arcantry version, provide the exact install command, explain the shortest adoption and verification path, and link to the skill catalog. Benefits MUST describe observable product behavior rather than broad feature labels.

#### Scenario: A new developer opens the overview

- **WHEN** they scan the first content region
- **THEN** they can identify what Arcantry changes, which command starts adoption, and how to verify the result

## MODIFIED Requirements

### Requirement: Documentation shell matches the approved Arcantry concept

The documentation MUST render its own wide desktop shell, including an independent left navigation rail and a separate top content bar, instead of exposing the default Starlight shell as the visible product interface.

#### Scenario: A developer navigates the documentation

- **WHEN** they move between overview, adoption, catalog, and reference pages
- **THEN** the shell stays stable, the current location remains clear, and the page does not repeat the brand

### Requirement: Overview page preserves the approved composition

The overview page MUST include the approved serif hero, concise product statement, primary and secondary actions, six navigation cells, lifecycle explanation and forward navigation without introducing a second brand mark.

#### Scenario: A new developer opens the overview

- **WHEN** they scan the first content region
- **THEN** they can identify what Arcantry changes, which command starts adoption, and how to verify the result

### Requirement: Brand assets are first-class

The documentation shell MUST use the Arcantry compact mark, full wordmark and favicon from `src/assets/brand` as structural navigation and browser assets.

#### Scenario: The documentation shell renders brand assets

- **WHEN** a page and its browser metadata are loaded
- **THEN** structural branding resolves from the canonical assets in `src/assets/brand`

### Requirement: Documentation remains operational

Search, responsive navigation, accessible content navigation and system/light/dark themes MUST continue to work after the custom shell is applied.

#### Scenario: A keyboard user changes theme and navigates

- **WHEN** they activate the theme control and follow an internal link
- **THEN** focus remains visible, the selected theme persists, and navigation completes without a full-page flash

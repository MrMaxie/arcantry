## MODIFIED Requirements

### Requirement: Brand assets are structural UI

The documentation shell MUST show exactly one Arcantry brand lockup at a time. The desktop lockup MUST use the wordmark and the narrow layout MUST use the compact mark. The wordmark's first glyph MUST reuse the compact mark geometry rather than a similar alternate drawing.

#### Scenario: A reader moves between viewport sizes

- **WHEN** the documentation changes between desktop and narrow layouts
- **THEN** the brand remains in the top-left navigation position and no page shows a second logo

### Requirement: Visual language remains restrained

The documentation MUST prefer flat surfaces, hairline separators, meaningful icons, and square-to-subtle corner treatment. Marketing motion MAY appear on the overview when it explains product behavior and respects reduced-motion preferences. Reference and skill pages MUST NOT use decorative canvas effects, glow, glass blur, oversized pills, gratuitous shadows, or filler containers.

#### Scenario: A reader opens a reference page

- **WHEN** the page is not the overview
- **THEN** product content remains static, legible, and visually quieter than the landing surface

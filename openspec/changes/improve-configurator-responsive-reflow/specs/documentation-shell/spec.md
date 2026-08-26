## MODIFIED Requirements

### Requirement: The configurator uses the documentation application shell

The configurator MUST use three regions only while setup navigation, questions and generated instructions each retain their defined readable minimum size. At intermediate widths it MUST reduce the parallel regions before text, controls or the preview become undersized, and at narrow widths it MUST preserve the questions as the primary column with the preview following the question flow. Layout changes MUST preserve every answer, available action, progress state, focus target and programmatic relationship.

The configurator MUST support browser zoom equivalent to a 320 CSS pixel viewport, 200 percent text resizing and user text-spacing overrides without loss of content or functionality or page-level scrolling in two dimensions. A region that requires its own scrolling MUST remain keyboard, pointer and touch accessible and MUST NOT obscure focused content.

The configurator MUST reuse the documentation sidebar's link treatment and credit footer in the left region on wide screens instead of approximating their spacing, width, typography or content. The compact mobile brand symbol MUST remain readable in both light and dark themes. Header search, repository and theme icons MUST use a moderate common visual size, and the search control MUST NOT display a keyboard-shortcut badge on any page. The full-width configurator header and workspace MUST use the same outer alignment instead of independently centered maximum widths.

#### Scenario: A visitor uses the configurator on a wide screen

- **WHEN** the viewport can display the documentation application shell
- **THEN** setup navigation, questions and the instructions preview appear as left, center and right regions
- **AND** the header aligns to the same full-width grid
- **AND** selecting an answer enables its applicable follow-up stage link

#### Scenario: A visitor uses the configurator below the documentation sidebar breakpoint

- **WHEN** the viewport is narrower than the documentation sidebar breakpoint
- **THEN** the stage navigation is available through a compact control above the questions
- **AND** the instructions preview follows the question flow
- **AND** the compact Arcantry symbol has sufficient contrast for the active theme

#### Scenario: The three-region workspace becomes constrained

- **WHEN** the viewport or browser zoom cannot preserve readable minimum sizes for all three regions
- **THEN** the configurator reduces its parallel layout before shrinking question or preview text
- **AND** the user retains every answer, action and progress relationship

#### Scenario: A user enlarges the configurator

- **WHEN** content is presented at the 320 CSS pixel reflow equivalent or text is resized to 200 percent
- **THEN** all non-exempt content remains available without two-dimensional page scrolling
- **AND** keyboard focus is not hidden by fixed or independently scrolling regions

#### Scenario: Side-by-side comparison remains useful

- **WHEN** the viewport provides enough width for readable questions and generated instructions
- **THEN** the preview remains available beside the question flow
- **AND** the layout continues to use the established visual system

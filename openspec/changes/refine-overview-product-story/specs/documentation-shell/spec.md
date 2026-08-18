## MODIFIED Requirements

### Requirement: Documentation shell matches the approved Arcantry concept

The published documentation MUST use Arcantry's shell rather than Starlight's default visual layout. The shell MUST include one responsive brand lockup with the current released version, a search region, a compact action group, meaningful navigation icons, Arcantry typography and spacing, and a global footer with authorship, a filled rose-pink heart with a soft glow and visible non-layout-shifting animation, and GitHub link. The header MUST remain one horizontal row and MAY use restrained transparency and backdrop blur. On the overview, its contents MUST align to the centered overview content column and search MUST remain compact beside the brand, while documentation routes MUST expand the same row and search region across the full reading shell. GitHub and theme actions MUST render as aligned, unframed icon buttons. The theme control MUST expose only light and dark states, initialize from the system preference when no override exists, follow later system changes until the user chooses a state, and persist that explicit choice locally. The overview hero title MUST render the full descender of its accented final word, and its secondary action MUST use adaptation-oriented wording.

#### Scenario: A developer navigates the documentation

- **WHEN** they move between overview, adoption, catalog, and reference pages
- **THEN** the shell stays stable, the current location remains clear, and the page does not repeat the brand
- **AND** the released version remains under the brand while authorship, GitHub link, and current theme state remain available
- **AND** overview-to-documentation navigation animates the header row from the centered overview column to the full documentation width

#### Scenario: The overview hero renders

- **WHEN** the accented title contains a descender
- **THEN** the glyph remains fully visible above the decorative underline
- **AND** the secondary action is labeled "Adapt your project"

### Requirement: Overview page presents the developer path

The overview MUST explain in its first content region that Arcantry maps project knowledge sources, preserves their distinct responsibilities, and lets a developer observe, validate, or change them independently. It MUST then present a directional with-and-without comparison with at least discovery, responsibility, adoption, planning, drift, and automation outcomes. Explanatory comparison and adoption copy MUST describe behavior without relying on fragmentary CLI commands. Each directional comparison MUST use a single connected line-and-arrow graphic rather than visually separate connector pieces.

Its explanatory map MUST act as navigation to getting started, adoption paths, project knowledge, todo.txt queues, and skill selection rather than as a decorative summary. It MUST explain private workspace behavior without assuming that a new reader already understands project-specific directory names. It MUST present npm, pnpm, Bun, and Nub as package-runner tabs separated from PowerShell, Scoop, Homebrew, and portable shell tabs in one command picker. System-specific tabs MUST include recognizable platform marks, and narrow viewports MUST wrap both groups into visible rows. The selected runner or installer MUST persist locally, the active command row MUST be the copy target, and recognized inline Arcantry commands across documentation MUST be selectable and copyable by pointer or keyboard. Command examples MUST explain the action they perform rather than appearing without context. Benefits MUST describe observable product behavior rather than broad feature labels, delivery-process commentary, or local checkout details.

The overview MUST present complete integration for either a new or existing project as the recommended destination while preserving private use in an established repository and selective adoption as supported choices. Only complete integration MUST carry a recommendation label. The private preset MUST keep repository files untouched, the selective preset MUST demonstrate a varied partial configuration, and changing an individual field MUST activate a custom state. A configuration questionnaire MUST expose independent choices for skill scope, settings scope, private `.local` use, todo.txt location, OpenSpec management, and changelog management. Each field MUST include an identifying icon, a plain-language explanation for a reader unfamiliar with the source, compact labeled controls, and a consequence placed directly below those controls that updates with the selected value. Selected controls MUST use the same blue, pink, violet, or combined scope colors as the graph. Preset and field selections MUST transition their colors instead of changing abruptly. The presets MUST populate those fields together without a redundant instructional heading or result strip, and the selected preset title MUST remain on one line when the available desktop width can contain it. Editing any individual field MUST stop automatic preset changes. The derived graph MUST arrange Computer, Repository, and Private per project scopes in that order, use separate non-crossing routes, replace generic source dots with source-specific icons, and reflect every questionnaire choice in its active nodes. Scope, route, and source states MUST transition visibly between active and inactive states, and the Arcantry engine MUST use a moving blue-violet-pink highlight that stops for reduced motion. Presets MAY advance automatically, while reduced-motion preference MUST stop automatic changes. The previous expected-outcome card composition MUST appear earlier in the product-value story. Every with-and-without comparison cell MUST visually emphasize its key phrase. The overview MUST end with authorship, last update, license, and GitHub details without repeating the released version already shown in the header; documentation routes MAY keep equivalent authorship in their sidebar instead. The overview's next-page action MUST preserve the standard pager shape and MAY animate a blue-violet-pink gradient border, while documentation routes retain the compact standard pager.

#### Scenario: A new developer opens the overview

- **WHEN** they scan the first content region
- **THEN** they can identify what Arcantry composes, choose a package runner or platform installer, copy its command, and find an adoption path

#### Scenario: A developer returns to the overview

- **WHEN** they previously selected any documented runner or installer
- **THEN** the overview restores that runner without changing the command's Arcantry behavior

#### Scenario: A new developer evaluates Arcantry

- **WHEN** they scan the overview from the hero through the comparison
- **THEN** they can state what Arcantry maps, what remains independent, how changes become reviewable, and why adoption does not need to be all-or-nothing

#### Scenario: A developer composes an adoption strategy

- **WHEN** they choose a typical preset or edit an individual questionnaire field
- **THEN** active sources and relationships identify the chosen project footprint and management boundary
- **AND** the selected field values explain their practical consequences
- **AND** an individual field edit activates the custom state and stops preset rotation
- **AND** private, selective, and fully integrated states remain clearly distinguishable

#### Scenario: A visitor prefers reduced motion

- **WHEN** reduced motion is requested
- **THEN** the questionnaire and graph remain manually operable without automatically changing preset state

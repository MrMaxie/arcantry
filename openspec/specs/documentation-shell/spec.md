## Purpose

Define the visual and interaction contract for Arcantry documentation so the published site represents the approved brand without giving up Starlight's documentation capabilities.

## Requirements

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

The overview MUST explain in its first content region that Arcantry maps project knowledge sources, preserves their distinct responsibilities, and lets a developer observe, validate, or change them independently. It MUST then present a directional with-and-without comparison with at least discovery, responsibility, adoption, planning, drift, and automation outcomes. Explanatory comparison and adoption copy MUST describe behavior without relying on fragmentary CLI commands. Each directional comparison MUST use a single connected line-and-arrow graphic rather than visually separate connector pieces.

Its explanatory map MUST act as navigation to getting started, adoption paths, project knowledge, todo.txt queues, and skill selection rather than as a decorative summary. It MUST explain private workspace behavior without assuming that a new reader already understands project-specific directory names. It MUST present verified npm, pnpm, Bun, and Nub package-runner tabs in one command picker, and narrow viewports MUST wrap them into visible rows. The selected runner MUST persist locally, the active command row MUST be the copy target, and recognized inline Arcantry commands across documentation MUST be selectable and copyable by pointer or keyboard. Command examples MUST explain the action they perform rather than appearing without context. Benefits MUST describe observable product behavior rather than broad feature labels, delivery-process commentary, or local checkout details.

The overview MUST present a shared and private model as the recommended destination while preserving private project scope and selected capabilities as supported choices. Only the shared and private model MUST carry a recommendation label. The private preset MUST keep its configuration and guidance untracked, the selective preset MUST demonstrate a varied partial configuration, and changing an individual field MUST activate a custom state. A configuration questionnaire MUST expose independent choices for skill scope, agent-guidance scope, private `.local` use, todo.txt location, OpenSpec management, and changelog management. Each field MUST include an identifying icon, a plain-language explanation for a reader unfamiliar with the source, compact labeled controls, and a consequence placed directly below those controls that updates with the selected value. Selected controls MUST use the same blue, pink, violet, or combined scope colors as the graph. Preset and field selections MUST transition their colors instead of changing abruptly. The presets MUST populate those fields together without a redundant instructional heading or result strip, and the selected preset title MUST remain on one line when the available desktop width can contain it. Editing any individual field MUST stop automatic preset changes. The derived graph MUST arrange Computer, Repository, and Private per project scopes in that order, use separate non-crossing routes, replace generic source dots with source-specific icons, and reflect every questionnaire choice in its active nodes. Scope, route, and source states MUST transition visibly between active and inactive states. The Arcantry engine MUST use the same current full wordmark as the header in a compact node, with a moving blue-violet-pink highlight that stops for reduced motion. Presets MAY advance automatically, while reduced-motion preference MUST stop automatic changes. The expected-outcome card composition MUST appear earlier in the product-value story. Every with-and-without comparison cell MUST visually emphasize its key phrase. The overview MUST end with authorship, last update, license, and GitHub details without repeating the released version already shown in the header; documentation routes MAY keep equivalent authorship in their sidebar instead. The overview's next-page action MUST preserve the standard pager shape and MAY animate a blue-violet-pink gradient border, while documentation routes retain the compact standard pager.

#### Scenario: A new developer opens the overview

- **WHEN** they scan the first content region
- **THEN** they can identify what Arcantry composes, choose a verified package runner, copy its command, and find an adoption path

#### Scenario: A developer returns to the overview

- **WHEN** they selected any documented package runner
- **THEN** the overview restores that runner without changing the command's Arcantry behavior

#### Scenario: A new developer evaluates Arcantry

- **WHEN** they scan the overview from the hero through the comparison
- **THEN** they can state what Arcantry maps, what remains independent, how changes become reviewable, and why adoption does not need to be all-or-nothing

#### Scenario: A developer composes an adoption strategy

- **WHEN** they choose a typical preset or edit an individual questionnaire field
- **THEN** active sources and relationships identify the chosen project footprint and management boundary
- **AND** the selected field values explain their practical consequences
- **AND** an individual field edit activates the custom state and stops preset rotation
- **AND** private, selected-capability, and shared-with-private states remain clearly distinguishable

#### Scenario: A visitor prefers reduced motion

- **WHEN** reduced motion is requested
- **THEN** the questionnaire and graph remain manually operable without automatically changing preset state

### Requirement: Visual language combines clarity with selective expression

The documentation MUST prefer flat reading surfaces, hairline separators, meaningful unframed icons, one shared serif family, and square-to-subtle corner treatment. The overview MUST include a lightweight blue, violet, and rose-pink magical canvas atmosphere inspired by the softness and sparkle of the Arcantry identity without reproducing the logo itself. The atmosphere MUST span the viewport without exposing a hard edge during resize or navigation, distribute a dense but restrained field of angled falling stars across the hero, let independently moving motes fade fully between varied brightness peaks, fade near its top and bottom edges, and show pointer-focused light only while the pointer is actively inside the hero. The overview command picker and shared header MAY use restrained transparency and backdrop blur, but command text MUST remain readable over the ambient layer in both themes. The three value cards and three adoption steps MUST each align to the tallest item in their row. The adoption steps MUST use a slow sequential background glow without animated frames or layout movement. The information-boundary navigation surface MAY use a restrained moving accent background that does not move its content. Marketing motion MUST respect reduced-motion preferences. Reference and skill content surfaces MUST NOT use decorative canvas effects, glass panels, oversized pills, gratuitous shadows, filler containers, or framed icons that do not communicate a boundary or state.

#### Scenario: A reader scans documentation controls and steps

- **WHEN** an icon represents an action or step without its own bounded surface
- **THEN** the icon remains visually unframed while focus and selected states stay perceivable

#### Scenario: A visitor explores the overview

- **WHEN** they move the pointer through the hero or leave the hero idle
- **THEN** the ambient canvas responds with restrained motion and the primary content remains readable
- **AND** pointer-focused light is absent after pointer exit or page scrolling
- **AND** reduced-motion preference produces a static composition

### Requirement: Documentation navigation preserves visual continuity

The documentation MUST enable Astro's built-in client router site-wide. The shared header MUST remain visually stable while changing content crossfades without geometric distortion and a colored navigation-progress cue communicates loading. Fixed documentation dividers MUST begin below the header in both live and transition snapshots. The overview pager and standard documentation pager MAY share a transition identity. Navigation MUST retain Astro's fallback and route-announcement behavior and MUST disable transition motion when reduced motion is requested.

#### Scenario: A developer follows an internal documentation link

- **WHEN** navigation starts and the next document loads
- **THEN** the header remains stable, a blue-to-pink progress cue communicates the pending navigation, and the new content enters without a full-page visual flash

### Requirement: Accent color remains controlled and recognizable

The documentation MUST define a blue, violet, and pink accent set with theme-appropriate contrast. Accent colors MUST establish recognizable rhythm across syntax roles, selected controls, visible focus, active navigation, section markers, hero atmosphere, and authorship details without replacing large reading surfaces or ordinary body copy.

#### Scenario: A reader scans the documentation

- **WHEN** accent colors appear in either theme
- **THEN** they clarify state or structure without replacing the neutral Arcantry palette

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

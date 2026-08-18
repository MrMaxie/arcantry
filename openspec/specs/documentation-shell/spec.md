## Purpose

Define the visual and interaction contract for Arcantry documentation so the published site represents the approved brand without giving up Starlight's documentation capabilities.

## Requirements

### Requirement: Documentation shell matches the approved Arcantry concept

The published documentation MUST use Arcantry's shell rather than Starlight's default visual layout. The shell MUST include one responsive brand lockup with the current released version, a search region, a compact action group, meaningful navigation icons, Arcantry typography and spacing, and a global footer with authorship, a filled rose-pink heart with a soft glow and visible non-layout-shifting animation, and GitHub link. The header MUST remain one horizontal row and MAY use restrained transparency and backdrop blur. On the overview, its contents MUST align to the centered overview content column and search MUST remain compact beside the brand, while documentation routes MUST expand the same row and search region across the full reading shell. GitHub and theme actions MUST render as aligned, unframed icon buttons. The theme control MUST expose only light and dark states, initialize from the system preference when no override exists, follow later system changes until the user chooses a state, and persist that explicit choice locally.

#### Scenario: A developer navigates the documentation

- **WHEN** they move between overview, adoption, catalog, and reference pages
- **THEN** the shell stays stable, the current location remains clear, and the page does not repeat the brand
- **AND** the released version remains under the brand while authorship, GitHub link, and current theme state remain available
- **AND** overview-to-documentation navigation animates the header row from the centered overview column to the full documentation width

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

The overview MUST state a concrete project outcome, then present product value, a directional with-and-without comparison, navigable product areas, and inspect-first adoption choices. Each directional comparison MUST use a single connected line-and-arrow graphic rather than visually separate connector pieces. Expected project outcomes MUST follow the adoption choices as an additional, hierarchically composed set of observable benefits. Its explanatory map MUST act as navigation to getting started, adoption paths, project knowledge, todo.txt queues, and skill selection rather than as a decorative summary. It MUST explain private workspace behavior without assuming that a new reader already understands project-specific directory names. It MUST offer equivalent npm, pnpm, and Nub commands for running `repo inspect` from the released Arcantry package. The selected runner MUST persist locally, the active command row MUST be the copy target, and recognized inline Arcantry commands across documentation MUST be selectable and copyable by pointer or keyboard. Command examples MUST explain the action they perform rather than appearing without context. Benefits MUST describe observable product behavior rather than broad feature labels or local checkout details. The overview's next-page action MUST preserve the standard pager shape and MAY animate a blue-violet-pink gradient border, while documentation routes retain the compact standard pager.

#### Scenario: A new developer opens the overview

- **WHEN** they scan the first content region
- **THEN** they can identify what Arcantry composes, choose their package runner, copy the read-only inspection command, and find an adoption path

#### Scenario: A developer returns to the overview

- **WHEN** they previously selected npm, pnpm, or Nub
- **THEN** the overview restores that runner without changing the command's Arcantry behavior

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

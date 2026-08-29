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

The documentation shell MUST use only the current full wordmark and the favicon `a` as structural navigation and browser assets. The full wordmark MUST appear on desktop, the favicon mark MUST replace it on narrow layouts, and no third compact logo variant MAY appear. Narrow layouts MUST keep the released version legible and make search, GitHub, and theme icons comfortably visible without changing the one-row header.

#### Scenario: The documentation shell renders brand assets

- **WHEN** a page and its browser metadata are loaded
- **THEN** structural branding resolves from the canonical full wordmark in `src/assets` and favicon in `public`

### Requirement: Documentation remains operational

Search, responsive navigation, accessible content navigation, and system/light/dark themes MUST continue to work. The theme control MUST be an icon button whose accessible name and visible icon communicate the current state. Client-side page transitions MUST preserve theme and navigation behavior and MUST honor reduced-motion preferences.

#### Scenario: A keyboard user changes theme and navigates

- **WHEN** they activate the theme control and follow an internal link
- **THEN** focus remains visible, the selected theme persists, and navigation completes without a full-page flash

### Requirement: Overview page presents the developer path

The overview MUST explain in its first content region that Arcantry maps project knowledge sources, preserves their distinct responsibilities, and lets a developer observe, validate, or change them independently. It MUST then present a directional with-and-without comparison with at least discovery, responsibility, adoption, planning, drift, and automation outcomes. Explanatory comparison and adoption copy MUST describe behavior without relying on fragmentary CLI commands. Each directional comparison MUST use a single connected line-and-arrow graphic rather than visually separate connector pieces.

Its explanatory map MUST act as navigation to getting started, adoption paths, project knowledge, todo.txt queues, and skill selection rather than as a decorative summary. It MUST explain private workspace behavior without assuming that a new reader already understands project-specific directory names. It MUST present npm installation, npm/npx, pnpm, and Nub package launchers, and native PowerShell and sh installers in one command picker, and narrow viewports MUST wrap them into visible rows. The selected method MUST persist locally, and the active command row MUST be the copy target. The overview picker MUST contain only the method selector, command, and copy affordance without redundant purpose labels or instructions. Recognized inline Arcantry commands across documentation MUST be selectable and copyable by pointer or keyboard. Documentation command examples outside the overview picker MUST explain the action they perform. The overview MUST offer a "Copy agent prompt" action, and the getting-started guide MUST show the exact full prompt. That prompt MUST ask an agent to detect the environment, explain material installation choices, avoid project adoption unless requested, verify the installed version, and stop instead of inventing unavailable release commands. Benefits MUST describe observable product behavior rather than broad feature labels, delivery-process commentary, or local checkout details.

The overview MUST present a shared and private model as the recommended destination while preserving private project scope and selected-source presets as supported choices. Only the shared and private model MUST carry a recommendation label, and it MUST appear first and be selected by default. A concise label MUST identify these choices as common setups. Each preset control MUST include its short description without a separate header that repeats the selected preset. A lightweight blue-violet-pink synchronization wave with the approved compact Arcantry mark MUST separate the presets from their scope view. The private preset MUST keep project guidance, todo.txt, OpenSpec, and changelog in private `.local` while retaining user-scoped skills. The selective preset MUST demonstrate a varied partial configuration. One stable three-column view MUST represent Computer, Repository, and Private project scopes. The columns MUST sit directly beside one another without nested cards, surrounding panel padding, decorative folder tabs, or gaps. Each column MUST use its own blue, pink, or violet scope color and an icon that identifies the scope. Every compatible source entry MUST remain visible in every preset; selecting a preset MUST change only its active state, subtle background, and mode. Inactive entries MUST remain visibly muted without dimming or otherwise changing their separator borders. Each active source MUST identify its effective user, project, private, read, or manage mode. The overview MUST NOT expose per-source editing, a custom state, an inactive-node graph, a decorative engine, or a duplicate mobile representation. Private `.local` MUST support todo.txt, OpenSpec, and changelog sources. Presets MUST remain stable until the reader selects another preset. The expected-outcome card composition MUST appear earlier in the product-value story. Every with-and-without comparison cell MUST visually emphasize its key phrase. The overview MUST end with authorship, last update, license, and GitHub details without repeating the released version already shown in the header; documentation routes MAY keep equivalent authorship in their sidebar instead. The overview's next-page action MUST preserve the standard pager shape and MAY animate a blue-violet-pink gradient border, while documentation routes retain the compact standard pager.

#### Scenario: A new developer opens the overview

- **WHEN** they scan the first content region
- **THEN** they can identify what Arcantry composes, choose an installation or one-command package path, copy its command, and find an adoption path

#### Scenario: A developer returns to the overview

- **WHEN** they selected any documented package runner
- **THEN** the overview restores that runner without changing the command's Arcantry behavior

#### Scenario: A new developer evaluates Arcantry

- **WHEN** they scan the overview from the hero through the comparison
- **THEN** they can state what Arcantry maps, what remains independent, how changes become reviewable, and why adoption does not need to be all-or-nothing

#### Scenario: A developer composes an adoption strategy

- **WHEN** they choose a typical preset
- **THEN** the same three scope columns identify the chosen project scope and management boundary
- **AND** active and muted source entries distinguish private, selected-source, and shared-with-private states
- **AND** no per-source controls or custom state imply configuration that the overview does not apply

#### Scenario: A visitor prefers reduced motion

- **WHEN** reduced motion is requested
- **THEN** the preset selector remains manually operable without automatically changing preset state

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

### Requirement: The configurator builds an agent-ready adoption scenario

The documentation MUST provide a full-width prompt configurator using the established header and footer without the overview hero treatment. It MUST ask progressive, plain-language questions that distinguish new setup from an existing Arcantry setup, installed tool state, Git and GitHub use, shared or private audience, external control, desired knowledge sources, agent compatibility, and the user's preferred execution boundary. A choice that materially changes the next decision MUST reveal a relevant follow-up instead of assuming technical knowledge.

#### Scenario: A new user does not use GitHub

- **WHEN** the user says the project does not use GitHub
- **THEN** the configurator asks whether local Git would be useful without requiring a remote
- **AND** the generated prompt states that Arcantry can continue without Git

#### Scenario: A project already uses Arcantry

- **WHEN** the user chooses to change an existing setup
- **THEN** the configurator asks which boundaries, sources, guidance, or repair concerns should be reviewed
- **AND** the generated prompt begins with read-only inspection before recommending an update or source transition

### Requirement: Configurator choices are portable in the URL

The configurator MUST begin with no selected answer when its URL contains no valid configuration parameters. It MUST reveal each subsequent question only after the preceding applicable question has an answer. It MUST encode relevant answers as compact URL query parameters, MUST omit unanswered values, MUST restore the scenario from valid parameters after reload, and MUST discard invalid or inapplicable values. It MUST NOT use local storage, cookies, accounts, analytics, or server persistence for scenario answers.

When a visitor enters the empty configurator and makes at least one selection, the page MUST use a browser navigation warning before leaving while selections remain. When a visitor enters through a URL that already contains valid selections, the page MUST NOT add that warning, including after the visitor temporarily changes those selections.

#### Scenario: The configurator is opened without choices

- **WHEN** a visitor opens the configurator without valid configuration parameters
- **THEN** no answer is selected or represented in the URL
- **AND** only the first unanswered question is available

#### Scenario: A new setup is abandoned

- **WHEN** a visitor entered without choices, selected at least one answer, and then attempts to leave
- **THEN** the browser asks whether to leave the changed setup
- **AND** resetting every answer removes that warning

#### Scenario: A shared configuration URL is explored

- **WHEN** a visitor entered through a URL containing valid selections
- **THEN** the page does not warn when the visitor leaves
- **AND** temporary changes do not convert that visit into a new unsaved setup

#### Scenario: A configured URL is reopened

- **WHEN** a user opens a configurator URL containing valid answers
- **THEN** the same applicable controls are selected
- **AND** the generated prompt and completion state match those selections

### Requirement: Prompt changes remain understandable and safe

The configurator MUST present a compact live prompt preview beside the questions on wide screens and after the form on narrow screens. Its header MUST show the answered count with a gently animated gradient progress indicator before naming the preview, and MUST provide a compact circular-arrow reset control without a separate answers-left badge. Preview entries MUST use the darker content surface while the header uses the lighter sidebar surface, and their generated content MUST use Fira Code with monospace fallbacks to distinguish it as copyable material. Changing an answer MUST visually identify the affected prompt sections through transform or opacity-based motion, with a reduced-motion fallback. A full-width white copy action labelled "Copy generated prompt" with a sparkles icon MUST remain available at every stage, MUST provide visible hover feedback, and MUST gain the product gradient when all applicable questions are answered.

The generated prompt MUST tell an agent to install Arcantry through an official route when needed, inspect the project without mutation, explain whether adoption or a requested change makes sense, preserve existing project ownership, keep shared and private sources independent, obtain approval before material changes, validate approved work, and explain how to use the selected setup. It MUST NOT imply that repository initialization creates OpenSpec, changelog, todo.txt, or skill sources.

#### Scenario: A user completes a new adoption scenario

- **WHEN** every applicable answer is present
- **THEN** the copy action provides one prompt covering installation if needed, read-only inspection, recommendation, approved adoption, validation, and practical usage guidance
- **AND** the prompt distinguishes selected knowledge sources from repository initialization

#### Scenario: A user changes one answer

- **WHEN** an answer changes the installation, inspection, adoption, source, compatibility, or execution guidance
- **THEN** the corresponding preview sections receive a brief visual update
- **AND** the URL and copied prompt reflect the new answer

### Requirement: The configurator presents an audience-appropriate setup surface

The configurator MUST describe its outcome as ready-to-copy Arcantry setup instructions for the visitor's agent. It MUST NOT imply that an agent is present or operating inside the configurator. Default copy MUST NOT explain URL persistence, prompt construction, or other implementation details. Explanations MUST add a distinction that is not already evident from the available answers. Simple choices SHOULD include a short consequence when it helps the visitor compare them. The interface MUST present the continuation cue as muted information, MUST place reset with the preview controls and remove it from layout and focus order until a selection exists, and MUST avoid empty status rows, unnecessary separators, trailing borders, or shell spacing that resembles broken content.

Documentation scrollbars, including the compact instructions preview, MUST use the site's themed low-profile treatment instead of the platform's unstyled default while remaining scrollable by pointer, keyboard, and touch.

#### Scenario: A visitor has not answered the first question

- **WHEN** the configurator first appears
- **THEN** the heading explains that the result is a set of Arcantry setup instructions for the visitor's agent
- **AND** a centered muted continuation cue explains that answering it reveals the next question
- **AND** reset is absent from layout and focus order until a selection exists

### Requirement: The configurator uses the documentation application shell

On wide screens, the configurator MUST use a full-width three-region layout aligned with the global header and fitted to the viewport below it. The document itself MUST NOT scroll while that workspace fits; any overflow MUST remain within the relevant center or preview panel. The left region MUST provide links to the homepage and documentation, MUST list every configuration stage, MUST mark unavailable follow-ups, and MUST track the visible stage while scrolling. The center region MUST contain the setup introduction and progressive questions without an additional text column. The right region MUST keep the live instructions preview available in the position normally used for documentation context.

The configurator MUST reuse the documentation sidebar's link treatment and credit footer in the left region on wide screens instead of approximating their spacing, width, typography, or content. On narrow screens, the left navigation MUST collapse into a compact setup navigation control, the questions MUST remain the primary column, the preview MUST follow the questions, and the normal project footer MUST remain available.

The compact mobile brand symbol MUST remain readable in both light and dark themes. Header search, repository, and theme icons MUST use a moderate common visual size, and the search control MUST NOT display a keyboard-shortcut badge on any page. The full-width configurator header and workspace MUST use the same outer alignment instead of independently centered maximum widths.

#### Scenario: A visitor uses the configurator on a wide screen

- **WHEN** the viewport can display the documentation application shell
- **THEN** setup navigation, questions, and the instructions preview appear as left, center, and right regions
- **AND** the header aligns to the same full-width grid
- **AND** selecting an answer enables its applicable follow-up stage link

#### Scenario: A visitor uses the configurator below the documentation sidebar breakpoint

- **WHEN** the viewport is narrower than the documentation sidebar breakpoint
- **THEN** the stage navigation is available through a compact control above the questions
- **AND** the instructions preview follows the question flow
- **AND** the compact Arcantry symbol has sufficient contrast for the active theme

## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: The configurator builds an agent-ready adoption scenario

The documentation MUST provide a full-width prompt configurator using the established header and footer without the overview hero treatment. It MUST offer a bounded set of supported scenarios and ask progressive, plain-language questions that distinguish read-only evaluation, new adoption and inspection of an existing setup. A scenario MUST seed only facts inherent to that scenario. A choice that materially changes the next decision, conflicts with an earlier answer or leaves authority uncertain MUST reveal one relevant follow-up instead of assuming technical knowledge or permission.

#### Scenario: A new user does not use GitHub

- **WHEN** the user says the project does not use GitHub
- **THEN** the configurator asks whether local Git would be useful without requiring a remote
- **AND** the generated prompt states that Arcantry can continue without Git

#### Scenario: A visitor starts from evaluation

- **WHEN** the visitor selects a scenario whose first outcome is deciding whether Arcantry fits
- **THEN** the generated request begins with read-only inspection and a no-change recommendation boundary
- **AND** does not imply approval to adopt or modify the repository

#### Scenario: Scenario answers conflict

- **WHEN** one selected answer conflicts with the scenario or another material answer
- **THEN** the relevant stage asks a focused follow-up and marks the request incomplete
- **AND** the generator does not silently choose one answer

#### Scenario: A project already uses Arcantry

- **WHEN** the visitor selects a scenario for an existing setup
- **THEN** the configurator asks which boundaries, sources, guidance or repair concerns should be inspected
- **AND** the generated request requires current-state inspection before recommending an update or transition

### Requirement: Configurator choices are portable in the URL

The configurator MUST encode the selected scenario and relevant answers as compact URL query parameters, MUST omit unanswered and inapplicable values, MUST restore the normalized scenario after reload, and MUST discard unknown or conflicting values without replacing them with an implicit decision. It MUST NOT use local storage, cookies, accounts, analytics or server persistence for scenario answers.

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

#### Scenario: A configured scenario URL is reopened

- **WHEN** a user opens a URL containing one supported scenario and valid answers
- **THEN** the same applicable controls are selected
- **AND** the generated request, unresolved follow-ups and completion state match those selections

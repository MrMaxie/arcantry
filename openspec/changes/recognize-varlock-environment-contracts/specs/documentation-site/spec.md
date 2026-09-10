## MODIFIED Requirements

### Requirement: Documentation explains the unified adoption journey

The authored documentation MUST explain how shared and private TOML configuration, OpenSpec, changelog, todo.txt, optional environment contracts, repository guidance, portable skills, and the three catalog families fit together. It MUST distinguish shared project state from private workstation state, present each source as independently adoptable, describe `AGENTS.md` and `.agents` as universal surfaces rather than provider-owned files, and distinguish Arcantry environment-contract discovery and best-effort environment loading from Varlock-owned validation, value resolution and secret handling.

#### Scenario: A new adopter follows the documentation

- **WHEN** the reader opens the adoption guide
- **THEN** they can identify the minimum shared or private setup, the source of truth for each information layer, and the commands that verify the Arcantry-owned result
- **AND** Claude-specific files and Varlock integration are presented only as optional compatibility paths

#### Scenario: A reader uses Varlock with Arcantry

- **WHEN** the reader finds an `env-spec@1` environment contract in Arcantry output
- **THEN** the documentation explains that Arcantry observes the contract without reading or resolving environment values
- **AND** explains that only an environment-dependent Arcantry operation may attempt the existing Varlock setup
- **AND** routes dedicated validation, injection, encryption and leak scanning to Varlock guidance

#### Scenario: Varlock is unavailable or incompatible

- **WHEN** a reader runs an environment-dependent Arcantry operation without a usable Varlock setup
- **THEN** the documentation explains the dotenv-compatible project-root `.env` fallback and process-value precedence
- **AND** makes clear that optional Varlock detection, version mismatch or resolution failure does not block otherwise satisfiable work

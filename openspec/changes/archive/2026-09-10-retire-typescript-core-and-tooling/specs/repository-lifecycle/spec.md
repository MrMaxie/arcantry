## ADDED Requirements

### Requirement: Product and repository tooling have one Rust implementation authority

All Arcantry product logic and repository-maintained automation outside `apps/docs` MUST be implemented in Rust. This includes project discovery, configuration, source handling, todo operations, skill and catalog processing, release planning and validation, generated artifact production, package assembly, smoke orchestration and publication preparation. TypeScript MAY remain inside `apps/docs` for the Astro documentation application. A package-manager launcher MAY use minimal JavaScript only to locate and execute the native binary and MUST NOT implement Arcantry policy, parsing, validation, planning, generation or mutation behavior.

#### Scenario: A contributor runs repository automation

- **WHEN** a contributor invokes an Arcantry generation, validation, packaging, smoke or release-preparation task
- **THEN** the repository entrypoint dispatches to Rust-owned behavior
- **AND** any invoked Node.js ecosystem tool remains an external boundary rather than an Arcantry implementation layer

#### Scenario: TypeScript is added outside documentation

- **WHEN** repository validation inventories an authored TypeScript file outside `apps/docs`
- **THEN** validation fails unless the file is still present in the reviewed shrinking migration inventory
- **AND** the final accepted inventory permits no TypeScript outside `apps/docs`

#### Scenario: The documentation application is built

- **WHEN** contributors build or validate `apps/docs`
- **THEN** its Astro and TypeScript implementation remains supported
- **AND** it consumes generated product data without becoming the authority for Arcantry behavior

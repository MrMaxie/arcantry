## ADDED Requirements

### Requirement: Public catalog is an audience-facing projection

The generated public catalog MUST group skills by the work a developer wants to complete. Each catalog item MUST use a readable display name, a short outcome, release state, and a link to its detail page. Raw tags MAY support search and filtering but MUST NOT occupy a primary catalog column or card region.

#### Scenario: A developer scans the catalog

- **WHEN** they compare skills in one goal group
- **THEN** names remain readable, summaries stay short, and internal metadata does not compete with the choice

### Requirement: Skill versions follow the Arcantry release

A generated skill page MUST derive its release state from archived OpenSpec change components and Arcantry release manifests. A released skill MUST show the first Arcantry version that included it. A skill without a released component MUST show `Unreleased`. The catalog and skill metadata MUST NOT define an independent skill version.

#### Scenario: A skill has no released component

- **WHEN** documentation generation runs
- **THEN** its page shows `Unreleased` instead of borrowing the current package version

# skill-catalog Specification

## Purpose
TBD - created by archiving change unify-arcantry-capabilities. Update Purpose after archive.

## Requirements

### Requirement: Catalog projections derive from canonical skill packages

The public skill catalog MUST be generated deterministically from validated skill metadata. Generated catalog, plugin, and documentation projections MUST NOT become competing authored sources.

#### Scenario: Canonical skill metadata changes

- **WHEN** generation runs after an accepted metadata change
- **THEN** every affected projection reflects the same name, description, scenarios, and dependency contract
- **AND** product-level plugin and CLI versions remain aligned with the latest Arcantry release manifest

### Requirement: Skills support individual and complete distribution

Users MUST be able to inspect and link one catalog skill through the CLI. Users MUST also be able to install the complete catalog through the Arcantry Codex plugin.

#### Scenario: A user chooses the full collection

- **WHEN** the user installs the Arcantry Codex plugin
- **THEN** the plugin exposes every validated public skill in the current catalog version

### Requirement: Distribution excludes private state

Catalog and plugin outputs MUST use an allowlist of public package fields and files. They MUST exclude `.local/`, credentials, logs, caches, backups, workstation paths, and generated diagnostics.

#### Scenario: A distribution package is verified

- **WHEN** package-content validation runs
- **THEN** the build fails if any private or non-allowlisted artifact is present

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

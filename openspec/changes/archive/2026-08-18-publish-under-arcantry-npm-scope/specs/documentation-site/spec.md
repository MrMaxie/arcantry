## ADDED Requirements

### Requirement: Public package commands use the canonical npm identity

Documentation and interactive copy surfaces MUST derive or validate package launcher commands against the canonical package manifest name.

#### Scenario: The npm package identity changes

- **WHEN** documentation generation and checks run
- **THEN** npm, pnpm and supported launcher examples use `@arcantry/arcantry`
- **AND** stale package scopes fail validation instead of remaining in public copy

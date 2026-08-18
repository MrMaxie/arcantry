## ADDED Requirements

### Requirement: The public npm package uses the global Arcantry name

The existing combined CLI and library package MUST use the unscoped public name `arcantry`, expose the `arcantry` binary and retain its declared public subpath exports. The private workspace root MUST use a distinct non-publishable name so package-manager filters resolve the public package unambiguously. The package MAY be governed by the Arcantry npm organization without using its scope in the consumer-facing name.

#### Scenario: A consumer runs the CLI without installing it

- **WHEN** the consumer invokes `npx arcantry`, `pnpm dlx arcantry` or an equivalent package launcher
- **THEN** npm resolves the public package and runs the `arcantry` binary

#### Scenario: A consumer imports a public module

- **WHEN** the consumer imports a declared `arcantry` subpath from the packed package
- **THEN** the import resolves to its shipped JavaScript and type declarations

#### Scenario: Repository package references are validated

- **WHEN** package, workspace, test and public documentation surfaces are checked
- **THEN** they resolve the canonical `arcantry` package identity
- **AND** no shipped or documented command refers to the previous scoped identities

## REMOVED Requirements

### Requirement: Public npm packages use the Arcantry organization scope

**Reason:** The organization scope makes the single combined package repeat the product name without helping consumers distinguish packages.

**Migration:** Replace `@arcantry/arcantry` launcher commands and imports with `arcantry` before the first public publication.

## MODIFIED Requirements

### Requirement: The first package publication is an explicit bootstrap

Because trusted publishing configuration requires an existing npm package, the first public `arcantry` version MUST be published by an authorized maintainer from the exact verified archive of a sealed release using 2FA. The package MAY then be assigned to the Arcantry npm organization. The trusted publisher MUST be configured before any later version is published by CI/CD.

#### Scenario: The package does not exist in npm

- **WHEN** the first sealed archive is ready and separately authorized for publication
- **THEN** a maintainer publishes that archive once with 2FA
- **AND** assigns organization governance when required
- **AND** configures trusted publishing for later releases

#### Scenario: Bootstrap has not been completed

- **WHEN** an automated publication is attempted before the package and trusted-publisher relationship exist
- **THEN** the workflow fails without creating or exposing a reusable npm write credential

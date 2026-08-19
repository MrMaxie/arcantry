## MODIFIED Requirements

### Requirement: Skills support individual and complete distribution

Users MUST be able to inspect and link one catalog skill through the CLI into a standard user or repository Agent Skills directory. The public catalog MUST remain compatible with canonical checkouts, the installed JavaScript package, the native executable's embedded catalog, manual copying, symbolic linking and independent Agent Skills installers without requiring those tools as runtime dependencies. Before creating a link from embedded content, the native CLI MUST materialize and verify an exact versioned public catalog in the user's operating-system-standard data directory.

#### Scenario: A user chooses one catalog skill

- **WHEN** the user selects a skill from a canonical checkout or installed JavaScript package
- **THEN** they can link it into user or repository scope with the Arcantry CLI
- **AND** the installed package remains an independently readable skill directory

#### Scenario: A user chooses one skill from the standalone executable

- **WHEN** the user links an embedded skill without an explicit catalog root
- **THEN** Arcantry verifies or atomically materializes the exact release catalog
- **AND** the link targets that durable versioned skill directory
- **AND** repeating the command does not create a duplicate installation

#### Scenario: A user unlinks an embedded skill

- **WHEN** the target is an exact link to the selected skill in the verified materialization
- **THEN** Arcantry removes only that link
- **AND** preserves the versioned catalog and unrelated user content

#### Scenario: A user chooses the full collection

- **WHEN** the user installs or links the complete canonical catalog with a compatible Agent Skills workflow
- **THEN** every validated public skill in the current catalog is available without a provider-specific runtime dependency

## MODIFIED Requirements

### Requirement: Catalog projections derive from canonical skill packages

The public skill catalog MUST be generated deterministically from validated skill metadata. Canonical metadata MUST provide a distinct provider-neutral summary for every skill. Generated catalog, plugin, and documentation projections MUST NOT become competing authored sources.

#### Scenario: Canonical skill metadata changes

- **WHEN** generation runs after an accepted metadata change
- **THEN** every affected projection reflects the same name, distinct summary, routing description, scenarios, and dependency contract
- **AND** product-level plugin and CLI versions remain aligned with the latest Arcantry release manifest

### Requirement: Skills support individual and complete distribution

Users MUST be able to inspect and link one catalog skill through the CLI into the standard user or repository `.agents/skills` directory. Adoption guidance MUST recommend user scope first and MUST NOT install, update, or copy a skill without explicit authorization for that action and scope. A repository-private package or override MUST remain an explicit fallback when the required capability is missing or the user-wide skill is unsuitable. Codex MAY consume the standard Agent Skills surface directly. Claude Code MAY use an explicitly requested compatibility link into `.claude/skills`. The public catalog MUST remain compatible with canonical checkouts, the installed JavaScript package, the native executable's embedded catalog, manual copying, symbolic linking and independent Agent Skills installers without requiring those tools as runtime dependencies. Before creating a link from embedded content, the native CLI MUST materialize and verify an exact versioned public catalog in the user's operating-system-standard data directory.

#### Scenario: A user chooses one catalog skill

- **WHEN** the user selects a skill from a canonical checkout or installed package
- **THEN** they can link it into a standard user or repository scope with the Arcantry CLI
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

#### Scenario: Adoption needs an unavailable or unsuitable skill

- **WHEN** a required capability is missing or its user-wide skill is unsuitable
- **THEN** adoption recommends a user-wide installation or update first
- **AND** offers a repository-private package or override only as an explicitly selected fallback

#### Scenario: A user chooses the full collection

- **WHEN** the user installs or links the complete canonical catalog with a compatible Agent Skills workflow
- **THEN** every validated public skill in the current catalog is available without a provider-specific runtime dependency

#### Scenario: A user requests Claude compatibility

- **WHEN** the user adds the Claude compatibility option while linking a skill
- **THEN** both destinations resolve to the same canonical package
- **AND** no provider-specific copy of `SKILL.md` is created

### Requirement: Public catalog is an audience-facing projection

The generated public catalog MUST group every skill into exactly one of `self-improvement`, `repo-safely`, or `content-safely`. Each catalog item MUST use a readable display name, a distinct short outcome, release state, and a link to its detail page. Raw tags MAY support search and filtering but MUST NOT occupy a primary catalog column or card region. Generated navigation MUST include every public skill exactly once.

#### Scenario: A developer scans the catalog

- **WHEN** they compare skills in one family
- **THEN** names remain readable, summaries remain distinct and short, and internal metadata does not compete with the choice
- **AND** every skill appears in exactly one family and once in generated navigation

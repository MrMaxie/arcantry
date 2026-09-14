# skill-catalog Specification

## Purpose
Define the canonical skill catalog, its generated public projections and distribution boundaries.
## Requirements
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

### Requirement: Distribution excludes private state

Catalog and plugin outputs MUST use an allowlist of public package fields and files. They MUST exclude `.local/`, credentials, logs, caches, backups, workstation paths, and generated diagnostics.

#### Scenario: A distribution package is verified

- **WHEN** package-content validation runs
- **THEN** the build fails if any private or non-allowlisted artifact is present

### Requirement: Public catalog is an audience-facing projection

The generated public catalog MUST group every skill into exactly one supported family, including `self-improvement`, `repo-safely`, `content-safely`, and `code-quality`. Each catalog item MUST use a readable display name, a distinct short outcome, independent skill version, role, and a link to its detail page. Raw tags MAY support search and filtering but MUST NOT occupy a primary catalog column or card region. Generated navigation MUST include every public skill exactly once.

#### Scenario: A developer scans the catalog

- **WHEN** they compare skills in one family
- **THEN** names remain readable, summaries remain distinct and short, and version and role remain secondary to the choice
- **AND** every skill appears in exactly one family and once in generated navigation

### Requirement: Canonical catalog metadata is schema strict

Catalog and skill metadata validation MUST require the canonical schema references, a `family` value from the supported family enum, supported field allowlists, valid lowercase identifiers, unique tags, and documented audience-facing text lengths. Source tooling and the distributed package MUST enforce the same contract.

#### Scenario: Unsupported metadata is introduced

- **WHEN** catalog or skill metadata contains an unknown field, invalid family or identifier, incorrect schema reference, or out-of-range text
- **THEN** repository validation and packaged runtime validation reject the metadata before generating or exposing projections

### Requirement: Host plugin projections preserve canonical product identity

Every supported host plugin manifest MUST project the same canonical Arcantry name, product version, concise description, author, homepage, repository and license. Each host adapter MUST use an explicit allowlist for host-specific presentation, capability and schema fields and MUST NOT copy unsupported fields from another host merely to make the manifests structurally identical. Repository and package validation MUST inspect every supported host manifest.

#### Scenario: Canonical identity changes

- **WHEN** one canonical product identity field changes through an accepted change
- **THEN** every supported host manifest and packaged projection is updated together
- **AND** each manifest remains valid for its own host contract

#### Scenario: A host exposes additional presentation fields

- **WHEN** one host supports branding or interface metadata that another host does not support
- **THEN** the additional fields remain in that host's projection only
- **AND** shared identity continues to agree across both manifests

### Requirement: Public skills have independent identity and role

Every public skill MUST declare its own semantic version and one role from `primary`, `supporting` or `advanced`. The version MUST remain independent from the Arcantry product and CLI version. Generated catalog projections MUST expose both fields without changing skill invocation policy. Every projected package MUST include a deterministic SHA-256 manifest and a bounded history derived from accepted OpenSpec outcomes. Manifest hashing MUST normalize CRLF to LF for UTF-8 text resources and MUST preserve exact bytes for binary resources so the same package has one identity across supported checkout platforms.

#### Scenario: A skill changes while Arcantry remains at the same version

- **WHEN** an accepted skill revision changes package content without an Arcantry release
- **THEN** its exact revision and content digest change independently
- **AND** the Arcantry product and CLI remain at version `1.0.0`

#### Scenario: A text package is checked out on different platforms

- **WHEN** Git materializes equivalent UTF-8 skill resources with LF or CRLF line endings
- **THEN** both checkouts produce the same file hashes and package digest
- **AND** non-UTF-8 resources continue to use their exact bytes

### Requirement: One skill can be compared and updated safely

The native CLI MUST report local management state, version, exact revision, content digest and local modification state for a selected installed skill. Remote checking MUST be explicit and MUST compare against the official Arcantry GitHub `master` unless an explicit local catalog is selected. Updating MUST first produce a serializable plan pinned to one exact source revision, package digest and installed preimage. Applying that plan MUST validate the complete package and unchanged preimage before atomically switching only the selected skill and recording its provenance outside skill discovery roots.

#### Scenario: A newer canonical revision is available

- **WHEN** the user checks one managed skill and its official package digest differs
- **THEN** status reports the exact installed and available identities without writing
- **AND** update planning describes the exact package and target changes

#### Scenario: Installed content changed after preview

- **WHEN** the installed package, managed target or source package no longer matches the saved plan
- **THEN** apply fails before replacing any link or receipt
- **AND** the previous installation remains usable

#### Scenario: Other skills share the same CLI installation

- **WHEN** one selected skill update is applied
- **THEN** its immutable snapshot and managed links change
- **AND** snapshots, receipts and links for every other skill remain unchanged

### Requirement: Legacy and private packages are not silently adopted

Legacy embedded links, source-checkout links and ordinary skill directories without an ownership receipt MUST remain readable and inspectable. The CLI MUST identify them as development or unmanaged and MUST NOT overwrite, migrate or claim them automatically. Private repository skills MUST never be matched to or uploaded to the official public source.

#### Scenario: A user checks an unowned or modified package

- **WHEN** no receipt proves ownership or installed bytes differ from its receipt
- **THEN** status identifies the state and update apply refuses an implicit replacement
- **AND** the existing files and links remain unchanged

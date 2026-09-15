# documentation-site Specification

## Purpose
Define the documentation content contract for developers adopting and verifying Arcantry.

## Requirements

### Requirement: Documentation explains the unified adoption journey

The authored documentation MUST explain how shared and private TOML configuration, OpenSpec, changelog, todo.txt, repository guidance, portable skills, and the three catalog families fit together. It MUST distinguish shared project state from private workstation state, present each source as independently adoptable, and describe `AGENTS.md` and `.agents` as universal surfaces rather than provider-owned files.

#### Scenario: A new adopter follows the documentation

- **WHEN** the reader opens the adoption guide
- **THEN** they can identify the minimum shared or private setup, the source of truth for each information layer, and the commands that verify the result
- **AND** Claude-specific files are presented only as optional compatibility adapters

### Requirement: Cross-cutting guidance is authored and skill detail is generated

Adoption, repository workflow, CLI, and skills overview pages MUST be authored for their user journeys. Per-skill detail pages and skill navigation MUST be generated from canonical skill packages. Cards and page leads MUST use canonical catalog summaries, while routing descriptions MUST remain visibly distinct from those summaries.

#### Scenario: Skill metadata is updated

- **WHEN** documentation generation runs
- **THEN** the skill detail, catalog entry and navigation change without requiring a second authored edit
- **AND** each public skill remains present exactly once

### Requirement: Arcantry documents its own conformance

The contributor documentation MUST identify the repository verification surface and accurately distinguish unit tests, black-box CLI contract tests, optional coverage diagnostics, disposable Linux system tests and release-target execution. It MUST NOT describe an independent native contract suite as a comparison with a retired implementation or describe missing branch data as branch coverage.

#### Scenario: A contributor checks Arcantry itself

- **WHEN** the contributor follows the Contributing to Arcantry section
- **THEN** they can identify the repository verification surface and Arcantry-specific lifecycle rules

#### Scenario: A contributor selects a verification command

- **WHEN** the contributor follows the command reference
- **THEN** they can identify which boundary the command executes and whether failure blocks acceptance
- **AND** the documented claim matches the command's current implementation

### Requirement: Documentation links and presentation remain stable

The sidebar MUST use grouped Start, Guides, Concepts, Reference, and Contributing to Arcantry sections. Internal links and fragments from the overview and sidebar MUST resolve in the built site. Moving or generating documentation source files MUST preserve established public routes. Content MUST describe the current Arcantry contract directly and MUST preserve the approved Starlight shell, typography, animation, components, and visual composition except for copy, information order, commands, and links required by the product contract.

#### Scenario: A reader navigates the documentation

- **WHEN** they follow an overview action or sidebar item
- **THEN** the intended existing route, page or section exists
- **AND** the established documentation shell remains visually unchanged

### Requirement: Distribution guidance remains provider-neutral

Documentation MUST recommend standard `.agents/skills` user and repository locations, the Arcantry local linker, and compatible manual or independent installer workflows. It MAY explain that Codex consumes the standard surface directly and MUST present Claude paths as optional compatibility adapters. It MUST NOT require a paid provider, remote issue tracker, host-specific plugin, or provider-branded directory to use the catalog.

#### Scenario: A developer adopts one skill

- **WHEN** they follow the skill installation guidance
- **THEN** they can choose a user-scoped or repository-scoped standard Agent Skills location
- **AND** they can explicitly add Claude compatibility without creating a second skill package

### Requirement: Public package commands use the canonical npm identity

Documentation and interactive copy surfaces MUST derive or validate the `npx` and `npm` commands against the canonical `arcantry` package manifest name. Native download guidance MUST derive or validate target names and the versioned GitHub Release URL against the declared release identity rather than presenting platform package names as end-user commands.

#### Scenario: The npm package identity changes

- **WHEN** documentation generation and checks run
- **THEN** `npx` and `npm` examples use `arcantry`
- **AND** native archive examples use only declared release targets
- **AND** stale package scopes or target names fail validation instead of remaining in public copy

### Requirement: Installation guidance distinguishes native and npm use

The documentation MUST present direct GitHub Release archives, checksum-verifying sh and PowerShell installers, and the `arcantry` npm package as the supported public distribution paths. The public installation picker MUST present `npx`, `npm`, PowerShell and `sh` in that order, followed by a `Download` link to the versioned GitHub Release. It MUST NOT present pnpm or Nub as installation-picker choices or accompanying public installation guidance. The documentation MUST list the supported operating-system and architecture matrix, explain that the Linux archives support both glibc and musl systems, and describe installation from the user's task without exposing launcher, optional-package, migration-oracle or build-pipeline details. It MUST NOT present unsupported distribution channels, targets, signing or automatic updates as delivered behavior.

#### Scenario: A user chooses an installation path

- **WHEN** the reader opens the CLI installation guidance
- **THEN** the choices appear as `npx`, `npm`, PowerShell, `sh` and `Download`
- **AND** `Download` is a link to the versioned GitHub Release rather than a copyable command tab
- **AND** pnpm and Nub are absent from the installation choices and accompanying installation copy
- **AND** they can select the archive matching Windows, macOS or Linux on x64 or ARM64
- **AND** they can run the same `arcantry` command after installation

#### Scenario: A user verifies a native download

- **WHEN** the reader installs from a GitHub Release
- **THEN** the documentation identifies the matching archive and `SHA256SUMS` verification path
- **AND** the provided sh and PowerShell installers verify the selected archive against that checksum manifest
- **AND** does not imply that an unsigned or unsupported distribution channel is available

### Requirement: Documentation projections are build outputs

Per-skill pages, generated skill navigation and public schema copies MUST be derived from their canonical repository sources before documentation development, checking and building. These documentation-only projections MUST remain untracked in Git, while the deployed site MUST include them at their established public routes.

#### Scenario: Documentation builds from a clean checkout

- **WHEN** a contributor or CI builds documentation without generated documentation files on disk
- **THEN** generation runs before Astro consumes the content
- **AND** every canonical skill page, generated navigation item and public schema is present in the build

#### Scenario: Generated documentation is inspected in Git

- **WHEN** generation completes against unchanged canonical inputs
- **THEN** documentation-only output remains ignored and untracked
- **AND** tracked plugin manifests remain independently drift-checked

### Requirement: CLI reference tables preserve literal command alternatives

Every rendered Markdown and MDX table MUST preserve the header's column count in each body row. CLI command tables MUST display option alternatives such as `shared|private` as literal command text. Verification MUST cover the complete documentation surface rather than selected examples.

#### Scenario: An option uses a vertical bar

- **WHEN** a command table contains alternative option values separated by `|`
- **THEN** the rendered row keeps the alternative inside one command cell
- **AND** no formatting marker or unintended column is exposed

### Requirement: Public CLI trust claims identify executable evidence

Claims about read-only behavior, network behavior, repository writes, removal boundaries, rollback and private data MUST be no broader than named executable evidence and an accepted OpenSpec requirement. Unsupported or incompletely executed platforms MUST be described as contract targets rather than current execution evidence. Documentation verification MUST validate the complete provenance chain rather than checking only that a marker is present.

#### Scenario: A trust claim loses executable evidence

- **WHEN** its accepted requirement, documentation reference or executable scenario is removed, renamed or rebound
- **THEN** documentation verification fails until the claim is narrowed or the complete evidence chain is restored

### Requirement: MVP setup follows the visitor outcome

The configurator MUST distinguish evaluation, new adoption and existing setup repair. It MUST allow source-based work without CLI installation, preserve private boundaries, restore applicable URL choices and require explicit approval only for a requested later apply. The layout MUST reflow without horizontal overflow on phone and desktop widths. Interactive examples MUST explain the context, next and explain workflow without claiming to execute commands in the browser.

#### Scenario: CLI is missing

- **WHEN** a visitor selects source-based work
- **THEN** the generated request reads project instructions, configuration, change tasks and templates directly
- **AND** does not block on installation

### Requirement: Public trust claims have audience-visible evidence

Every material public claim about data flow, network activity, permissions, repository writes, rollback, private state, skill trust, package integrity or vulnerability handling MUST identify an accepted requirement, evidence at least as broad as the claim and the boundary Arcantry actually controls. Evidence required to evaluate a public claim MUST be available to that audience. When complete public evidence is unavailable, documentation MUST narrow the claim and state the remaining responsibility instead of exposing private diagnostics or presenting an aspiration as a guarantee.

#### Scenario: A public guarantee exceeds its evidence

- **WHEN** the claim covers behavior, platforms or actors that its linked evidence does not cover
- **THEN** documentation validation fails until the evidence expands or the wording narrows
- **AND** a responsibility boundary is not presented as verified Arcantry behavior

#### Scenario: Evidence contains private diagnostic data

- **WHEN** supporting evidence cannot be published safely to the claim's audience
- **THEN** the public claim is limited to evidence that audience can inspect
- **AND** private diagnostics are not copied into the public trust surface

### Requirement: Trust evidence has one non-duplicative owner

The public trust inventory MUST reference the CLI provenance ledger for CLI-specific claims and MUST NOT restate or independently rebind its executable evidence. Non-CLI trust claims MUST have their own stable owners and evidence references.

#### Scenario: A CLI trust claim appears in the public inventory

- **WHEN** the inventory includes a claim already owned by the CLI provenance contract
- **THEN** it references that existing claim and evidence identity
- **AND** validation rejects a competing duplicate mapping

### Requirement: Adoption guidance explains ownership exit consequences

Every adoption path MUST make removal, rollback and permanent detachment consequences discoverable before apply. Guidance MUST distinguish verified Arcantry-owned metadata, user-authored project knowledge, managed links, copied or reimplemented detached assets and independently maintained project behavior. It MUST identify which updates, compatibility promises, validation, distribution and support are retained or forfeited.

#### Scenario: A team compares managed and detached ownership

- **WHEN** a fixed repository-specific implementation may fit better than managed Arcantry
- **THEN** guidance explains the maintenance, security, migration and compatibility obligations transferred to the project
- **AND** explains that Arcantry's portability, safe upgrades, cross-project consistency, distribution and supported evolution remain separate product value

#### Scenario: A user reads removal guidance

- **WHEN** the user intends only to stop Arcantry management
- **THEN** guidance does not describe removal as ownership transfer or independence proof
- **AND** identifies any preserved user-authored content separately

### Requirement: Adoption guidance supports an informed ownership decision

Adoption guidance MUST identify the work required to evaluate and adopt the selected scope, the ongoing ownership retained by the project, and the observable behavior available for the project's own evaluation. The project MUST retain authority over its evaluation period, success signals and stopping or removal conditions. Claims about effort or value MUST be grounded in current product behavior and representative project workflows and MUST NOT present unsupported return-on-investment estimates.

#### Scenario: A team evaluates a pilot

- **WHEN** a product or engineering leader compares Arcantry's expected value with organizational overhead
- **THEN** the guidance identifies available scopes, responsibilities and observable product behavior
- **AND** leaves evaluation criteria and the stopping decision to that team

### Requirement: Adjacent engineering practices retain their authority

Documentation MUST explain that ADRs and RFCs own decision rationale, issue trackers own delivery coordination, project documentation owns durable usage knowledge and release practices own delivered history. Arcantry MUST describe supported connections and boundaries without turning those systems into Arcantry-owned sources or duplicating their content by default.

#### Scenario: A mature repository compares integration

- **WHEN** an evaluator already has credible decision, planning, documentation and release practices
- **THEN** the guidance shows what remains authoritative, what Arcantry may connect and what it does not manage
- **AND** identifies when preserving the existing system without Arcantry is the lower-cost choice

### Requirement: Project-work adoption does not imply delivery integration

Documentation MUST distinguish shared project-work configuration from integrating Arcantry into product runtime, build, CI or publication workflows. Guidance and generated adoption requests MUST NOT infer those delivery-toolchain changes from shared scope and MUST require an explicit user selection or request before including them.

#### Scenario: A project adopts shared configuration only

- **WHEN** a user selects shared project-work configuration without selecting a delivery-toolchain integration
- **THEN** adoption guidance limits the requested changes to the selected project-work scope
- **AND** does not request changes to product runtime, build, CI or publication workflows

### Requirement: Documentation has one canonical public origin

The documentation site, canonical and social metadata, sitemaps, public schema identities, runtime schema defaults, package metadata and plugin metadata MUST use `https://arcantry.dev/` as the public Arcantry origin. The site MUST be published from the domain root through the repository's GitHub Pages workflow. Public documentation links MUST NOT depend on the previous `/arcantry/` repository base path or identify the GitHub Pages origin as canonical. The independent `https://maxie.dev` author identity MAY remain linked as author information.

#### Scenario: A public Arcantry URL is generated

- **WHEN** documentation and package projections are generated from a clean checkout
- **THEN** canonical metadata, Open Graph URLs, sitemap entries, schema locations and product homepage links use `https://arcantry.dev/`
- **AND** internal documentation links resolve from the domain root
- **AND** generated-output verification rejects the previous public origins and repository base path

### Requirement: Hashed documentation assets use a dedicated cache path

Astro-generated JavaScript, CSS, optimized images, fonts and other content-hashed documentation assets MUST be emitted under `/static/`. Requests under `/static/` MUST be served through Cloudflare with an edge and browser cache TTL of 15,552,000 seconds. HTML and non-hashed public files MUST NOT inherit this long-lived cache policy.

#### Scenario: A visitor requests a generated asset twice

- **WHEN** a deployed content-hashed asset under `/static/` is requested through `arcantry.dev`
- **THEN** the response advertises a 15,552,000-second browser cache lifetime
- **AND** a repeated request can be served as a Cloudflare cache hit
- **AND** an HTML response remains outside the long-lived asset rule

### Requirement: The custom domain preserves secure canonical routing

The GitHub Pages site MUST assign `arcantry.dev` as its custom domain, serve it over enforced HTTPS and accept both the apex and `www` DNS variants. Cloudflare MUST proxy the web records only after GitHub Pages has provisioned the custom-domain certificate, use strict TLS to the origin and redirect HTTP to HTTPS. The `www` variant MUST redirect to the canonical apex origin.

#### Scenario: A visitor uses a non-canonical entry point

- **WHEN** they request HTTP or the `www` hostname
- **THEN** they reach the corresponding HTTPS route on `https://arcantry.dev/`
- **AND** the delivered page identifies the apex origin as canonical

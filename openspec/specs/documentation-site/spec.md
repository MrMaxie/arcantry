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

The contributor documentation MUST state that Arcantry validates its own repository through documented project commands and contracts. It MUST NOT present Arcantry's mandatory OpenSpec change and release lifecycle as a universal requirement for projects that only use Arcantry.

#### Scenario: A contributor checks Arcantry itself

- **WHEN** the contributor follows the Contributing to Arcantry section
- **THEN** they can identify the repository verification surface and Arcantry-specific lifecycle rules

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

Documentation and interactive copy surfaces MUST derive or validate package launcher commands against the canonical `arcantry` package manifest name. Native download guidance MUST derive or validate target names against the declared release matrix rather than presenting platform package names as end-user commands.

#### Scenario: The npm package identity changes

- **WHEN** documentation generation and checks run
- **THEN** npm, pnpm and supported launcher examples use `arcantry`
- **AND** native archive examples use only declared release targets
- **AND** stale package scopes or target names fail validation instead of remaining in public copy

### Requirement: Installation guidance distinguishes native and npm use

The documentation MUST present direct archives, checksum-verifying sh and PowerShell installers, and the `arcantry` npm package as supported installation paths. It MUST list the supported operating-system and architecture matrix, explain that the Linux archives support both glibc and musl systems, and retain npm/npx, pnpm, and Nub package-runner guidance. It MUST describe installation from the user's task without exposing launcher, optional-package, migration-oracle or build-pipeline details. It MUST NOT document Homebrew, Scoop, unsupported targets, signing or automatic updates as delivered behavior.

#### Scenario: A user chooses an installation path

- **WHEN** the reader opens the CLI installation guidance
- **THEN** they can select the archive matching Windows, macOS or Linux on x64 or ARM64, or use the matching sh or PowerShell installer
- **AND** can run the same `arcantry` command after installation

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

Claims about read-only behavior, network behavior, repository writes, removal boundaries, rollback and private data MUST be no broader than named executable evidence. Unsupported or incompletely executed platforms MUST be described as contract targets rather than current execution evidence.

#### Scenario: A trust claim loses executable evidence

- **WHEN** its mapped executable scenario is removed or renamed
- **THEN** documentation verification fails until the claim is narrowed or evidence is restored

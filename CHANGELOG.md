# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-09-14

### Added

<!-- openspec: add-agent-prompt-configurator -->
#### Configure an Arcantry setup prompt

The documentation configurator guides visitors through installation, adoption scope, project sources and agent compatibility choices, then produces a copyable setup prompt whose state can be restored from the page URL.

<!-- openspec: add-code-quality-skills -->
#### Add focused code-quality workflows

Arcantry separates evidence-backed code assessment, maintainable structural design and authorized behavior-preserving refactoring into three focused public skills.

<!-- openspec: add-design-coherent-gui-skill -->
#### Design coherent graphical interfaces

Arcantry now guides graphical interface work around audience-fit information density, purposeful spatial composition, established component contracts, and rendered consistency.

<!-- openspec: add-deterministic-context-discovery -->
#### Inspect complete Arcantry repository context deterministically

Repository inspection exposes concise, detailed and machine-readable Arcantry context without repeated platform-specific scans.

<!-- openspec: define-managed-and-detached-adoption -->
#### Transfer selected capabilities into independent project ownership

Arcantry distinguishes managed removal from reviewed one-way detachment and verifies that detached outputs remain useful without an Arcantry dependency or update channel.

<!-- openspec: deliver-practical-mvp -->
#### Answer project context through the CLI and MCP

Arcantry exposes shared `context`, `next` and `explain` behavior through the native CLI and a read-only MCP server so agents can resume work from discovered project sources without inferring approval.

<!-- openspec: deliver-practical-mvp -->
#### Adapt release workflows to project conventions

Projects can choose SemVer, monotonically increasing integer or calendar release identifiers and can render managed changelogs from a project template while preserving existing history.

<!-- openspec: expose-generic-release-cli -->
#### Manage local release stories from adopted projects

Adopted projects can establish a release baseline, plan and cut SemVer manifests, render a managed changelog and check release consistency through a configuration-driven local CLI that never publishes implicitly.

<!-- openspec: extend-configurator-adoption-scenarios -->
#### Start adoption guidance from a supported scenario

The configurator turns representative project scenarios into portable, approval-aware agent requests and asks when material choices conflict.

<!-- openspec: model-audience-based-release-projection -->
#### Project release work into audience-appropriate changelog stories

Arcantry separates audience, observable impact and inclusion policy, allowing traceable consolidation and intentional omission without losing manifest or SemVer ownership.

<!-- openspec: ship-native-rust-cli -->
#### Run Arcantry as a native cross-platform CLI

Arcantry provides verified native executables for Windows, macOS and Linux on x64 and ARM64 while preserving package-runner commands for npm consumers.

<!-- openspec: stabilize-arcantry-1-0 -->
#### Coordinate project knowledge and agent workflows with Arcantry

Arcantry provides local-first project configuration, safe repository adoption, a portable catalog of focused agent skills, and one documented workflow for maintaining project intent, release meaning, task intake, verification, and audience-safe content.

<!-- openspec: support-composable-release-units -->
#### Model independent and composed release units

Projects can manage one release story, several independently versioned units or a composed product whose unit manifests pin exact direct dependencies, without turning child releases into implicit parent version changes.

<!-- openspec: support-multiple-agent-hosts -->
#### Use universal Agent Skills with Claude compatibility

Arcantry links individual skills through the universal `.agents` directory, lets Codex consume that standard directly, and can add Claude Code compatibility aliases to the same canonical packages.

<!-- openspec: support-multiple-release-outcomes -->
#### Represent distinct consumer outcomes within one release change

Arcantry release artifacts can keep several consumer outcomes under their truthful changelog categories while preserving one OpenSpec change as the SemVer, manifest, visibility, component, and provenance boundary.

<!-- openspec: version-and-update-skills -->
#### Version and update individual skills

Arcantry can identify, compare and safely update one public skill from an exact canonical revision without replacing other installed skills or requiring a CLI release.

### Changed

<!-- openspec: align-host-plugin-identity -->
#### Keep plugin identity aligned across supported hosts

Arcantry presents one canonical product identity through platform-appropriate Codex and Claude plugin manifests.

<!-- openspec: deliver-practical-mvp -->
#### Keep planned project mutations recoverable

Saved plans retain source hashes, refuse stale inputs and preserve ambiguous interrupted state for explicit recovery instead of overwriting project content.

<!-- openspec: establish-arcantry-dev-public-domain -->
#### Publish Arcantry from its own public domain

Arcantry documentation, metadata and public schemas use `arcantry.dev`, with long-lived caching limited to content-hashed site assets.

<!-- openspec: establish-executable-cli-contract -->
#### Verify the native CLI against its public contract

The native CLI now proves its documented command surface and repository effects through independent executable contract cases instead of treating the retired TypeScript CLI as the expected result. Repository adoption, project plans and multi-target skill links now restore their complete pre-command filesystem state when a staged mutation fails.

<!-- openspec: explain-arcantry-fit-and-adoption-tradeoffs -->
#### Explain when Arcantry fits and what adoption requires

Arcantry presents recognizable project outcomes, adoption trade-offs and compatibility with mature engineering practices before requiring readers to learn its source model. It makes clear that shared project-work configuration does not automatically integrate Arcantry into product runtime, build, CI or publication workflows.

<!-- openspec: make-cli-documentation-verifiable -->
#### Keep the CLI reference aligned and readable

The CLI reference now fails verification when command syntax drifts, safety claims lose their evidence or Markdown tables split option alternatives into unintended columns.

<!-- openspec: organize-docs-and-adopt-nub -->
#### Organize documentation and repository tooling

Arcantry now keeps its documentation application in a dedicated workspace, uses mise to provision pinned `just` and Nub versions, retains the root `justfile` as its task runner, and recreates documentation projections during builds instead of storing them as authored sources.

<!-- openspec: preserve-cli-intent-provenance -->
#### Keep CLI expectations traceable to executable evidence

Arcantry traces public CLI behavior and trust claims from authored documentation through accepted OpenSpec requirements to executable native evidence.

<!-- openspec: refine-documentation-navigation -->
#### Navigate Arcantry documentation quickly

Documentation navigation exposes every skill through clear nested families, consistent icons, readable tables, and responsive process diagrams.

<!-- openspec: refine-overview-product-story -->
#### Explain why and how to adopt Arcantry

The documentation overview explains Arcantry's value through concrete outcomes, clear recommended adoption paths, an interactive source configuration map, one cross-platform command picker, and complete project footer details.

<!-- openspec: restore-verified-release-publishing -->
#### Publish one verified Arcantry 1.0 distribution

Arcantry 1.0 ships the same sealed release through native archives, checksum-verifying installers, the `arcantry` npm launcher and exact platform packages. GitHub Actions executes every supported target, retains the verified package archives and keeps the GitHub Release as a draft until the complete npm package set is confirmed.

<!-- openspec: retire-typescript-core-and-tooling -->
#### Use one native Rust engine for Arcantry

Arcantry's CLI, repository operations and supporting project tools use one Rust implementation. The npm package remains a launcher for the native executable and no longer exposes a separate JavaScript library API.

<!-- openspec: support-private-project-artifacts-and-universal-agent-files -->
#### Use universal agent files with private project artifacts

Arcantry now treats `AGENTS.md` and `.agents/skills` as the universal project surfaces, offers explicit Claude compatibility without duplicating source material, and discovers private OpenSpec, changelog and skill artifacts alongside their shared counterparts.

### Fixed

<!-- openspec: adopt-discovered-sources-into-config -->
#### Persist adopted project sources

Adopting a discovered or standard project source now records it in the active configuration, including explicitly selected dependencies, so later inspections retain the adopted responsibility.

<!-- openspec: align-public-trust-surface-with-evidence -->
#### Bound public safety claims to visible evidence

Arcantry distinguishes verified guarantees from responsibility boundaries and narrows public trust claims when evidence does not support the broader promise.

<!-- openspec: harden-native-cli-verification-gates -->
#### Make native CLI verification fail on untested behavior

Arcantry holds native CLI behavior to independent command-level evidence and keeps host, Linux system and release-target execution as distinct gates. Coverage remains an optional diagnostic.

<!-- openspec: improve-configurator-responsive-reflow -->
#### Keep the setup configurator readable between desktop and mobile widths

The configurator reflows questions and generated instructions before its three-region workspace becomes cramped and preserves content at a 320 CSS pixel viewport and with user text-spacing overrides.

<!-- openspec: normalize-cross-platform-skill-manifests -->
#### Keep skill package identity stable across platforms

Arcantry now produces and transports the same skill package revision from equivalent Windows and Linux text checkouts while preserving exact byte identity for binary resources. Package projection, skill update, native package smoke, release workflow and filesystem-specific CLI checks also run reliably from clean checkouts.

<!-- openspec: preserve-todo-queue-conventions -->
#### Preserve each todo queue's established vocabulary during capture

Arcantry previews the exact task line, reuses compatible local conventions and asks before inventing or omitting meaningful optional metadata.

<!-- openspec: support-official-todo-txt-baseline -->
#### Preserve the official todo.txt baseline across Arcantry writes

Arcantry CLI and canonical skills use the official todo.txt baseline for new or directly changed tasks while preserving existing queue content and optional metadata.

## [0.4.3] - 2026-08-18

### Changed

<!-- openspec: use-unscoped-arcantry-package -->
#### Use the concise Arcantry npm package name

The combined CLI and library package now uses `arcantry`, so launcher commands and public imports no longer repeat the product name.

## [0.4.2] - 2026-08-18

### Security

<!-- openspec: harden-release-artifact-parser -->
#### Bound release artifact title parsing

Release validation now parses contributed title lines in linear time, preventing malformed OpenSpec content from causing polynomial regular-expression work in CI.

## [0.4.0] - 2026-08-18

### Changed

<!-- openspec: compose-versioned-project-knowledge -->
#### Compose versioned project knowledge

Arcantry can discover and combine independently versioned OpenSpec, changelog and todo.txt sources with optional TOML configuration, explicit compatibility and safe per-source transitions in new or established projects. Skills can declare compatible source kinds, adapter ranges and learning outcomes without becoming project state.

<!-- openspec: preserve-derived-artifact-fidelity -->
#### Preserve derived artifact fidelity

Audience and scope guidance now preserves the structure and comparison semantics of existing artifacts unless the requester explicitly authorizes a different representation.

<!-- openspec: publish-under-arcantry-npm-scope -->
#### Publish from the Arcantry npm organization

Arcantry uses the `@arcantry/arcantry` public package identity and can publish a verified package archive from a sealed release through token-free npm trusted publishing after a one-time maintainer bootstrap.

<!-- openspec: refine-documentation-entrypoint -->
#### Clearer documentation entrypoint

Arcantry now explains its repository outcome in plain language, offers equivalent npm, pnpm, and Nub launch commands, keeps version, authorship, theme, and GitHub controls consistent across the documentation, and adds a responsive overview atmosphere with smooth page-to-page navigation.

## [0.3.2] - 2026-08-17

### Fixed

<!-- openspec: initialize-ci-adoption -->
#### Clean CI checkouts initialize private adoption state

Arcantry CI now initializes ephemeral repository adoption through the public CLI before running strict read-only self-validation, without committing private `.local` configuration.

## [0.3.1] - 2026-08-17

### Fixed

<!-- openspec: enforce-release-dogfooding -->
#### Release governance covers every completed repository change

Arcantry now requires each completed repository state to be backed by archived OpenSpec intent, an internal SemVer release and the generated changelog, even when no package, tag or GitHub release is published.

<!-- openspec: harden-repository-validation -->
#### Repository validation detects drift and schema violations

Arcantry now detects outdated managed guidance, reports actionable doctor repairs, rejects unsupported catalog metadata and runs its public repository and skill validators in CI.

<!-- openspec: require-complete-review-coverage -->
#### Complete code reviews account for the full requested surface

The staged code review skill now requires repository-wide reviews to account for every material component and contract before claiming complete coverage.

## [0.3.0] - 2026-08-17

### Added

<!-- openspec: unify-arcantry-capabilities -->
#### Unified Arcantry capabilities

Arcantry now combines its OpenSpec lifecycle with reusable skills, a complete Codex plugin, a versioned catalog, and one CLI for safe repository adoption and skill management.

### Changed

<!-- openspec: redesign-developer-documentation -->
#### Developer-first documentation

Arcantry now opens with a concrete repository adoption path, keeps one brand lockup in the shell, and routes developers through the skill catalog by the work they need to do.

## [0.2.3] - 2026-08-16

### Fixed

<!-- openspec: fix-theme-and-visual-restraint -->
#### Theme switching now works and the docs chrome is more restrained

Arcantry documentation now applies distinct light and dark palettes correctly, keeps brand assets readable in both modes, and removes glassy, pill-heavy visual treatments that were not part of the approved design direction.

## [0.2.2] - 2026-08-16

### Fixed

<!-- openspec: fix-brand-fidelity -->
#### Documentation branding fidelity

The documentation now uses the approved Arcantry logo assets, neutral concept palette and intended Manrope / Instrument Serif typography, with the compact mark carried through the header and favicon.

## [0.2.1] - 2026-08-16

### Changed

<!-- openspec: match-docs-concept -->
#### Documentation now follows the Arcantry concept

The documentation shell and overview now use the approved Arcantry layout, typography hierarchy and navigation treatment instead of presenting the default Starlight visual language.

## [0.2.0] - 2026-08-16

### Changed

<!-- openspec: harden-release-integrity -->
#### Release state is now self-checking

Arcantry now validates release manifests and archived change assignments as one state, can cut the next release from the OpenSpec plan, and fails repository checks when the committed changelog drifts from its OpenSpec sources.

## [0.1.0] - 2026-08-16

### Added

<!-- openspec: bootstrap-arcantry -->
#### Initial Arcantry repository lifecycle

Arcantry now provides a spec-driven repository foundation with OpenSpec-based release history, SemVer planning from delivered changes, a stable `just` command surface, `mise` tool pinning, Astro documentation and GitHub Actions automation.

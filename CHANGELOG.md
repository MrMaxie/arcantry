# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-08-18

### Added

<!-- openspec: stabilize-arcantry-1-0 -->
#### Coordinate project knowledge and agent workflows with Arcantry

Arcantry provides local-first project configuration, safe repository adoption, a portable catalog of focused agent skills, and one documented workflow for maintaining project intent, release meaning, task intake, verification, and audience-safe content.

<!-- openspec: support-multiple-agent-hosts -->
#### Use universal Agent Skills with Claude compatibility

Arcantry links individual skills through the universal `.agents` directory, lets Codex consume that standard directly, and can add Claude Code compatibility aliases to the same canonical packages.

### Changed

<!-- openspec: refine-documentation-navigation -->
#### Navigate Arcantry documentation quickly

Documentation navigation exposes every skill through clear nested families, consistent icons, readable tables, and responsive process diagrams.

<!-- openspec: refine-overview-product-story -->
#### Explain why and how to adopt Arcantry

The documentation overview explains Arcantry's value through concrete outcomes, clear recommended adoption paths, an interactive source configuration map, one cross-platform command picker, and complete project footer details.

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

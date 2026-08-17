# Changelog

## 0.3.1 - 2026-08-17

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

## 0.3.0 - 2026-08-17

### Added

<!-- openspec: unify-arcantry-capabilities -->
#### Unified Arcantry capabilities

Arcantry now combines its OpenSpec lifecycle with reusable skills, a complete Codex plugin, a versioned catalog, and one CLI for safe repository adoption and skill management.

### Changed

<!-- openspec: redesign-developer-documentation -->
#### Developer-first documentation

Arcantry now opens with a concrete repository adoption path, keeps one brand lockup in the shell, and routes developers through the skill catalog by the work they need to do.

## 0.2.3 - 2026-08-16

### Fixed

<!-- openspec: fix-theme-and-visual-restraint -->
#### Theme switching now works and the docs chrome is more restrained

Arcantry documentation now applies distinct light and dark palettes correctly, keeps brand assets readable in both modes, and removes glassy, pill-heavy visual treatments that were not part of the approved design direction.

## 0.2.2 - 2026-08-16

### Fixed

<!-- openspec: fix-brand-fidelity -->
#### Documentation branding fidelity

The documentation now uses the approved Arcantry logo assets, neutral concept palette and intended Manrope / Instrument Serif typography, with the compact mark carried through the header and favicon.

## 0.2.1 - 2026-08-16

### Changed

<!-- openspec: match-docs-concept -->
#### Documentation now follows the Arcantry concept

The documentation shell and overview now use the approved Arcantry layout, typography hierarchy and navigation treatment instead of presenting the default Starlight visual language.

## 0.2.0 - 2026-08-16

### Changed

<!-- openspec: harden-release-integrity -->
#### Release state is now self-checking

Arcantry now validates release manifests and archived change assignments as one state, can cut the next release from the OpenSpec plan, and fails repository checks when the committed changelog drifts from its OpenSpec sources.

## 0.1.0 - 2026-08-16

### Added

<!-- openspec: bootstrap-arcantry -->
#### Initial Arcantry repository lifecycle

Arcantry now provides a spec-driven repository foundation with OpenSpec-based release history, SemVer planning from delivered changes, a stable `just` command surface, `mise` tool pinning, Astro documentation and GitHub Actions automation.

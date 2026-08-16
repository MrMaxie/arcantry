# Changelog

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

---
title: Repository contract
description: Responsibilities and boundaries of an Arcantry repository.
---

## Sources of truth

| Concern | Source of truth |
| --- | --- |
| Private machine-local context | `.local/` |
| Durable project guidance | `.docs/` |
| Current behavior | `openspec/specs/` |
| Change intent and rationale | active/archived OpenSpec change |
| Release-facing description | archived change `release.md` |
| Release membership | release manifest |
| Skill instructions and resources | `skills/<name>/` |
| Catalog, plugin and skill reference pages | generated from canonical skill packages |
| Versioned implementation | Git |
| Shared tool versions | `mise.toml` |
| Repository commands | `justfile` |
| Automation | GitHub Actions calling `just` |

## Invariants

- A changelog entry must resolve to an archived OpenSpec change.
- A public changelog must not be generated from Git commit messages.
- A release manifest must not duplicate release prose.
- CI must use the same repository command surface as local development.
- Project-native tooling remains free to vary behind that command surface.
- Private `.local/` state must not enter commits, packages or public documentation.
- Generated discovery surfaces must not become competing authored sources.
- Repository and skill diagnostics must use the same public contracts in Arcantry and adopter repositories.

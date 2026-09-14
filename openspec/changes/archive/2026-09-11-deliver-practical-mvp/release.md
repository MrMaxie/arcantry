---
impact: minor
audiences:
  - developers
  - operators
observable_impact: user-felt
changelog: include
components:
  - cli
  - mcp
  - repository-lifecycle
  - docs
---

## Added

### Answer project context through the CLI and MCP

Arcantry exposes shared `context`, `next` and `explain` behavior through the native CLI and a read-only MCP server so agents can resume work from discovered project sources without inferring approval.

### Adapt release workflows to project conventions

Projects can choose SemVer, monotonically increasing integer or calendar release identifiers and can render managed changelogs from a project template while preserving existing history.

## Changed

### Keep planned project mutations recoverable

Saved plans retain source hashes, refuse stale inputs and preserve ambiguous interrupted state for explicit recovery instead of overwriting project content.

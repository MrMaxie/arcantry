# Why

**Status:** Implementation Complete  
**Completed:** 2026-08-17

Arcantry currently defines a strong specification and release lifecycle, but it does not yet provide the reusable skills or repository adoption tooling needed to apply that lifecycle consistently. Users must assemble those capabilities from separate surfaces, and the repository cannot validate itself through the same product contract it presents to adopters.

# What changes

Unify Arcantry as one product with a versioned skill catalog, a complete Codex plugin, a single `arcantry` CLI, and a safe repository adoption workflow.

The CLI exposes `repo` commands for managed repository guidance and `skills` commands for discovering and linking individual skills. Repository adoption gives `.local/`, `.docs/`, and OpenSpec separate responsibilities. Canonical skill packages drive generated catalog, plugin, and per-skill documentation projections.

Arcantry uses the same commands and validation contracts in its own repository that it exposes to adopters.

# Out of scope

- Publishing packages, enabling hosting, or retiring another repository.
- Importing workstation state, credentials, logs, caches, or private working notes.
- Adding alternate binaries or top-level aliases for nested CLI commands.
- Hand-authoring per-skill reference pages that can be generated from canonical skill packages.

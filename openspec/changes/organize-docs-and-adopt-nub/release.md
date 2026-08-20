---
category: changed
impact: patch
visibility: public
components:
  - docs
  - tooling
  - repository-lifecycle
---

# Organize documentation and repository tooling

Arcantry now keeps its documentation application in a dedicated workspace, uses mise to provision pinned `just` and Nub versions, retains the root `justfile` as its task runner, and recreates documentation projections during builds instead of storing them as authored sources.

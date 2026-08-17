---
category: fixed
impact: patch
visibility: public
components:
  - repository-adoption
  - tooling
---

# Clean CI checkouts initialize private adoption state

Arcantry CI now initializes ephemeral repository adoption through the public CLI before running strict read-only self-validation, without committing private `.local` configuration.

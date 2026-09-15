---
category: changed
impact: major
visibility: public
components:
  - cli
  - tooling
  - catalog
  - repository-adoption
  - npm-distribution
---

# Use one native Rust engine for Arcantry

Arcantry's CLI, repository operations and supporting project tools use one Rust implementation. The npm package remains a launcher for the native executable and no longer exposes a separate JavaScript library API.

---
impact: patch
visibility: public
components:
  - cli
  - project-knowledge-stack
  - repository-lifecycle
  - skill-catalog
  - native-distribution
---

## Security

### Enforce repository trust boundaries

Arcantry now rejects repository paths that escape through configuration or filesystem links, bounds dependency and changelog processing, escapes terminal control characters in human output, and isolates the Linux release smoke test from verified artifacts.

---
category: fixed
impact: patch
visibility: internal
components:
  - tooling
  - project-knowledge-stack
---

# Keep configuration path checks portable

Arcantry now verifies its absolute-source-path policy consistently on Windows and Linux so repository CI protects the same configuration contract on every supported runner.

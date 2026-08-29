---
category: fixed
impact: patch
visibility: public
components:
  - cli
  - tooling
  - docs
---

# Make native CLI verification fail on untested behavior

Arcantry holds native CLI behavior to independent command-level evidence, blocking per-file Rust coverage and distinct host, Linux system and release-target execution gates.

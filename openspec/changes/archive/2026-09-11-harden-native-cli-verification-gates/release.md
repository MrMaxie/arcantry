---
category: fixed
impact: patch
visibility: public
components:
  - cli
  - tooling
  - docs
audiences: [developers, maintainers]
observable_impact: significant-technical
changelog: include
---

# Make native CLI verification fail on untested behavior

Arcantry holds native CLI behavior to independent command-level evidence and keeps host, Linux system and release-target execution as distinct gates. Coverage remains an optional diagnostic.

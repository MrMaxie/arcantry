---
category: security
impact: patch
visibility: public
components:
  - tooling
  - repository-lifecycle
---

# Bound release artifact title parsing

Release validation now parses contributed title lines in linear time, preventing malformed OpenSpec content from causing polynomial regular-expression work in CI.

---
category: fixed
impact: patch
visibility: internal
components:
  - repository-lifecycle
---

# Validate sealed pull requests in CI

Pull request checks now preserve merge-result coverage while validating the release seal against the exact submitted head commit.

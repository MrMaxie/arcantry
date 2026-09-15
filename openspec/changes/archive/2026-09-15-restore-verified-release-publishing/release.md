---
impact: patch
visibility: public
components:
  - repository-lifecycle
  - native-distribution
  - npm-distribution
  - docs
---

## Changed

### Publish one verified Arcantry 1.0 distribution

Arcantry 1.0 ships the same sealed release through official GitHub Release archives, checksum-verifying installers, the `arcantry` npm launcher and exact platform packages. GitHub Actions executes every supported target, resolves isolated package tools from the repository-pinned toolchain, retains the verified package archives and keeps the GitHub Release as a draft until the complete npm package set is confirmed. Public installation guidance is limited to npm-compatible launchers and GitHub Release downloads.

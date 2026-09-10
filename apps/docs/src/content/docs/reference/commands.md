---
title: Contributor commands
description: Stable contributor commands used to build and verify the Arcantry repository itself.
---

mise provisions the pinned contributor toolchain. The root `justfile` is the documented task runner for building and verifying Arcantry. Recipes use Nub for TypeScript, JavaScript, and Astro paths, and Cargo for Rust paths. The product CLI is documented separately under [CLI](/reference/cli/).

| Command | Contract |
| --- | --- |
| `mise install` | Install the pinned contributor toolchain. |
| `just setup` | Install the exact dependency graph from `nub.lock`. |
| `just check` | Run generation, Biome, type, test, docs, Clippy and dependency-policy checks. |
| `just build` | Generate documentation projections and build the native CLI and documentation. |
| `just format` | Format the `justfile` and Rust with the repository's two-space policy, then format TypeScript, JavaScript and Astro with Biome. |
| `just generate` | Refresh package metadata and documentation projections from canonical sources. |
| `just native-conformance` | Compare CLI behavior across the black-box compatibility suite. |
| `just rust-coverage` | Produce an optional LCOV diagnostic report using cargo-llvm-cov. |
| `just native-target-check <target>` | Verify one declared native target through tests, build, executable smoke and platform-package smoke. |
| `just openspec-validate` | Run strict validation for the OpenSpec schema and every change. |
| `just package-check` | Build and smoke-test the npm packages for the current platform. |
| `just ci` | Run strict OpenSpec, Windows host and Linux container checks locally. |
| `just docs` | Generate documentation projections and start the documentation site locally. |
| `just release-plan` | Show unassigned archived changes and the resulting SemVer bump. |
| `just release-cut` | Create the next release manifest from that plan. |
| `just release-render` | Regenerate `CHANGELOG.md` from release manifests and archived changes. |
| `just release-check` | Check persisted release artifacts while allowing active and unassigned work. |
| `just release-seal` | Require complete assignment, clean Git state and the final Git release seal. |
| `just publish-check vX.Y.Z` | Verify that an npm release tag matches the sealed release and package identity. |

Use `just check-fast` while editing, `just check-host` before a local commit, and `just linux-system-test` for Linux evidence. Coverage is optional diagnostic evidence. Only Pages runs remotely, using `just docs-build`; CI and release workflows are disabled.

`just --list` contains stable contributor entrypoints. Workflow-only helpers remain callable by automation without appearing in that list.

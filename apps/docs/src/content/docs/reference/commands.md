---
title: Contributor commands
description: Stable contributor commands used to build and verify the Arcantry repository itself.
---

mise pins and installs `just` and Nub. The root `justfile` is the task runner for building and verifying Arcantry itself; its recipes use Nub to provision Node, install dependencies and invoke the underlying repository tools. The product CLI is documented separately under [CLI](/arcantry/reference/cli/).

| Command | Contract |
| --- | --- |
| `mise install` | Install the pinned `just` and Nub versions. |
| `just setup` | Install the exact dependency graph from `nub.lock`. |
| `just check` | Run generation, Biome, type, test, release-consistency and docs checks. |
| `just build` | Generate documentation projections and produce package and documentation builds. |
| `just ci` | Validate OpenSpec, release consistency, tests, generated state, builds and public self-checks. |
| `just docs` | Generate documentation projections and start the documentation site locally. |
| `just arcantry-init` | Initialize this checkout's private adoption state through the public CLI. |
| `just arcantry-doctor` | Diagnose this checkout through the public repository command. |
| `just arcantry-validate` | Validate this checkout through the public repository command. |
| `just arcantry-skills-doctor` | Validate this checkout's canonical skill catalog through the public CLI. |
| `just release-plan` | Show unassigned archived changes and the resulting SemVer bump. |
| `just release-cut` | Create the next release manifest from that plan. |
| `just release-render` | Regenerate `CHANGELOG.md` from release manifests and archived changes. |
| `just release-check` | Check persisted release artifacts while allowing active and unassigned work. |
| `just release-seal` | Require complete assignment, clean Git state and the final Git release seal. |
| `just publish-check vX.Y.Z` | Verify that an npm release tag matches the sealed release and package identity. |

CI installs the pinned `just` and Nub versions through mise, initializes ephemeral private adoption state and runs `just ci` instead of reimplementing repository logic in workflow YAML. The generated `.local` state remains excluded from Git.

Arcantry's checks call the same repository and skill validation contracts exposed through the `arcantry` CLI. The `justfile` coordinates project-specific checks; it does not define a second product contract.

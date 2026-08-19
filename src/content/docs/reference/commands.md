---
title: Contributor commands
description: Stable contributor commands used to build and verify the Arcantry repository itself.
---

The Arcantry repository uses `justfile` as its contributor command surface. These commands build and verify Arcantry itself; the product CLI is documented separately under [CLI](/arcantry/reference/cli/).

| Command | Contract |
| --- | --- |
| `just setup` | Install project dependencies after `mise install`. |
| `just check` | Run type, test, release-consistency and docs checks. |
| `just build` | Produce the normal build output. |
| `just ci` | Validate OpenSpec, release consistency, tests, generated state, builds and public self-checks. |
| `just docs` | Start the documentation site locally. |
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

CI initializes ephemeral private adoption state and then calls the repository commands instead of reimplementing their logic in workflow YAML. The generated `.local` state remains excluded from Git.

Arcantry's checks call the same repository and skill validation contracts exposed through the `arcantry` CLI. The wrapper coordinates project-specific checks; it does not define a second product contract.

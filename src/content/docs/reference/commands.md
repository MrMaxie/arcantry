---
title: Contributor commands
description: Stable contributor commands used to build and verify the Arcantry repository itself.
---

The Arcantry repository uses `justfile` as its contributor command surface. These commands build and verify Arcantry itself; the product CLI is documented separately under [CLI](/arcantry/reference/cli/).

| Command | Contract |
| --- | --- |
| `just setup` | Install project dependencies after `mise install`. |
| `just check` | Run type, test, release-integrity and docs checks. |
| `just build` | Produce the normal build output. |
| `just ci` | Validate OpenSpec, release sealing, tests, generated state, builds and public self-checks. |
| `just docs` | Start the documentation site locally. |
| `just arcantry-doctor` | Diagnose this checkout through the public repository command. |
| `just arcantry-validate` | Validate this checkout through the public repository command. |
| `just arcantry-skills-doctor` | Validate this checkout's canonical skill catalog through the public CLI. |
| `just release-plan` | Show unassigned archived changes and the resulting SemVer bump. |
| `just release-cut` | Create the next release manifest from that plan. |
| `just release-render` | Regenerate `CHANGELOG.md` from release manifests and archived changes. |
| `just release-check` | Fail when OpenSpec, version, changelog or Git release sealing is incomplete. |

CI calls the repository commands instead of reimplementing their logic in workflow YAML.

Arcantry's checks call the same repository and skill validation contracts exposed through the `arcantry` CLI. The wrapper coordinates project-specific checks; it does not define a second product contract.

---
title: Commands
description: Stable repository-level commands exposed by just.
---

The `justfile` is the public command surface. Native package/build commands remain implementation details.

| Command | Contract |
| --- | --- |
| `just setup` | Install project dependencies after `mise install`. |
| `just check` | Run type, test, release-integrity and docs checks. |
| `just build` | Produce the normal build output. |
| `just ci` | Validate OpenSpec, run checks and build. |
| `just docs` | Start the documentation site locally. |
| `just release-plan` | Show unassigned archived changes and the resulting SemVer bump. |
| `just release-cut` | Create the next release manifest from that plan. |
| `just release-render` | Regenerate `CHANGELOG.md` from release manifests and archived changes. |
| `just release-check` | Fail when release state is invalid or the committed changelog is stale. |

CI calls the repository commands instead of reimplementing their logic in workflow YAML.

---
title: Commands
description: Stable repository-level commands exposed by just.
---

The `justfile` is the public command surface. Native package/build commands remain implementation details.

| Command | Contract |
| --- | --- |
| `just setup` | Install project dependencies after `mise install`. |
| `just check` | Run the repository's fast correctness checks. |
| `just build` | Produce the normal build output. |
| `just ci` | Validate OpenSpec, run checks and build. |
| `just docs` | Start the documentation site locally. |
| `just release-plan` | Inspect archived changes and proposed SemVer impact. |
| `just release-render` | Render `CHANGELOG.md` from a release manifest. |

CI should call these commands rather than duplicate their underlying command lines in workflow YAML.

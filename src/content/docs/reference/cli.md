---
title: CLI
description: The stable arcantry command surface for repository adoption and skill management.
---

Arcantry exposes one binary with two command groups. Repository operations live under `arcantry repo`; skill operations live under `arcantry skills`.

There are no top-level install or update aliases.

## Repository commands

| Command | Contract |
| --- | --- |
| `arcantry repo init` | Add the missing managed repository foundation while preserving existing unowned content. |
| `arcantry repo update` | Bring managed metadata and generated artifacts forward without resetting user-editable choices. |
| `arcantry repo doctor` | Explain current state, conflicts and explicit repair actions without changing files. |
| `arcantry repo validate` | Return a deterministic validation result without changing files. |
| `arcantry repo remove` | Remove verified Arcantry-managed artifacts while preserving unowned user content. |

Run repository commands from inside the target Git repository. Mutating commands report each artifact they create, update or remove. Diagnostic commands are safe for CI and inspection.

## Skill commands

| Command | Contract |
| --- | --- |
| `arcantry skills list` | List validated skills in the current catalog. |
| `arcantry skills inspect <name>` | Show one skill's purpose, tags and usage scenarios. |
| `arcantry skills link <name>` | Link one canonical skill into the configured skill home. |
| `arcantry skills unlink <name>` | Remove only the verified Arcantry-managed link for one skill. |
| `arcantry skills doctor` | Check catalog integrity and local link health without changing state. |

## Verification path

Use the read-only commands together when a repository consumes local skills:

```text
arcantry repo validate
arcantry skills doctor
```

Arcantry runs the same contracts against its own repository. Its contributor and CI command surface may wrap them, but does not maintain a separate validation implementation.

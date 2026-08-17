---
title: Skills
description: Discover one focused capability or install the complete Arcantry skill catalog.
---

Arcantry skills are focused instruction packages for repeatable agent work. Each package owns its instructions, public metadata and any assets, references, scripts or tool dependencies needed to use it.

## Choose a distribution path

| Need | Use |
| --- | --- |
| See what is available | `arcantry skills list` |
| Understand one skill before adopting it | `arcantry skills inspect <name>` |
| Use one skill from this checkout | `arcantry skills link <name>` |
| Remove one Arcantry-managed link | `arcantry skills unlink <name>` |
| Install the complete collection | Arcantry Codex plugin |

Individual linking keeps the installed surface small. The plugin provides the complete catalog as one versioned collection. Both use the same canonical skill packages.

## Inspect before linking

```text
arcantry skills list
arcantry skills inspect <name>
arcantry skills link <name>
arcantry skills doctor
```

`inspect` shows the skill's purpose, tags and usage scenarios. `doctor` checks catalog and link health without changing the installed state.

Linking is idempotent when the existing link already targets the canonical package. Arcantry does not replace an ordinary skill directory silently. Unlinking removes only a verified Arcantry-managed link.

## Tool and write boundaries

A skill may declare a connector or command-line dependency. That dependency describes what the skill needs; it does not authorize external writes.

Creating an issue, posting a reply, publishing content or updating another system still requires authority for the exact target and action. Read-only context remains read-only unless the user explicitly expands it.

## Generated skill reference

Per-skill reference pages are generated from canonical skill packages so descriptions, scenarios and dependencies stay aligned with the catalog and plugin. Cross-cutting journeys such as adoption and repository workflow remain authored documentation.

[Browse the complete skill catalog](./catalog/)

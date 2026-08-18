---
title: Skills
description: Choose focused Arcantry skills for self-improvement, repository safety, and content safety.
---

Arcantry skills are self-contained Agent Skills packages. Each one owns a focused procedure, public metadata, scenarios, and any references or scripts it needs.

## Three families

| Family | Use it for |
| --- | --- |
| `self-improvement` | Capture repeated work, create and evaluate skills, maintain agent guidance, select task skills, and stage review findings. |
| `repo-safely` | Adopt a repository, capture incoming work, promote todo work into OpenSpec, reconcile sources, maintain release meaning, and verify proportionally. |
| `content-safely` | Protect audience and privacy, design terminal experiences, and write concrete product content without slop. |

No family is a router skill. Every catalog entry performs one focused job.

## Inspect and link one skill

```text
arcantry skills list
arcantry skills inspect <name>
arcantry skills link <name> --scope user
arcantry skills link <name> --scope user --agent claude
arcantry skills link <name> --scope user --agent gemini
arcantry skills doctor --scope user
```

The target depends on scope and agent:

| Agent | User scope | Repository scope |
| --- | --- | --- |
| Codex or default | `~/.agents/skills` | `<repo>/.agents/skills` |
| Claude Code | `~/.claude/skills` | `<repo>/.claude/skills` |
| Gemini CLI | `~/.gemini/skills` | `<repo>/.gemini/skills` |

Repository scope uses the same agent profile:

```text
arcantry skills link <name> --scope repo --agent claude
```

Use `--target <path>` only for an advanced explicit Agent Skills directory. It cannot be combined with `--scope` or `--agent`.

Linking is idempotent when the target already points to the canonical package. Arcantry does not overwrite an ordinary directory. `--replace` creates a backup first. Unlinking removes only an exact link to the selected Arcantry skill.

## Load the complete catalog

The repository exposes the same `skills/` tree through native host manifests:

```text
claude --plugin-dir ./arcantry
gemini extensions install https://github.com/MrMaxie/arcantry
```

Claude Code namespaces plugin skills as `/arcantry:<name>`. Gemini CLI loads the catalog as the `arcantry` extension. Codex uses `.codex-plugin/plugin.json` from the same repository. All three manifests carry the current Arcantry version.

## Other Agent Skills workflows

The packages use the open Agent Skills directory format, so compatible independent installers can discover them from `skills/*/SKILL.md`:

```text
gh skill install MrMaxie/arcantry <name> --agent codex --scope user
npx skills add MrMaxie/arcantry --skill <name> -a codex -g
```

See the [GitHub CLI skill installer](https://cli.github.com/manual/gh_skill_install) and the [open `skills` CLI](https://github.com/vercel-labs/skills) for their current scope and host rules. They may choose host-specific destinations. Manual copying or symbolic linking also works.

These workflows are alternatives to the Arcantry linker, not runtime dependencies.

## Tool and write boundaries

A skill may declare a connector or command-line dependency. That declaration does not authorize creating issues, posting replies, publishing content, or changing another system. External writes still require authority for the exact target and action.

## Generated reference

Per-skill pages are generated from canonical packages so descriptions, scenarios, family placement, and dependencies remain aligned.

[Browse the complete skill catalog](./catalog/)

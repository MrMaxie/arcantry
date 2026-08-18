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
arcantry skills link <name> --scope user --compat claude
arcantry skills doctor --scope user
```

Arcantry recommends the universal Agent Skills locations:

| Surface | User scope | Repository scope |
| --- | --- | --- |
| Universal Agent Skills | `~/.agents/skills` | `<repo>/.agents/skills` |
| Optional Claude compatibility | `~/.claude/skills` | `<repo>/.claude/skills` |

Codex reads the universal surface directly. Claude Code compatibility is an additional alias to the same canonical package:

```text
arcantry skills link <name> --scope repo --compat claude
```

Use `--target <path>` only for an advanced explicit Agent Skills directory. It cannot be combined with `--scope` or `--compat`.

Linking is idempotent when the target already points to the canonical package. Arcantry preflights the universal and compatibility destinations before writing. It does not overwrite an ordinary directory. `--replace` creates a backup first. Unlinking removes only exact links to the selected skill.

## Keep repository skills private

A repository can keep a canonical skill under `.local/skills/<name>` and expose it through locally excluded links:

```text
arcantry skills list --scope private
arcantry skills inspect <name> --scope private
arcantry skills link <name> --scope private
arcantry skills link <name> --scope private --compat claude
```

Private and public packages cannot reuse the same skill name. `.agents` and `.claude` aliases that resolve to one real package remain one skill, not duplicates.

## Load the complete catalog

The repository exposes the same `skills/` tree through optional package manifests:

```text
claude --plugin-dir ./arcantry
```

Claude Code namespaces plugin skills as `/arcantry:<name>`. Codex can use `.codex-plugin/plugin.json` from the same repository. Both manifests carry the current Arcantry version, but neither is required for the universal `.agents` workflow.

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

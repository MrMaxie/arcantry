---
title: Skills
description: Choose and install one focused Arcantry skill.
---

Arcantry skills are self-contained Agent Skills packages. Each skill owns one focused procedure, and every detail page is generated from its canonical package.

## Choose a family

| Family | Use it for |
| --- | --- |
| Self improvement | Improve repeatable agent work, guidance, skill selection, and review. |
| Repository safety | Adopt repositories, manage project knowledge, and verify changes without crossing ownership boundaries. |
| Content safety | Protect audience and privacy, and design or write clear user-facing content. |
| Code quality | Assess code, design maintainable boundaries, and perform authorized behavior-preserving refactors. |

Families organize the catalog; they do not add another routing layer.

Every public skill declares its own semantic version and role: `primary` for a complete user outcome, `supporting` for a companion boundary, or `advanced` for an expert workflow. These fields describe the skill package and do not change whether a host may select it automatically.

## Install one skill

Find the exact name, inspect it, then link only that skill:

```sh
arcantry skills list
arcantry skills inspect <name>
arcantry skills link <name> --scope user
```

User scope makes the skill available across projects. Use `arcantry skills link <name> --scope repo` when one repository should own the link. Add `--compat claude` only when you also need a Claude alias.

| Surface | User scope | Repository scope |
| --- | --- | --- |
| Universal Agent Skills | `~/.agents/skills` | `<repo>/.agents/skills` |
| Optional Claude alias | `~/.claude/skills` | `<repo>/.claude/skills` |

Codex reads the universal `.agents/skills` surface directly. Linking is idempotent and never overwrites an ordinary directory unless you explicitly use `--replace`, which creates a backup first.

## Check and update one skill

Local status does not contact the network:

```sh
arcantry skills status assess-code-quality --scope user
```

Add `--check` to compare it with the official Arcantry `master`. Save and review an exact update before applying it:

```sh
arcantry --output skill-update.json skills update assess-code-quality --scope user
arcantry skills apply --plan skill-update.json
```

An explicit `--catalog-root` uses a local catalog instead. Updates never match private skills to the public source or replace an unmanaged or locally modified package.

## Keep a skill private

A repository may keep a canonical package under `.local/skills/<name>` and expose only locally excluded links:

```sh
arcantry skills list --scope private
arcantry skills link <name> --scope private
```

Private and public packages cannot reuse the same skill name. Universal and Claude links to one canonical package still count as one skill.

## Install without the Arcantry CLI

Compatible Agent Skills installers can install one named package directly:

```sh
gh skill install MrMaxie/arcantry <name> --agent codex --scope user
npx skills add MrMaxie/arcantry --skill <name> -a codex -g
```

[`gh skill install`](https://cli.github.com/manual/gh_skill_install) and [`npx skills add`](https://github.com/vercel-labs/skills) are alternatives to the Arcantry linker, not runtime dependencies. Manual copying or symbolic linking also works. Use `claude --plugin-dir ./arcantry` only when you explicitly want the complete catalog instead of one skill.

A declared tool dependency never authorizes an external write. The user still controls the exact target and action.

[Browse the complete skill catalog](./catalog/)

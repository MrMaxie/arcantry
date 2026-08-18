---
title: Adoption paths
description: Choose a trace-free, external, tracked, monorepo, changelog, queue, or skills-only path.
---

Arcantry does not classify an entire project as greenfield or brownfield. Inspect first, then choose discovery, footprint, and responsibility independently for each source.

```sh
arcantry repo inspect
```

Inspection is read-only and valid without Git, configuration, or recognized sources.

## Choose an adoption path

| Situation | Start here | Repository footprint |
| --- | --- | --- |
| Empty directory | Inspect, then adopt only the source you need. | None until an explicit plan is applied. |
| Mature project | Observe discovered sources before assigning responsibility. | None while inspection remains configuration-free. |
| No tracked Arcantry metadata | Use one explicit external `arcantry.toml`. | None from configuration. |
| Shared project contract | Track one `arcantry.toml` at the selected root. | One project-owned configuration file. |
| Monorepo | Set one root and distinct source ids, paths, scopes, and relationships. | External or one tracked configuration. |
| Existing changelog | Observe it, preserve it, or define an explicit managed boundary. | No rewrite without an applied plan. |
| Queues or skills only | Use `todo` or `skills` commands directly. | Only an explicitly applied queue change or chosen skill link. |

## Empty directory

An empty inspection is a successful result. Add a source only when the project has a concrete use for it. Define that source in an external or tracked configuration, inspect again, then use the reported id:

```sh
arcantry repo plan --source <id> --transition adopt --json > plan.json
```

Planning does not write. Review and store the serialized plan according to its visibility before applying it.

## Mature project

Discovered OpenSpec, `CHANGELOG.md`, root `todo.txt`, and `.local/todo.txt` default to observation when no configuration assigns another responsibility. Unrelated files remain project-owned. Existing `.docs` content is outside Arcantry's responsibility and is not classified, moved, validated, or removed.

Use `preserve` when the current source remains as it is, `adopt` to take responsibility for an existing or missing source, `rebind` to change its role, `cutover` to manage only a future changelog boundary, `migrate` when old meaning is recoverable, and `relocate` to move a source explicitly.

## External configuration

Keep the control contract outside the target when the project must remain free of Arcantry metadata:

```sh
arcantry --cwd ./project --config ../control/project.toml repo inspect
```

An explicit external configuration may use absolute source paths. It remains singular and is not merged with a nearer project file.

## Tracked configuration

Track one `arcantry.toml` when collaborators and automation need the same root, sources, versions, and responsibilities. The nearest file is selected automatically unless `--config` overrides it.

Configuration is not mandatory, and adding it does not install Node tooling, initialize Git, or create package-manager or task-runner files. See [Configuration](/arcantry/reference/configuration/).

## Monorepo

Resolve one project root, then model each knowledge source separately. Use `scope` to partition OpenSpec authority and `from` to connect a changelog projection to one or more explicit authorities. Overlapping managed OpenSpec scopes and dependency cycles are rejected.

Separate configuration files are not layered. Choose one explicit file for each invocation or place one shared file at the intended ancestor.

## Existing changelog

An existing changelog can remain observed without OpenSpec. Arcantry does not infer its prose from commits or diffs.

When Arcantry manages future changelog entries, the configured changelog must depend on OpenSpec through `from`. Use `managed_from` and the `cutover` transition to preserve earlier history while deriving later release meaning from accepted OpenSpec artifacts. Use `migrate` only when historical meaning can be mapped without guessing.

## Queues or skills only

Todo queues and skills do not require OpenSpec or project configuration:

```sh
arcantry todo list
arcantry skills list
arcantry skills inspect <name>
```

A todo mutation previews by default and writes only with `--apply`. Linking a skill changes only the selected agent skill directory and does not make the skill authoritative project state.

## Legacy compatibility

`.local/arcantry.json` remains readable as a private legacy contract. `repo init`, `repo update`, and `repo remove` are compatibility commands, not the recommended adoption path. Only `repo init --docs none` is accepted; `.docs` remains entirely project-owned.

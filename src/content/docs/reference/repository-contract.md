---
title: Project knowledge stack
description: Understand authority, projections, queues, capabilities, privacy, and integrity evidence.
---

Arcantry composes project knowledge without treating every artifact as the same kind of truth. Each source keeps a distinct role, responsibility, path, and versioned adapter.

## Roles in the stack

| Role | Arcantry model | Responsibility |
| --- | --- | --- |
| Authority | OpenSpec source | Accepted product and engineering meaning for its configured scope. |
| Projection | Changelog source | Human release history derived from explicit authority when managed. |
| Queue | todo.txt source | Short shared or private work items without product authority. |
| Capability | Skill package | A reusable procedure that may describe compatibility but is not project state. |
| Privacy boundary | `.local/` | Machine-local or private sources that must not be presented as shared. |
| Integrity evidence | Optional VCS, including Git | Evidence about implementation history or release sealing, never the source of release prose. |

Project-owned documentation, including any `.docs` directory, remains outside Arcantry's model unless a future explicit capability says otherwise.

## Three independent axes

### Discovery

A project can use configuration-free discovery, one explicit `--config` file, or the nearest ancestor `arcantry.toml`. These modes decide how Arcantry finds a stack, not what it may change.

### Footprint

Arcantry can remain absent from the project, use an external or private configuration, or use one tracked project configuration. Footprint does not imply management responsibility.

### Management

Each source independently uses one level:

| Level | Meaning |
| --- | --- |
| `ignore` | Exclude the source from Arcantry responsibility. |
| `observe` | Describe the source without enforcing or changing it. |
| `validate` | Check the configured contract without writing. |
| `manage` | Permit explicit planned writes through the source adapter. |

A project can therefore manage one source, validate another, observe a third, and omit the rest.

## Relationships and meaning

Source dependencies form an acyclic `from` graph. A managed changelog is a projection of one or more OpenSpec authorities, so its wording, category, and version impact come from accepted OpenSpec release artifacts. Git commits and diffs are not semantic inputs.

An observed or validated changelog can exist without OpenSpec. In that case Arcantry can report or check its structure, but it cannot generate new release meaning for it.

Todo queues are independent. They preserve todo.txt metadata and do not become specifications or changelog inputs. Skills are also independent: they teach a procedure but do not install themselves or establish project facts.

## Version boundaries

Four versions may coexist:

- the Arcantry CLI version;
- the `config_version` format;
- each source adapter version, such as `openspec@1`;
- the source data version, such as a changelog's historical format boundary.

Updating the CLI does not automatically update the configuration, adapter, or source data. A supported earlier adapter remains usable until an explicit transition changes it.

## Safety invariants

- Empty directories and missing source kinds are valid.
- Git, configuration, package managers, task runners, and project-local Node tooling are optional.
- Configurations are singular and never implicitly merged.
- Managed source relationships are acyclic and managed OpenSpec authorities do not overlap.
- `inspect` and `plan` do not write.
- `apply` rejects changed inputs and corrupt planned content before accepting the transition.
- Serialized plans must be protected because they can contain planned source content.
- Private `.local` sources cannot be declared shared.
- Managed changelog meaning always comes from OpenSpec.

## Built-in adapter compatibility

| Source kind | Read | Write |
| --- | --- | --- |
| OpenSpec | `openspec@1` | `openspec@1` |
| Changelog | `keep-a-changelog@1`, `keep-a-changelog@2` | `keep-a-changelog@1`, `keep-a-changelog@2` |
| todo.txt | `todo-txt@1` | `todo-txt@1` |

An unsupported adapter reports its id and stops before producing a partial transition. See [Configuration](/arcantry/reference/configuration/) for the complete `arcantry.toml` contract.

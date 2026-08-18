---
title: Project knowledge stack
description: Understand authority, release projections, queues, capabilities, privacy, and verification evidence.
---

Arcantry keeps project knowledge roles distinct while making them inspectable through one local-first model.

## Roles in the stack

| Role | Arcantry model | Responsibility |
| --- | --- | --- |
| Authority | `openspec` or `.local/openspec` | Accepted shared or private product and engineering meaning for its configured scope. |
| Projection | `CHANGELOG.md` or `.local/CHANGELOG.md` | Shared or private release history derived from explicit authority when managed. |
| Queue | todo.txt source | Short shared or private work items without product authority. |
| Capability | `skills/<name>` or `.local/skills/<name>` | A reusable public or private procedure that is not project knowledge. |
| Guidance | `AGENTS.md` or `.local/AGENTS.md` | Universal shared or private instructions that shape agent behavior. |
| Privacy boundary | `.local/` | Machine-local configuration and sources that must remain untracked. |
| Verification evidence | Static, automated, live, independent, or user checks | Proof matched to the risk and acceptance boundary. |

## Configuration scopes

Shared `arcantry.toml` and private `.local/arcantry.toml` use the same schema. They are independent sources of configuration, not layers to merge.

An explicit `--config` wins. Otherwise Arcantry walks toward the filesystem root and checks the private file before the shared file at each directory. Inspection reports the active file and any sibling it shadows.

## Source responsibility

Each source independently uses one management level:

| Level | Meaning |
| --- | --- |
| `ignore` | Exclude the source from Arcantry responsibility. |
| `observe` | Describe the source without enforcing or changing it. |
| `validate` | Check the configured contract without writing. |
| `manage` | Permit explicit planned writes through the source adapter. |

A project can manage one source, validate another, observe a third, and omit the rest.

## Relationships and release meaning

Source dependencies form an acyclic `from` graph. A managed changelog projects release meaning from one or more OpenSpec authorities. Git commits and diffs remain implementation evidence and do not generate consumer release prose.

Shared release history cannot depend on private intent. Private release history may compose shared and private intent because it remains inside the same privacy boundary.

An observed changelog can exist without OpenSpec. Arcantry may inspect it, but it cannot invent new release meaning for it.

Todo queues stay independent until an explicit move. A hot thought belongs in todo before it becomes accepted intent. Accepted intent belongs in OpenSpec. Completed consumer impact belongs in the changelog. A repeated procedure belongs in the skill improvement pipeline.

## Skill families

- `self-improvement` captures repeated work and maintains scoped skills and agent guidance.
- `repo-safely` adopts repositories, captures project work, reconciles sources, maintains release meaning, and scales verification.
- `content-safely` protects audience and privacy while improving terminal experiences and product writing.

Skills can be used without repository adoption. They do not install themselves or establish project facts.

`.agents/skills` is the universal installation surface. Codex consumes it directly. Claude compatibility uses imports and aliases in branded locations while preserving the universal guidance and canonical package as the only source.

## Safety invariants

- Missing configuration and missing source kinds are valid.
- Git, package managers, task runners, and project-local Node tooling are optional.
- Configuration files are singular for each invocation and never merged.
- Shared and private sources are not synchronized automatically.
- `inspect` and `plan` do not write.
- `apply` rejects changed inputs and corrupt planned content.
- Private source data remains private in plans, logs, and public output.
- External writes require authority for the exact target and action.
- Complete verification claims name their actual coverage.

## Built-in adapter compatibility

| Source kind | Read | Write |
| --- | --- | --- |
| OpenSpec | `openspec@1` | `openspec@1` |
| Changelog | `keep-a-changelog@1`, `keep-a-changelog@2` | `keep-a-changelog@1`, `keep-a-changelog@2` |
| todo.txt | `todo-txt@1` | `todo-txt@1` |

An unsupported adapter stops before a partial transition. See [Configuration](/arcantry/reference/configuration/) for the TOML contract.

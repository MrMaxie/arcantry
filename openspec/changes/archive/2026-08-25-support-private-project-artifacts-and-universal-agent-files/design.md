# Approach

## Universal guidance and Agent Skills

`AGENTS.md` is the shared guidance source and `.local/AGENTS.md` is the private guidance source. Claude compatibility is additive: shared compatibility uses a managed `CLAUDE.md` import of `@AGENTS.md`, while private compatibility uses a locally excluded `CLAUDE.local.md` import of `@.local/AGENTS.md`. Existing user-authored Claude content is preserved outside the managed import.

Canonical public skill packages remain under `skills/<name>`, while private packages live under `.local/skills/<name>`. The standard user and repository destinations remain `~/.agents/skills/<name>` and `.agents/skills/<name>`. `--compat claude` additionally links the same canonical directory into the corresponding `.claude/skills` destination. Codex consumes the standard surface directly and is not modeled as its owner.

The old `--agent` profiles are removed before 1.0. An explicit `--target` remains a single advanced destination and cannot be combined with `--scope` or `--compat`.

## Private project artifacts

Default discovery recognizes both members of each standard pair:

| Responsibility | Shared | Private |
| --- | --- | --- |
| Product intent | `openspec` | `.local/openspec` |
| Release history | `CHANGELOG.md` | `.local/CHANGELOG.md` |
| Task queue | `todo.txt` | `.local/todo.txt` |
| Skill packages | `skills/<name>` | `.local/skills/<name>` |

Private OpenSpec and changelog sources use stable ids `openspec-local` and `changelog-local`. Skills remain procedural capabilities and are reported by skill commands rather than added as project knowledge source kinds.

Shared and private sources can coexist. A managed shared changelog cannot depend on private OpenSpec because that would make public release history irreproducible outside one workstation. A private changelog may depend on either shared or private OpenSpec.

## Identity and atomicity

Skill identity is the frontmatter name plus the canonical real path. Multiple aliases resolving to the same directory are one installation. Different real directories claiming the same name are a hard conflict. Compatibility operations preflight every destination before writing and remove only links created by the failed operation if a later write fails.

## Generated projections and release correction

Canonical `arcantry.json` summaries drive catalog cards and skill-page leads. `SKILL.md` descriptions remain routing text, and `agents/openai.yaml` remains optional Codex UI metadata. Generated documentation presents `.agents` as the recommended standard and Claude as a compatibility section.

The archived proposal and design for the earlier three-host approach remain historical evidence. Because 1.0.0 is not released, its release-facing artifact is corrected in place and the changelog is regenerated so the candidate does not claim Gemini support or symmetric native host profiles.

# Trade-offs

- Claude receives a small branded adapter because it does not consume the universal files directly. This preserves one source without pretending the branded path is the standard.
- Private skills remain outside the project knowledge source graph. Adding them as another source kind would blur procedural capabilities with project facts and duplicate the dedicated skill inventory.
- Existing `--agent` options are removed rather than deprecated because the contract has not been released and retaining them would preserve the incorrect model.

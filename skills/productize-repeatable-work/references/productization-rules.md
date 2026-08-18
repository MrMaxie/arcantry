# Productization Rules

## Surface choice

| Accepted behavior | Implement as |
| --- | --- |
| Shared repository convention or verified command | Surgical `AGENTS.md` update |
| Machine-specific or private project workflow | `.local/AGENTS.md`, `.local/WORKFLOW.md`, or `.local/scripts` |
| Stable mechanical transformation or repeated command construction | Script with parameters, validation, and a nonzero failure exit |
| Reusable judgment-based workflow | Focused skill with concise metadata and progressive disclosure |
| Mechanical enforcement around tool or lifecycle events | Hook after verifying the supported host hook surface |
| Scheduled check or follow-up | Automation with explicit schedule, project, and owner |
| Live authenticated data or controlled external action | Separate plugin or MCP proposal |

## Skill destinations

- Version-controlled public skill: the approved catalog's canonical `skills/<name>` directory plus its required catalog metadata.
- Private project-only skill: canonical `.local/skills/<name>` package plus a locally excluded `.agents/skills/<name>` link.
- User skill: the approved catalog package linked through `~/.agents/skills/<name>`; add a Claude alias only when explicitly requested.

## Completion gate

- Scope matches the accepted candidate.
- No secret or raw transcript was added.
- Public and private guidance are separated.
- Generated files are reproducible or have recorded source hashes.
- Relevant syntax, unit, validation, and forward tests passed.
- No commit or external publication occurred without separate authorization.

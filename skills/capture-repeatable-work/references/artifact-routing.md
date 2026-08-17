# Artifact Routing

Choose the smallest surface that reliably owns the repeated decision.

| Signal | Surface | Default location |
| --- | --- | --- |
| One-off request or unresolved ambiguity | No durable artifact | Current conversation |
| Shared repository convention, command, validation rule, or source of truth | `AGENTS.md` | Closest applicable repository directory |
| Machine path, private tool setup, or personal project workflow | Private local instruction | `.local/AGENTS.md` or `.local/WORKFLOW.md` |
| Stable inputs, outputs, and mechanical steps | Script | Repository tooling path when shared; `.local/scripts` when private |
| Reusable workflow requiring judgment or conditional tool use | Skill | Repository skill for one product; global skill for cross-project behavior |
| Mechanical lifecycle enforcement | Hook | Project or global Codex configuration, matching its audience |
| Work that must run on a schedule | Automation | Codex automation with an explicit owner and scope |
| Live authenticated service or controlled external action | MCP or plugin candidate | Propose separately; do not build incidentally |

## Scope gates

- Require evidence from unrelated projects before recommending a global skill, unless one case exposes a clear safety-critical omission.
- Keep project data, thread identifiers, local paths, and private service details out of global artifacts.
- Prefer replacing or consolidating an existing artifact over adding a competing rule or skill.
- A script is preferable to prose when correctness depends on deterministic parsing, hashing, transformation, or repeated command construction.


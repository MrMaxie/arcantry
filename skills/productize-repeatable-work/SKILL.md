---
name: productize-repeatable-work
description: Implement one approved repeatable-work candidate in its chosen durable carrier without reopening its scope. Use only with an accepted stable candidate id.
---

# Productize Repeatable Work

Implement exactly one accepted repeatable-work candidate without reopening its decision or absorbing neighboring improvements.

## Authorization gate

Require an accepted stable ID such as `R1` plus its exact outcome, audience, and target. Treat `productize R1`, `implement R1`, or equivalent explicit wording as authorization for that candidate; do not request duplicate confirmation.

Stop and return a revised proposal when the target preimage drifted, the accepted candidate is incomplete, or implementation requires a broader surface.

## Workflow

1. Re-read the accepted candidate, target, applicable `AGENTS.md`, and relevant private instructions.
2. Read [references/productization-rules.md](references/productization-rules.md) and confirm the narrowest artifact.
3. Preserve unrelated files and existing user changes.
4. Use the owning companion:
   - `maintain-agent-guidance` for `AGENTS.md` or `.local/AGENTS.md`;
   - the host's `skill-creator` capability for a skill;
   - the automation tool for scheduled work;
   - applicable host configuration guidance for a hook;
   - the target catalog's documented workflow for an approved catalog operation.
5. Add deterministic scripts only when stable inputs, outputs, and failure behavior are defined. Test every added script.
6. Run `evaluate-skill-change` before adopting a new or changed skill.
7. Validate the resulting artifact and report the accepted ID, target, checks, and remaining gaps.

## Safety

- Put private paths, personal workflow, and machine setup only under `.local` or a private global skill.
- Keep a private repository skill under `.local/skills/<name>`, expose it through the universal `.agents/skills/<name>` link, and exclude `.local/` plus each private link in `.git/info/exclude`; never edit shared `.gitignore` for this purpose.
- Keep a global personal skill's canonical source in its approved version-controlled catalog and use the catalog's supported install or link workflow.
- Refuse to overwrite locally modified generated artifacts unless the accepted candidate explicitly authorizes replacement.
- Do not commit, push, publish, send messages, or enable automatic adoption without separate authorization.
- Do not install SkillOpt or any self-modifying runtime.

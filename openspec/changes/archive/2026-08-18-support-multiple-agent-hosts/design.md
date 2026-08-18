# Host profiles

The canonical skill package remains the directory under `skills/<name>`. Host selection changes only the link destination:

| Host | User scope | Repository scope |
| --- | --- | --- |
| Default / Codex | `~/.agents/skills` | `<repo>/.agents/skills` |
| Claude Code | `~/.claude/skills` | `<repo>/.claude/skills` |
| Gemini CLI | `~/.gemini/skills` | `<repo>/.gemini/skills` |

The CLI accepts `--agent codex|claude|gemini` only with `--scope user|repo`. An explicit `--target` remains the escape hatch for any other verified host and remains mutually exclusive with both `--scope` and `--agent`. Omitting `--agent` preserves the existing `.agents/skills` destination.

# Complete catalog distribution

`tooling/generate.ts` remains the authored projection boundary. It derives `.codex-plugin/plugin.json`, `.claude-plugin/plugin.json`, and `gemini-extension.json` from the current release version and shared product identity. Each host consumes the same root `skills/` tree; no copied or provider-specific skill bodies are created.

Package preparation copies the generated host manifests into the npm package projection. Package validation allowlists those exact files and checks that every manifest matches the package name and version before the archive can pass.

# Compatibility boundary

This change guarantees discovery and distribution of the canonical Agent Skills packages in the three verified hosts. It does not promise that every optional host-specific capability mentioned by an individual workflow exists in every agent. Provider-specific metadata such as `agents/openai.yaml` remains optional metadata layered on the portable `SKILL.md` package.

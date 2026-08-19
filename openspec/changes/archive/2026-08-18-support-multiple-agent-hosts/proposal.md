# Why

Arcantry 1.0.0 currently stores and links portable Agent Skills only through `.agents/skills` and publishes only a Codex plugin manifest. Claude Code requires `.claude/skills` or a Claude plugin, while Gemini CLI supports its own skill directory and extension format. The canonical skills are portable, but the current distribution surfaces do not make that portability usable.

# What changes

- Add explicit Codex, Claude Code, and Gemini CLI target profiles to skill link, unlink, and doctor commands.
- Generate a Claude Code plugin manifest and a Gemini CLI extension manifest from the same Arcantry identity and `1.0.0` version used by the Codex plugin.
- Include the host manifests in the verified package projection.
- Document individual-skill and complete-catalog workflows for the supported hosts.
- Keep the existing `.agents/skills` behavior as the default when no host is selected.

# Out of scope

- Supporting provider-specific hooks, subagents, MCP servers, or repository instruction files.
- Claiming compatibility for hosts whose skill locations and package formats are not verified.
- Publishing a plugin, extension, npm package, tag, or release.
- Promoting the product beyond the pending `1.0.0` release.

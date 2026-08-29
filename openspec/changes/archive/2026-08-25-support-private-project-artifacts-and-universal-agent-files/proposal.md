# Why

Arcantry currently labels the standard `AGENTS.md` and `.agents/skills` surfaces as Codex-specific, while treating branded agent directories as equivalent sources. It also requires explicit configuration for private OpenSpec and changelog sources even though private todo guidance is discovered automatically. The unreleased 1.0 candidate therefore documents a provider model and privacy boundary that no longer match the intended product contract.

# What changes

- Treat `AGENTS.md` and `.agents/skills` as the universal repository guidance and Agent Skills surfaces.
- Keep Claude Code support as an optional compatibility adapter that imports universal guidance and links the same canonical skills without copying them.
- Discover shared and private OpenSpec, changelog, todo and skill sources independently, while preventing shared release history from depending on private intent.
- Keep one canonical package for each skill, reject different sources with the same identity, and make all catalog descriptions distinct and provider-neutral.
- Remove Gemini and the symmetric `--agent` model from the unreleased 1.0 product, package and documentation.

# Out of scope

- Removing or consolidating any of the 18 current public skills.
- Making skills an authoritative project knowledge source.
- Automatically creating branded Claude files without an explicit compatibility request.
- Publishing, tagging or releasing 1.0.0.
- Expanding the active native Rust CLI migration beyond direct conformance with this contract.

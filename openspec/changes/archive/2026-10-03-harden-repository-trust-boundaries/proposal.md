# Why

Repository-controlled paths and text cross several trust boundaries before they reach filesystem operations, release tooling or a terminal. Those boundaries need link-aware enforcement and bounded processing so an inspected repository cannot redirect reads or writes to unrelated host state, exhaust resources or emit terminal control sequences.

# What changes

- Keep auto-discovered project roots, repository-scoped skill targets and repository reads inside their trusted project boundary, including through links and junctions.
- Preserve explicitly supplied external configuration as the only authority for external sources.
- Make dependency-cycle validation linear, changelog output bounded and human terminal output control-safe.
- Run the Linux release smoke test from an immutable, isolated container without write access to verified artifacts.

# Out of scope

- Changing JSON or MCP response schemas.
- Removing support for explicitly configured external project sources.
- Changing the product version independently of the authorized first public release.

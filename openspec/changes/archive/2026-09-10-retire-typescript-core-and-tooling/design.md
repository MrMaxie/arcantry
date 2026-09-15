# Approach

Use `arcantry-core` as the sole owner of product rules, parsing, validation, planning and filesystem models. Keep `arcantry-cli` responsible for command parsing and human or machine rendering over that core. Extend `xtask` as the repository automation boundary for generated artifacts, catalog verification, documentation-output checks, coverage processing, package assembly, smoke tests and publication preparation. Rust tooling may invoke an ecosystem command such as Astro, npm or cargo when that external tool owns the boundary, but TypeScript must not remain as Arcantry's orchestration or policy layer.

Keep `apps/docs` as an explicit exception. Its Astro configuration and content integration may use TypeScript, but product rules and generated source authority must remain in Rust. The npm package may contain a minimal JavaScript launcher that selects and executes the exact optional native package. It must not expose a library API, parse project state, implement commands or reproduce core behavior.

Migrate by capability rather than by file extension alone. For each slice, establish or reuse independent Rust contract evidence, switch every repository and package caller to the Rust owner, then remove the superseded TypeScript implementation and tests in the same slice. A shrinking inventory prevents new TypeScript outside `apps/docs` and reaches an allowlist containing only the documentation files.

Deliver the active deterministic context-discovery change as the first slice. It adds immediate CLI and skill value while proving the intended ownership path through `repo inspect`, `arcantry-core` and native JSON output.

# Trade-offs

Removing the JavaScript library exports is a deliberate breaking change for programmatic npm consumers. Keeping compatibility wrappers would preserve the divided implementation model and create an open-ended binding obligation, so the npm package remains only a native CLI distribution route.

The migration temporarily retains TypeScript files while their complete Rust slices are built. A shrinking explicit inventory and a prohibition on new TypeScript outside documentation bound that transition without requiring one unsafe repository-wide rewrite.

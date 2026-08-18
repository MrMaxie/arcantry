# Approach

Model each project artifact as a named source with a kind, path, management level, versioned adapter and optional dependencies. Source dependencies form a directed acyclic graph. OpenSpec sources are authorities, changelog sources are projections and todo.txt sources are independent queues.

Resolve configuration in this order: explicit `--config`, the nearest ancestor `arcantry.toml`, then discovery-only defaults. A config file may identify its project root relative to its own location; explicit `--cwd` remains authoritative. Unconfigured discovered sources default to observation.

Publish the configuration contract as TOML Schema 1.0.0 in `schemas/arcantry-config-v1.tosd`. Runtime parsing uses the same value constraints through Zod after `smol-toml` parsing. The optional reserved `[toml-schema]` table locates the public schema and is not application data.

Separate tool compatibility, configuration version and adapter version. A newer CLI reads supported older adapters without rewriting their data. Transitions are explicit plans: preserve, adopt, rebind, cutover, migrate or relocate. Plans capture input hashes and application rejects drift before writing.

Keep the current repository adoption commands as compatibility shims. They no longer create `.docs/`; `--docs none` remains accepted while other legacy values fail before writes.

# Trade-offs

A versioned adapter registry and two-phase changes add more code than binary greenfield/brownfield modes. The additional model is justified because compatibility and ownership are independent per source. Hierarchical config merging and automatic migrations remain excluded to keep behavior inspectable.

TOML Schema does not replace runtime validation. The `.tosd` file is the portable editor and tool contract, while runtime validation enforces cross-field rules such as graph acyclicity and source relationships.

# Tasks

- [ ] Inventory every Rust, schema, catalog, generated-package launcher, CLI-contract and documentation surface that enumerates project source kinds, adapters, standard locations or Arcantry-owned environment access.
- [ ] Add `environment-schema` and `env-spec@1` configuration contracts to the native Rust CLI.
- [ ] Discover `.env.schema` and `.local/.env.schema` with stable source ids, normal visibility rules and no recursive environment-file inspection.
- [ ] Enforce observation-only management and allow explicit registration of an existing contract without creating, editing, relocating or deleting it.
- [ ] Extend human and JSON inspection output while preventing schema contents, defaults, resolver references and resolved values from entering output or diagnostics.
- [ ] Keep repository initialization, validation and doctor commands independent of Varlock executables, plugins, caches, secret providers and network access.
- [ ] Add a shared environment-loading contract for Rust entry points that preserves process values, attempts a compatible existing Varlock setup, falls back to project-root `.env` loading and never terminates the host before fallback.
- [ ] Add regression fixtures covering missing schemas, shared and private schemas, configured custom paths, unsupported management levels, neighboring value files containing secret sentinels, unavailable or incompatible Varlock versions, Varlock resolution failure, process-value precedence and dotenv fallback.
- [ ] Verify that optional Varlock failures do not fail environment-independent operations and that environment-dependent operations fail only for inputs still required after fallback.
- [ ] Update adoption, configuration, repository-contract and CLI documentation with the Arcantry and Varlock ownership boundary and the non-blocking dotenv fallback.
- [ ] Reconcile the supported-source inventory with `add-deterministic-context-discovery` and verify every applicable candidate identified by the implementation inventory.
- [ ] Run focused source and CLI tests, native conformance, `just check` and strict OpenSpec validation.
- [ ] Review `release.md` against what was actually delivered.

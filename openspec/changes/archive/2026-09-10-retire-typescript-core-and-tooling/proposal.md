# Why

Arcantry exposes a native Rust CLI, but product logic and repository automation still remain in a second TypeScript implementation. Contributors must maintain overlapping Rust and TypeScript behavior, the npm package still presents a JavaScript library surface, and routine generation, validation, packaging and release preparation depend on TypeScript entrypoints. This divides authority and directs effort into preserving a migration state that the product no longer intends to keep.

# What changes

- Make Rust the only implementation authority for Arcantry core behavior, the public CLI and repository tooling.
- Retire the TypeScript library implementation and its public npm subpath exports.
- Keep the npm package as a minimal platform launcher for the native executable without product logic.
- Move generation, validation, release preparation, packaging, smoke orchestration and other repository automation into Rust-owned commands.
- Migrate in complete vertical slices, starting with deterministic repository context discovery, and remove each superseded TypeScript owner after its Rust replacement is verified.
- Keep Astro and TypeScript inside `apps/docs` as the documentation application boundary.

# Out of scope

- Replacing Astro or removing TypeScript from `apps/docs`.
- Changing public CLI behavior except where a separate accepted OpenSpec change requires it.
- Adding a second long-lived implementation or JavaScript binding layer over Rust.
- Changing version `1.0.0`, cutting a release, adding release rehearsal, tagging, publishing, committing or pushing.

---
title: Adoption
description: Add the Arcantry repository contract without replacing your build system.
---

Arcantry sits above the project's native build system. It does not replace Cargo, CMake, pnpm, Gradle or another domain tool.

## Minimum adoption

1. Add `mise.toml` and pin the tools shared by contributors and CI.
2. Add a `justfile` with `setup`, `check`, `build` and `ci`.
3. Initialize OpenSpec with the Arcantry schema.
4. Require a `release.md` artifact for every change.
5. Generate release history only from archived changes.

```text
mise install
just setup
just check
```

## Keep project-specific commands behind `just`

The implementation below `just check` is intentionally project-specific. A TypeScript package may run type checking and Vitest; a Rust project may run `cargo fmt`, Clippy and tests. The repository-level contract remains the same.

## Do not migrate commit history into release history

Existing tags and changelogs can remain historical records. Start using OpenSpec-derived releases from the first Arcantry-managed version. Do not manufacture OpenSpec changes from old commits unless the original intent can be recovered reliably.

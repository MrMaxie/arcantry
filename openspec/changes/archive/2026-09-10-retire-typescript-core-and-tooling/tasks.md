# Tasks

- [x] Inventory every authored TypeScript file and classify it as documentation-owned or scheduled for Rust replacement.
- [x] Add a shrinking TypeScript-boundary check that rejects unlisted TypeScript outside `apps/docs` and ends with only the two documentation files allowed.
- [x] Deliver deterministic repository context discovery entirely through `arcantry-core` and `arcantry-cli` without adding TypeScript.
- [x] Move remaining catalog, configuration, repository, todo, source-transition, project-plan and release behavior and tests from the npm library into Rust owners.
  - Rust release slice complete: legacy and canonical multi-outcome artifact parsing, SemVer planning, assignment validation, multi-source changelog rendering, version and changelog consistency, release sealing, publication state, and repository release command dispatch are covered by Rust tests.
- [x] Move catalog generation, documentation-output verification, version checks and repository setup orchestration from `tooling/*.ts` into `xtask`.
  - Rust slices complete: CLI documentation contracts, catalog/package validation, catalog and documentation projection generation, standard release version sourcing, repository release adapter and recipe dispatch, built documentation output verification, and CI environment setup.
- [x] Move native package assembly, installer and registry smoke tests, release preparation, publication checks and coverage processing from `tooling/*.ts` into `xtask`.
  - Rust slices complete: npm package identity and projection checks, native target metadata, native npm package assembly and smoke orchestration, local registry and release installer smoke orchestration, sealed publication validation, retry-safe npm preflight and publication, GitHub Release publication, instrumented coverage collection, LCOV policy parsing, production-file inventory, per-file coverage enforcement, and coverage reporting.
- [x] Convert the `arcantry` npm package to a launcher-only package, remove public JavaScript library exports and type declarations, and remove tsup and package-level TypeScript tests.
- [x] Route `justfile` and GitHub workflows through Rust tooling while retaining Nub and Astro only for the documentation and npm ecosystem boundaries that still require them.
- [x] Remove every superseded TypeScript file and dependency outside `apps/docs`, then repeat the inventory and mark every candidate compliant or explicitly out of scope.
- [x] Run focused Rust tests, native CLI contract tests, package smoke tests, `just check` and strict OpenSpec validation.
- [x] Review `release.md` against what was actually delivered.

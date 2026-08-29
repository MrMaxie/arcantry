# Why

Arcantry currently assumes that one repository has one version, one release manifest stream and one managed changelog. Repositories that ship independently versioned modules or a product assembled from separately released units cannot represent their release state without flattening ownership or maintaining parallel release tooling outside Arcantry.

# What changes

- Add an opt-in `openspec-release@2` adapter with `single`, `independent` and `composed` release topologies while preserving `openspec-release@1` behavior.
- Let multi-unit projects assign OpenSpec outcomes to release units through explicit source and component selectors.
- Add schema-aware classification so schemas without a release artifact are skipped intentionally instead of requiring `impact: none` metadata.
- Add unit-scoped release manifests, changelogs, version sources, dependency pins and CLI operations.
- Keep parent releases explicit: a child release does not bump its parents, and a parent adopts a newer direct dependency only through acknowledged OpenSpec metadata.
- Keep TypeScript and native Rust behavior and public JSON output aligned.

# Out of scope

- Automatic migration or reconstruction of `openspec-release@1` history.
- Cross-repository release-unit dependencies.
- Candidate, accept, publish, provider or snapshot workflows.
- Implicit commits, tags, pushes, GitHub Releases or package publication.

# Approach

`openspec-release@2` is a tagged configuration family. Its `single` topology retains the existing flat release fields and defaults when `topology` is omitted. `independent` and `composed` use named `[release.units.<id>]` tables. Every unit owns its manifest path, managed changelog, version sources and tag prefix. Selectors bind units to configured OpenSpec source ids and optionally to disjoint component ids.

Multi-unit validation first resolves the source schema for every active and archived change from `.openspec.yaml`, falling back to the source `config.yaml`. It then loads `<source>/schemas/<schema>/schema.yaml`. A schema that generates `release.md` is release-bearing and requires a valid release artifact. A schema without that artifact is non-release and must not contain `release.md`. Unknown schemas and release artifacts generated at unsupported paths are errors.

Release artifacts retain one shared title, body, category and visibility. The scalar `impact` is the fallback for every matched unit, while `unit_impacts` may override SemVer impact per unit. `dependency_updates` acknowledges which direct dependencies a selected parent outcome adopts. A materially different consumer story remains a separate OpenSpec outcome rather than per-unit prose in YAML.

Format 2 manifests are unit-scoped and uniquely assign `(unit, change)` pairs. The same change may therefore ship from several units at different times. A composed manifest pins the latest released version of every direct dependency. Planning reports newer dependency versions separately, but cutting a parent adopts them only when one of the selected parent changes acknowledges the dependency. Parent SemVer is computed only from explicit parent release metadata.

Multi-unit `baseline`, `plan`, `cut` and `render` require `--unit`. Unscoped `check` validates the entire release system. `check --sealed` requires a unit and validates only that unit's work boundary plus its composed dependency state at the same commit. Publication remains an external action bound to the exact tagged commit.

# Trade-offs

The v2 adapter is explicit and does not reinterpret v1 configuration or manifests. This duplicates a small compatibility surface but prevents existing projects from changing meaning. Exact dependency pins and explicit acknowledgements add metadata, but avoid surprise parent releases and make composed history reproducible.

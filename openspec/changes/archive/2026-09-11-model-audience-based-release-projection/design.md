# Approach

Use `openspec-release@1` as the single final adapter. Fold single, independent and composed topologies into that contract and remove the unpublished `openspec-release@2` configuration option. Historical release manifest formats remain readable, but no public adapter migration workflow is introduced.

Release metadata separates SemVer impact, affected components, intended audiences and observable impact. `audiences` contains project-defined slugs. `observable_impact` is one of `customer-outcome`, `user-felt`, `significant-technical` or `maintenance`. A separate `changelog: include|omit` policy decides whether a change participates in changelog projection without changing manifest assignment or SemVer calculation.

An included change may expose its own outcomes or join an explicit projection group stored under `release-groups/<id>.yaml`. The group itself owns category, audiences, title, body and ordered member ids. Rendering emits one entry and records traceability to every member. Omitted changes remain release-bearing, assigned to manifests and included in version calculation and validation.

# Trade-offs

Independent classification dimensions are more expressive but add metadata and validation rules. Keeping one adapter removes prepublication compatibility cost, while explicit projection groups prevent heuristic consolidation from changing release meaning.

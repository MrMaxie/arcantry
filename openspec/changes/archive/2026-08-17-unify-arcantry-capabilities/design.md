# Approach

Keep each reusable skill self-contained under `skills/<name>`. Its instructions, metadata, assets, references, scripts, and declared tool dependencies form the canonical package. Generate catalog entries, plugin registration, and per-skill documentation from those packages instead of maintaining authored copies.

Expose one binary with two stable command groups:

- `arcantry repo init|update|doctor|validate|remove`
- `arcantry skills list|inspect|link|unlink|doctor`

Repository adoption uses three explicit information layers. `.local/` contains private machine-local execution state and is excluded through `.git/info/exclude`. `.docs/` contains durable project guidance that a repository may share. OpenSpec contains accepted product and engineering intent, observable requirements, delivery tasks, and release metadata.

Mutation is ownership-aware. Initialization and update preserve unowned files and user-editable content; removal targets only verified Arcantry-managed artifacts and requires an explicit command. Diagnostic commands do not mutate the repository. Remote systems remain read-only unless a skill declares the dependency and the user authorizes the exact write.

Arcantry dogfoods the contract by routing its own local and CI verification through the same CLI commands, schemas, package validation, and repository rules available to consumers.

# Trade-offs

A single namespaced CLI is more verbose than top-level aliases, but it makes repository and skill operations unambiguous and leaves room for additional capability groups without multiplying binaries.

Separating `.docs/` from OpenSpec creates two durable surfaces. The distinction is intentional: `.docs/` explains the project as it exists, while OpenSpec governs accepted change intent and release history.

Generated discovery surfaces constrain presentation to canonical metadata, but prevent plugin, catalog, and per-skill documentation drift. Authored documentation remains appropriate for cross-cutting journeys such as adoption, repository workflow, and CLI use.

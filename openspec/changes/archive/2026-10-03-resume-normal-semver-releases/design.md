# Approach

The repository release adapter remains the single owner of product version changes. Its plan covers the Rust workspace, internal Rust dependency pins, lockfile, main npm package, platform npm packages and plugin manifests.

A version becomes externally immutable when the main npm package or public GitHub Release exists. Before that boundary, an explicitly authorized first-release reseal may replace a failed tag and draft artifacts. Partial platform publication remains a same-integrity retry only.

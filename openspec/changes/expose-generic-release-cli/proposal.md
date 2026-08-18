# Why

Arcantry already contains an OpenSpec-backed release model, but only repository-specific tooling can call it. Adopted projects need a configuration-driven local CLI that can establish a brownfield baseline, plan the next version, render release history and validate consistency without coupling Arcantry to their CI or publication workflow.

# What changes

- Add optional release configuration for manifest, changelog, repository link, tag prefix and version-source adapters.
- Add `release baseline`, `release plan`, `release cut`, `release render` and `release check` commands.
- Make every mutating release command preview a drift-checked plan by default and require `--apply` before writing.
- Support JSON package versions and Cargo workspace package versions through explicit adapters.
- Allow a baseline manifest with no assigned OpenSpec changes and keep internal change entries out of the public changelog.

# Out of scope

- Committing, tagging, pushing, publishing packages or creating releases.
- Adding project CI integration.
- Inferring release entries from Git history.
- Updating arbitrary Cargo package versions outside `[workspace.package]`.

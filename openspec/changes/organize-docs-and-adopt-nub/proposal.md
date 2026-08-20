# Why

The documentation application, repository tooling and package workspace currently share the repository root, while dependency installation and repository scripts depend on pnpm and mise beneath the root `justfile` helper. Generated documentation projections are also committed beside authored content. This makes ownership unclear and allows generated output to drift from canonical skill and schema sources.

# What changes

- Move the Astro and Starlight application into an `apps/docs` workspace while preserving its routes, content and visual contract.
- Use mise to pin and provision `just` and Nub, retain the root `justfile` as the task runner, and use Nub as the package manager, Node provider and tool runner.
- Generate documentation-only projections before local and CI documentation work without tracking them in Git.
- Apply Biome formatting and linting to authored Astro application code and repository tooling.

# Out of scope

- Changing the public Arcantry CLI, library API or supported package launchers.
- Redesigning documentation content, styling, animation or navigation.
- Removing generated plugin manifests from the tracked and published package.
- Releasing, tagging or publishing Arcantry.

<!-- arcantry:start -->
## Arcantry

Use `openspec/` as the only source of product and engineering specifications.
Read `.local/arcantry.toml` when present for private operational configuration.
<!-- arcantry:end -->

## Implementation coverage

When an OpenSpec requirement applies to a class of files, components, or pages, inventory the matching candidates from the repository before implementation and again before completion. Mark each candidate compliant or explicitly out of scope; do not treat named examples as the complete surface unless the requirement limits them.

## Delivery

- Implement and commit coherent, verified increments using the repository's protected-branch workflow. Deliver changes to `master` through a pull request when branch protection requires it.
- Keep existing local commits and unrelated work intact. Pushes and remote settings changes require separate authorization.
- Pull requests and `master` run the repository CI workflow. Documentation changes also run the Pages workflow. Protected release tags run the native release workflow.
- Use `just check-fast`, `just check-host`, and `just linux-system-test` locally before delivery; remote checks complement rather than replace local evidence.
- Coverage is an optional diagnostic, not an implementation or publication gate.
- Prefer concrete Rust code and established dependencies over new frameworks. Compare the total maintenance cost before adding infrastructure.

## Versioning and release authorization

- Derive product and distributable versions from accepted OpenSpec release impact through the repository release workflow. Keep every version source and dependency pin aligned in the same release plan.
- Finalize the unpublished first public version in place while neither its main npm package nor a public GitHub Release exists. A failed tag or draft release may be replaced only as part of an explicitly authorized reseal.
- Treat a live npm version or public GitHub Release as immutable. Later work must use the next version computed by the configured strategy instead of changing published release state.
- Do not cut or seal a release, change release manifests or release changelog headings, create or replace a version tag, create a GitHub Release, or publish packages or versioned release artifacts without explicit user authorization for that release action.
- Treat updates to `master` and deployments to GitHub Pages as normal continuous delivery. Once the underlying commit, push or merge is authorized, update them whenever the product or documentation requires it; no separate release approval is needed.

## MVP complexity budget

Prefer concrete functions, types and existing project conventions. Before adding infrastructure, inspect maintained ecosystem tools and compare dependency plus integration against custom implementation, tests and ongoing maintenance. Remove a replaced mechanism instead of adding a parallel one. Compare meaningful alternatives without mandatory option counts, scores or a fixed presentation template. Report measurements with their conditions. A missing CLI does not block direct work on source files. Current user authorization overrides older process advice; do not ask for it again.

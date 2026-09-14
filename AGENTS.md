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

## Continuous 1.0 delivery and release authorization

- Maintain the implementation, documentation, validation, packaging, and product-facing version references as a complete, release-ready 1.0 product.
- Treat evidence-led private audits as diagnostic work, not product positioning: report verified defects and maturity risks directly without changing the 1.0 posture. In product-facing or externally shared artifacts, do not describe the product as a draft, release candidate, or incomplete pre-release while this policy is active.
- Keep every Arcantry product and distributable version value at `1.0.0` until the user explicitly authorizes a version change. Continue improving the product under that unchanged version; readiness, elapsed work, merged changes, or successful validation never imply permission to bump it.
- Do not cut or seal a release, change release manifests or release changelog headings, create or push a version tag, create a GitHub Release, or publish packages or versioned release artifacts without explicit user authorization for that release action.
- Treat updates to the `master` branch and deployments to GitHub Pages as normal continuous delivery, not as release, tagging, or package-publication actions. Once the underlying commit, push, or merge is authorized, update `master` and Pages whenever the product or documentation requires it; no separate release approval is needed.

## MVP complexity budget

Prefer concrete functions, types and existing project conventions. Before adding infrastructure, inspect maintained ecosystem tools and compare dependency plus integration against custom implementation, tests and ongoing maintenance. Remove a replaced mechanism instead of adding a parallel one. Compare meaningful alternatives without mandatory option counts, scores or a fixed presentation template. Report measurements with their conditions. A missing CLI does not block direct work on source files. Current user authorization overrides older process advice; do not ask for it again.

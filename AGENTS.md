<!-- arcantry:start -->
## Arcantry

Use `openspec/` as the only source of product and engineering specifications.
Read `.local/arcantry.toml` when present for private operational configuration.
<!-- arcantry:end -->

## Implementation coverage

When an OpenSpec requirement applies to a class of files, components, or pages, inventory the matching candidates from the repository before implementation and again before completion. Mark each candidate compliant or explicitly out of scope; do not treat named examples as the complete surface unless the requirement limits them.

## Branches and pull requests

- Name branches `<type>/<english-kebab-case-description>`, for example `feat/ship-native-rust-cli`, `fix/validate-release-seal`, or `chore/update-github-actions`.
- Use a Conventional Commit type such as `feat`, `fix`, `chore`, `docs`, `refactor`, or `test`. The description must identify the concrete outcome or content of the branch. Do not use workflow-state or catch-all names such as `current-master-updates`, `changes`, or `misc`.
- Title pull requests `<type>: <English description>`, using the same type as the branch. The title must describe the concrete outcome rather than the act of opening, updating, or preparing a pull request.
- Keep each branch and pull request focused on one coherent outcome. The pull request description must explain what changes, why it is needed, and how it was verified. It must match the actual diff and must not consist only of a restated title or generic wording such as "updates" or "improvements".
- Use `.github/pull_request_template.md` for every pull request. Remove placeholder guidance and record skipped verification as `Not run: <reason>`.

## Continuous 1.0 delivery and release authorization

- Maintain the implementation, documentation, validation, packaging, and product-facing version references as a complete, release-ready 1.0 product.
- Treat evidence-led private audits as diagnostic work, not product positioning: report verified defects and maturity risks directly without changing the 1.0 posture. In product-facing or externally shared artifacts, do not describe the product as a draft, release candidate, or incomplete pre-release while this policy is active.
- Keep every Arcantry product and distributable version value at `1.0.0` until the user explicitly authorizes a version change. Continue improving the product under that unchanged version; readiness, elapsed work, merged changes, or successful validation never imply permission to bump it.
- Do not cut or seal a release, change release manifests or release changelog headings, create or push a version tag, create a GitHub Release, or publish packages or versioned release artifacts without explicit user authorization for that release action.
- Treat updates to the `master` branch and deployments to GitHub Pages as normal continuous delivery, not as release, tagging, or package-publication actions. Once the underlying commit, push, or merge is authorized, update `master` and Pages whenever the product or documentation requires it; no separate release approval is needed.

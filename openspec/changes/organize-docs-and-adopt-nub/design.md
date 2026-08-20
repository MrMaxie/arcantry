# Approach

The repository remains one JavaScript workspace. Its root owns `package.json`, `nub.lock`, the Node pin, `mise.toml`, `justfile`, repository scripts, Biome configuration and cross-package tooling. The deployable documentation site becomes the private `@arcantry/docs` workspace under `apps/docs`; the distributable library and CLI remain under `packages/arcantry`.

`apps/docs/astro.config.ts` contains both Astro and Expressive Code settings. The generated skill pages are written below `apps/docs/src/content/docs/_generated/skills`, and a custom Starlight content id generator removes `_generated/` so existing `/skills/*` routes remain stable. The generated sidebar and public schema copy stay inside the docs workspace. These outputs are ignored by Git and recreated by every docs development, check and build entrypoint.

The generator distinguishes tracked package projections from ephemeral documentation projections. Check mode validates tracked plugin manifests without requiring ephemeral files to exist. Normal generation writes both sets; docs generation writes only the ignored documentation outputs.

mise pins and provisions `just` and Nub. Nub owns the workspace and lockfile, provisions the Node version and runs TypeScript and workspace tools directly. The root `justfile` is the stable task runner and invokes tools through Nub without routing through root package scripts. GitHub workflows use mise as the single tool bootstrap before `just ci-setup` installs dependencies and exposes Nub's pinned Node to later workflow steps, then run `just ci`. npm remains only for the final npm Trusted Publishing mutation because that authentication flow is implemented by npm CLI.

Biome checks only authored TypeScript, JavaScript, CSS and Astro files in `apps/docs` and `tooling`. Markdown, MDX, package sources and generated outputs remain outside its scope.

# Trade-offs

The repository root still contains the minimal files required to coordinate one workspace, including `mise.toml` and the contributor-facing `justfile`. Keeping one root lockfile avoids dependency drift and duplicated installation state. Astro formatting is experimental in Biome 2.5.9, so the initial mechanical rewrite is reviewed with Astro checks, production builds and visual smoke tests while generated and product package sources remain excluded.

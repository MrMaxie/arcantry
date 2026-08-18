# Configuration and project boundary

Arcantry uses one TOML schema for both `arcantry.toml` and `.local/arcantry.toml`. An explicit `--config` path has highest precedence. Without it, discovery walks from the requested directory toward its ancestors and checks `.local/arcantry.toml` before `arcantry.toml` at each directory. The first match is authoritative and no files are merged.

The project root for `.local/arcantry.toml` is the directory containing `.local`, not the `.local` directory itself. Diagnostics expose the active configuration and any sibling configuration shadowed at the same project boundary. Repository and private sources remain independent and may be reconciled only by an explicit inspected plan and apply step.

# Minimal repository adoption

`repo init`, `repo update`, and `repo remove` require `--scope shared|private`. Shared scope manages `arcantry.toml` and an Arcantry section in the repository `AGENTS.md`. Private scope manages `.local/arcantry.toml` and `.local/AGENTS.md`, and ensures `.local/` is excluded through `.git/info/exclude` when Git is present. These commands do not create package manifests, task runners, runtime configuration, OpenSpec sources, or other project scaffolding.

Managed guidance uses explicit markers so updates and removals can verify ownership. Existing content outside a managed section remains untouched. Read-only commands use normal configuration resolution and report shadowing rather than repairing it.

# Skill catalog and distribution

Every catalog entry declares exactly one family: `self-improvement`, `repo-safely`, or `content-safely`. Generated catalog JSON and documentation derive that family from canonical metadata. The catalog contains focused capabilities, not provider-specific intake flows or forwarding-only packages.

The built-in linker installs one canonical local skill into `~/.agents/skills` for user scope or `<repo>/.agents/skills` for repository scope. `--target` remains an advanced explicit destination and cannot be combined with `--scope`. Linking and unlinking remain idempotent and validate exact ownership. Other compatible skill installers may be documented as alternatives but are not runtime dependencies.

# Documentation and dogfooding

Documentation presents the final Arcantry contract directly. Content explains the three skill families, independent project knowledge sources, local-first configuration, minimal adoption, and verification layers. Existing layout, components, typography, animation, and visual styling remain unchanged.

Arcantry's quality gate builds the public CLI, initializes private repository state through that CLI, and runs public repository and skill diagnostics. Private dogfooding state remains excluded from Git.

# Release projection

All public package, catalog, and generated documentation versions derive from the `1.0.0` release manifest. Consumer-facing release text describes product outcomes from OpenSpec release artifacts rather than Git history.

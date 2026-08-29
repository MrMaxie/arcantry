# Approach

An optional `[release]` block in `arcantry.toml` selects the OpenSpec release adapter, manifest directory, managed changelog source, tag prefix, repository URL and an ordered list of version-source adapters. The changelog source identifies its OpenSpec authorities through its existing `from` relationship, so release configuration does not duplicate source ownership.

The CLI resolves this configuration into the existing release model. `baseline`, `cut` and `render` produce normal drift-checked `ProjectPlan` operations. They print the plan by default and call the existing atomic apply path only with `--apply`. `plan` and `check` never write.

A baseline manifest has `baseline: true` and may contain no changes. It anchors an existing version and date without inventing historical public entries. Later manifests retain the existing non-empty change rule. Internal archived changes affect SemVer planning and manifest assignment but are omitted from the public changelog.

Version sources use named adapters. `json-package@1` updates a top-level JSON `version`. `cargo-workspace@1` reads and replaces exactly one `version` entry inside `[workspace.package]` and refuses ambiguous or absent sections.

Normal `release check` validates semantic consistency while allowing active and unassigned work. `--sealed` additionally applies the existing final release-seal checks. Neither mode publishes anything.

# Trade-offs

Release configuration is explicit rather than inferred from repository language. This adds a small amount of TOML but prevents accidental edits to unrelated manifests and keeps non-Node projects supported without language-specific initialization.

# Why

Arcantry needs one coherent product contract for coordinating project knowledge, repository maintenance, reusable agent workflows, and audience-safe content. The command line, skill catalog, documentation, and Arcantry's own repository must expose the same provider-neutral model without imposing unrelated tools or repository scaffolding.

# What changes

- Define shared and private TOML configuration with deterministic local-first discovery and no implicit merging.
- Make repository initialization a minimal, scope-aware bootstrap for configuration and managed agent guidance.
- Organize the public catalog into self-improvement, repo-safely, and content-safely families with focused skills for recurring project work.
- Keep skill distribution portable through standard agent skill locations and a narrow local linking helper.
- Align public documentation, generated catalog projections, validation, and Arcantry dogfooding with the same contract.
- Establish `1.0.0` as the product release represented by package metadata and release projections.

# Out of scope

- Automatic synchronization or merging between shared and private sources.
- Remote provider integrations or mandatory issue-tracker services.
- Package-manager, task-runner, runtime, or product-source scaffolding during repository initialization.
- A visual redesign of the documentation site.
- Publishing packages, creating tags, or modifying remote repositories.

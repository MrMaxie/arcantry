# Why

Repository adoption currently assumes a Git repository, a private JSON configuration and a fixed Node-oriented scaffold. This prevents Arcantry from safely serving projects that need only one capability, must preserve an established artifact or cannot leave Arcantry-specific state in the repository.

# What changes

Introduce a language-neutral project knowledge model whose OpenSpec, changelog and todo.txt sources are discovered independently and may be ignored, observed, validated or managed. Add optional `arcantry.toml` configuration with a TOML Schema contract, explicit tool and adapter compatibility, read-only inspection, serializable transition plans and drift-safe application.

Keep OpenSpec authoritative for product and engineering meaning while treating changelog as a projection, todo.txt as an independent queue, `.local/` as a privacy boundary and skills as procedural capabilities. Stop creating or managing `.docs/`.

# Out of scope

- Automatic migration during an Arcantry upgrade.
- Deriving changelog meaning from commits or file diffs.
- External tracker, Backlog.md or worklog adapters.
- Automatic skill installation or project-specific package-manager setup.
- Deleting existing project-owned `.docs/` content.

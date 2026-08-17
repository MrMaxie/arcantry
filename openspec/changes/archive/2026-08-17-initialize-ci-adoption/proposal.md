# Why

Arcantry's public repository validator correctly requires private adoption configuration, but a clean CI checkout does not contain `.local/arcantry.json`. The full quality gate therefore passes release validation, tests, documentation and packaging before failing its final public self-check. Committing `.local` or weakening `repo validate` would violate the repository contract.

# What changes

- Initialize Arcantry's ephemeral private adoption state through the built public CLI before CI runs public repository validation.
- Keep initialization explicit, deterministic and idempotent for both clean CI checkouts and already-adopted local checkouts.
- Keep `repo validate` read-only and strict.
- Verify the complete `just ci` path in a clean checkout.

# Out of scope

- Committing `.local` configuration or changing its privacy boundary.
- Making repository validation infer or create missing configuration.
- Publishing a package, Git tag or GitHub Release.

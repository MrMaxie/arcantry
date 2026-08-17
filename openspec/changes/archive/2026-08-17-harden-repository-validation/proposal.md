# Why

Repository and catalog validation accepted some structurally present but outdated or unsupported content. Arcantry could therefore report a valid adoption while managed guidance had drifted, omit actionable doctor repairs, or accept metadata that did not match the published schemas. Its own CI also did not run the public repository and skill validation commands.

# What changes

- Detect outdated managed guidance, not only missing ownership markers.
- Keep `repo validate` deterministic and non-mutating while making `repo doctor` report explicit repair actions.
- Enforce canonical catalog and skill metadata schema references, field allowlists, identifiers and supported text lengths.
- Require non-empty release component metadata.
- Run the public repository and skill validation contracts in Arcantry CI.

# Out of scope

- Automatically repairing repository state during validation.
- Changing repository ownership or removal behavior.
- Adding new catalog fields or skill distribution surfaces.

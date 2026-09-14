# Why

Public skill packages currently inherit the Arcantry product version and embedded catalogs are materialized under that shared value. A skill cannot identify its own compatible revision, report local drift or move to a newer canonical snapshot without replacing shared cache state or updating the CLI.

# What changes

- Give every public skill an independent semantic version and deterministic content digest.
- Record installation provenance outside directories scanned as skills.
- Add local and remote status for one installed skill.
- Add previewed, drift-checked application of one exact skill update from Arcantry's official GitHub `master` or an explicit local catalog.
- Replace shared mutable catalog cache behavior with immutable per-skill snapshots.
- Generate a bounded history for each skill from accepted OpenSpec outcomes.

# Out of scope

- Updating private skills from a public remote.
- Automatically adopting unmanaged copies, overwriting local modifications or updating every skill together.
- Executing scripts from downloaded packages.
- Requiring Git or a JavaScript runtime for remote updates.
- Changing Arcantry version `1.0.0`, creating release tags, publishing packages or pushing commits.

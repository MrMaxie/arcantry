# Why

Arcantry currently validates only the release metadata that exists. The two commits after version 0.3.0 changed repository behavior without their own OpenSpec changes, version assignment or changelog entries, yet `release:check` still passes. This lets Arcantry bypass the lifecycle it requires from adopted repositories and makes a clean release check an incomplete statement about repository state.

# What changes

- Make archived OpenSpec intent mandatory for every completed product or engineering change, while allowing that intent to be recovered after implementation and before completion.
- Treat an internal SemVer release as the completion boundary even when no package, tag or GitHub release is published.
- Require each completed repository state to be sealed by its latest release manifest, distribution version and generated changelog.
- Use Git history only to detect repository changes after the latest release seal, never to infer changelog prose, category, impact or components.
- Recover the two changes made after 0.3.0 as separate behavior-level OpenSpec changes and include them with this lifecycle fix in version 0.3.1.
- Document and test the dogfooded workflow through the same commands used by CI.

# Out of scope

- Publishing an npm package, Git tag or GitHub Release.
- Rewriting existing Git history or changing released changelog entries.
- Requiring OpenSpec intent to be authored before implementation begins.
- Generating release meaning from commit messages or file diffs.

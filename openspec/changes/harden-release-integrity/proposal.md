# Why

Arcantry already derives release history from archived OpenSpec changes, but the generated state can still drift: a manifest may be malformed, a change may be assigned twice, or `CHANGELOG.md` may stop matching its sources.

# What changes

- Validate release manifests and change assignment as one release state.
- Add a check that fails when `CHANGELOG.md` is stale.
- Cut the planned SemVer release from archived, unassigned changes instead of hand-writing manifests.
- Keep changelog entries traceable to the archived OpenSpec change that produced them.

# Out of scope

- Inferring release meaning from Git commits.
- Publishing packages, tags or GitHub Releases.

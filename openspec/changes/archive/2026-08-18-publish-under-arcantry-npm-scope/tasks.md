# Tasks

- [x] Rename the combined package to `@arcantry/arcantry`, add canonical repository metadata and regenerate workspace dependency state.
- [x] Replace personal-scope references in repository commands, tests, package smoke checks and generated or authored public commands.
- [x] Make package verification retain and smoke-test one exact tarball for publication.
- [x] Add release-tag validation for the tag, newest release manifest, package version and release-seal commit.
- [x] Add a GitHub-hosted npm publication workflow with the protected `npm` environment, minimum permissions, a compatible npm CLI and OIDC trusted publishing.
- [x] Document the one-time 2FA bootstrap and the npm organization trusted-publisher configuration without storing credentials in the repository.
- [x] Verify the renamed package through build, type checks, tests, archive allowlist checks, packed installation and CLI and subpath import smoke tests.
- [x] Verify workflow failure paths for an invalid tag, version drift, an unsealed commit, a dirty or unexpected archive and an existing registry version.
- [x] Run strict OpenSpec validation and `just ci`.
  - Strict OpenSpec validation, types, 101 tests, catalog validation and generated-state checks pass. Before archive, `just ci` stops at the expected active-change release boundary; the complete gate is rerun after the release seal commit.
- [x] Review `release.md` against what was actually delivered.
- [x] Record the separately authorized first-publication and trusted-publisher operator handoff without performing npm publication in this change.

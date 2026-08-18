# Why

Arcantry's only npm package still uses a personal scope, and the package name is repeated across workspace commands, package smoke tests and public documentation. The repository also has no npm publication workflow, so moving to the `@arcantry/*` organization namespace without a release contract would leave naming, authentication and published contents vulnerable to drift.

# What changes

- Make `@arcantry/arcantry` the canonical name of the existing combined CLI and library package while preserving the `arcantry` binary and current exports.
- Replace personal-scope references in workspace tooling, package verification and public command examples with the organization-scoped identity.
- Add a release-tagged npm publication path that validates the sealed OpenSpec release, tests the exact package archive and publishes that archive as public.
- Bootstrap the previously unpublished package once with maintainer 2FA, then use npm trusted publishing from a protected GitHub Actions environment without a long-lived publish token.
- Keep package identity, release version, documentation and CI checks aligned through repository validation.

# Out of scope

- Splitting the combined package into `@arcantry/cli`, `@arcantry/core` or other packages.
- Publishing, redirecting or deprecating a compatibility package under `@maxiedev/*`.
- Publishing private npm packages.
- Creating GitHub Releases or deriving release meaning from Git tags.
- Adopting npm staged publishing in this change.

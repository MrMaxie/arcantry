# Why

The public package name repeats the product name as `@arcantry/arcantry`, which makes the primary launcher command unnecessarily long without adding a distinction for users. The package has not been published, so Arcantry can adopt the concise global name before creating a compatibility obligation.

# What changes

- Make `arcantry` the canonical name of the existing combined CLI and library package while preserving the `arcantry` binary and declared subpath exports.
- Align launcher examples, package validation, release checks and npm publication with the unscoped identity.
- Keep the package public, organization-managed and protected by the existing verified-archive and trusted-publishing controls.

# Out of scope

- Splitting the combined package into separate CLI and library packages.
- Publishing the package, cutting a release or creating a release tag.
- Reserving or publishing additional packages under `@arcantry/*`.

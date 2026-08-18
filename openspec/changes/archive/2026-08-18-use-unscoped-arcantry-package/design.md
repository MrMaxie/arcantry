# Approach

Treat `packages/arcantry/package.json` as the canonical npm identity and change its name from `@arcantry/arcantry` to `arcantry`. Rename the private root package to `arcantry-workspace` so pnpm name filters do not match both workspace projects. Retain the public package directory, binary, exports, version and verified archive flow. Existing smoke tests already derive the installed path and import specifiers from the manifest, so the unscoped name must continue through those paths without a special case.

Update authored launcher commands and identity assertions. Keep the publication workflow manifest-driven and continue publishing the verified archive through npm trusted publishing. The first-publication bootstrap and protected `npm` environment remain unchanged except that trusted-publisher configuration targets `arcantry`.

# Trade-offs

The unscoped name occupies the global npm namespace and can never be private. Arcantry is intended to be public, the name is currently unpublished, and npm organizations can govern unscoped packages, so the shorter launcher and import paths outweigh the loss of an organization-qualified namespace.

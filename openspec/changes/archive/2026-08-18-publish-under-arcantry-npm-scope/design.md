# Approach

Treat `packages/arcantry/package.json` as the canonical npm package identity. Rename the existing package to `@arcantry/arcantry`, retain its `arcantry` executable and subpath exports, and derive workspace filters, installed paths and smoke-test imports from the manifest where practical. Regenerate the lockfile and update public launcher examples so the personal scope is absent from shipped and documented surfaces.

Extend package verification to produce one retained tarball, inspect its allowlist and install that same tarball for CLI and export smoke tests. The publication job publishes the verified tarball rather than repacking mutable workspace state.

Add a GitHub Actions workflow triggered only by protected `v<version>` tags. Before publication it checks out the tag, installs the pinned repository toolchain, ensures npm is new enough for trusted publishing, runs the full repository and package verification, and proves that the tag version, newest release manifest, package version and release-seal commit are identical. The workflow uses a GitHub environment named `npm`, grants only `contents: read` and `id-token: write`, and publishes with `--access public` from a GitHub-hosted runner. No npm write token is stored in repository or environment secrets.

The first public version is a deliberate bootstrap exception because npm trusted publishers can only be configured after a package exists. A maintainer publishes the verified tarball from the exact sealed release with 2FA, configures `@arcantry/arcantry` to trust the repository's publication workflow and `npm` environment, and restricts later publication to that trusted publisher. Subsequent release tags use OIDC. Bootstrap publication and npm organization configuration remain explicit operator actions and require separate authorization when implementation reaches that boundary.

Git tags trigger distribution but do not define product meaning. OpenSpec release artifacts and the release manifest remain authoritative; a mismatched, unsealed or already-published version fails without changing npm state.

# Trade-offs

`@arcantry/arcantry` repeats the product name, but it is the smallest safe rename for the current package, which serves both CLI and library consumers. Naming it `@arcantry/cli` would misrepresent its exported API, while splitting packages would expand the migration before a real package boundary is established.

The one-time bootstrap cannot use OIDC or staged publishing because the npm package does not yet exist. Keeping it as a documented, 2FA-protected exception avoids introducing a reusable automation token solely for initial publication.

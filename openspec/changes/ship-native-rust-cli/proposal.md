# Why

The public Arcantry CLI currently executes JavaScript from the combined npm package and requires Node.js 24 even when a user only needs repository or skill commands. Arcantry needs a compiled, self-contained command-line distribution that works consistently across supported Windows, macOS and Linux systems without requiring a JavaScript or Python runtime.

# What changes

- Make a Rust executable the authoritative implementation behind the existing `arcantry` command surface while preserving command names, options, diagnostics, exit codes and filesystem behavior.
- Publish verified native archives for Windows, macOS and Linux on x64 and ARM64, using musl for Linux, from the same sealed release state as the npm packages.
- Keep the unscoped `arcantry` npm package as the JavaScript library and package-runner entrypoint, with exact optional platform packages supplying the native executable.
- Embed the public catalog, skills, schemas and templates in the executable and materialize a verified versioned catalog only when a command needs durable files for linking.
- Require cross-implementation conformance and native target smoke tests before the Rust CLI replaces the TypeScript CLI entrypoint.

# Out of scope

- Rewriting or removing the public JavaScript library and its declared subpath exports.
- Changing the public command hierarchy or repository, knowledge, todo and skill behavior.
- Supporting glibc-specific, 32-bit or additional operating-system targets in the first native release.
- Adding an automatic updater, operating-system package-manager formulae, binary signing or macOS notarization.
- Publishing packages, creating release tags or changing remote registry and repository settings while implementing this change.

# Approach

## CLI ownership and compatibility

Build the authoritative `arcantry` command implementation in Rust. Keep the current TypeScript CLI available only as a migration reference until a black-box conformance suite proves that both implementations produce equivalent command discovery, standard output, diagnostics, exit codes and filesystem results for the supported scenarios. Cut over the npm launcher and repository dogfooding only after that gate passes. Retain the existing JavaScript modules, type declarations and subpath exports; this change does not turn the native executable into a JavaScript binding layer.

The native executable must not require Node.js, Bun, Python or another language runtime. First-party CLI source remains Rust, while the existing TypeScript library and repository tooling remain TypeScript. Dependencies must support every declared target without adding a runtime dependency or first-party C or C++ source. Select the YAML implementation only after a focused spike proves compatibility with the existing configuration, metadata and release fixture corpus; record the selected crates and internal workspace layout in the later implementation plan.

## Embedded public assets

Embed the generated public catalog, canonical skills, schemas and OpenSpec templates in the executable at build time. Read directly from embedded content when a command needs bytes only. When `skills link` requires a durable directory, materialize the complete embedded catalog into an operating-system-standard per-user data directory namespaced by the Arcantry version.

Materialization writes a temporary sibling directory, validates an embedded manifest and file digests, then renames it atomically. A matching cache is reused. Arcantry may replace an invalid cache only when its ownership marker proves that Arcantry created it; unexpected unowned content must produce an actionable error instead of being overwritten. Skill links point to the exact versioned materialization so repeated link and unlink operations retain their ownership and idempotency guarantees. Automatic cleanup of older versioned caches is not part of this change.

## Native release artifacts

Build and execute release smoke tests for these six Rust targets:

- `x86_64-pc-windows-msvc`
- `aarch64-pc-windows-msvc`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-musl`

Attach one deterministic ZIP archive per Windows target and one deterministic TAR.GZ archive per macOS and Linux target to the GitHub Release for the sealed version. Name archives from the product version and Rust target triple, and publish one `SHA256SUMS` manifest covering every native archive. Each archive contains the target executable and the public license; runtime assets remain inside the executable.

## npm distribution

Keep `arcantry` as the unscoped public package. Its existing JavaScript and type exports remain unchanged, while its `bin` entry becomes a narrow JavaScript launcher that resolves and executes an exact-version optional dependency without downloading code at install or execution time.

Publish one scoped package per target:

- `@arcantry/cli-win32-x64`
- `@arcantry/cli-win32-arm64`
- `@arcantry/cli-darwin-x64`
- `@arcantry/cli-darwin-arm64`
- `@arcantry/cli-linux-x64-musl`
- `@arcantry/cli-linux-arm64-musl`

Each platform package declares the matching npm `os`, `cpu` and, for Linux, `libc` constraint and contains only its executable, manifest, README and license. The main package pins every platform package to its own exact version through `optionalDependencies`. The launcher maps `process.platform`, `process.arch` and Linux libc to one package, rejects unsupported combinations, and reports how to restore omitted optional dependencies or use the matching GitHub Release. It never performs a network fallback.

Build, allowlist, install and smoke-test all seven npm archives before registry mutation. Publish or confirm the exact platform package versions before publishing the main `arcantry` package, so the public launcher is never visible without its complete target set. A retry may treat an existing platform version as satisfied only when registry metadata and integrity match the verified archive; no package version may be overwritten. All npm packages continue to use protected trusted publishing after their explicit first-publication bootstrap.

## Release boundary

The GitHub tag, newest release manifest, JavaScript package, six platform packages, native archive versions, embedded product version and release-seal commit must agree before external publication. The release pipeline must execute each release candidate on its declared operating system and architecture rather than treating successful cross-compilation as proof of support.

# Trade-offs

Rust adds a second build toolchain and a six-target release matrix, but removes a runtime requirement from direct CLI use and provides one implementation suitable for all supported platforms.

Separate npm platform packages increase package and publication count. They avoid making every npm user download all six binaries and avoid runtime downloads, so the additional release coordination is accepted.

Embedding assets duplicates some content already present in the JavaScript package and requires a managed local materialization for symbolic links. It keeps direct releases self-contained and preserves the existing link contract without shipping a companion directory.

Keeping the JavaScript library preserves compatibility but temporarily leaves related behavior implemented in two languages. Shared conformance fixtures are the compatibility boundary; removing or replacing the JavaScript API requires a separate change.

# Approach

## CLI ownership and compatibility

Build the authoritative `arcantry` command implementation in Rust. Keep the current TypeScript CLI available only as a migration reference until a black-box conformance suite proves that both implementations produce equivalent command discovery, standard output, diagnostics, exit codes and filesystem results for the supported scenarios. Cut over the npm launcher and repository dogfooding only after that gate passes. Retain the existing JavaScript modules, type declarations and subpath exports; this change does not turn the native executable into a JavaScript binding layer.

The native executable must not require Node.js, Bun, Python or another language runtime. Use a Rust 2024 workspace with `arcantry-core` for domain behavior, `arcantry-cli` for the command surface and embedded assets, and `xtask` only for validation and Rust artifact assembly that Cargo or cargo-dist does not provide. Use `clap`, `serde`, `serde_json`, `toml_edit`, `todo-txt`, `semver`, `chrono`, `jsonschema`, `rust-embed`, `directories`, `sha2`, `tempfile`, `fs4`, `junction`, `duct`, `thiserror` and `anyhow`, with `assert_cmd`, `snapbox` and `walkdir` for tests. Use `serde-saphyr` only after a focused spike proves compatibility with the existing YAML fixture corpus. Dependencies must support every declared target without adding a runtime dependency or first-party C or C++ source.

Derive the complete command hierarchy, arguments and descriptions with `clap`. Compatibility formatting may adapt that metadata to the established output contract, but it must not duplicate the command definitions. Parse each todo.txt source into one typed document, apply changes to task structures and serialize the complete document. Patch existing TOML source tables through `toml_edit` rather than through line-oriented string editing.

Pin Rust 1.97.1, cargo-dist 0.32 and cargo-deny through mise. Keep `just` as the documented contributor entrypoint. Nub owns TypeScript, JavaScript and Astro dependency and command paths, Cargo owns Rust dependency and command paths, and npm is used only for the final publication of already verified package archives.

Use Commander for contributor-tool argument schemas, validation and generated help, and Execa for cross-platform subprocess execution. Keep workflow-only `just` recipes private so `just --list` exposes only stable contributor operations.

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

Attach one deterministic ZIP archive per Windows target and one deterministic TAR.XZ archive per macOS and Linux target to the GitHub Release for the sealed version. Name archives from the product version and Rust target triple, and publish one `SHA256SUMS` manifest covering every native archive. Each archive contains the target executable and the public license; runtime assets remain inside the executable.

Use cargo-dist 0.32 for native archives, checksums, and its sh and PowerShell installers. Disable cargo-dist npm publication, Homebrew generation and updater support. During final assembly, `xtask` binds each cargo-dist archive checksum into both generated installers and makes SHA-256 verification mandatory because cargo-dist 0.32 does not enforce it consistently in both scripts. The installers otherwise retain cargo-dist's target selection, download, extraction, PATH and unmanaged-install behavior. Smoke tests install from a local artifact server through both scripts and require a corrupted archive to fail.

Keep target metadata and publication behavior in reviewed repository tools. The release workflow invokes those tools with artifact roots and target identifiers; it does not embed package-integrity algorithms, registry retry logic or per-target binary-path mappings in workflow YAML.

Qualify the native implementation locally through two system boundaries: the current host gate and a disposable Linux container managed by Testcontainers. The Linux image pins Rust 1.97.1 on Alpine 3.23 by tag and digest, copies only the shared Rust test inputs, uses `Cargo.lock`, and executes Clippy, the workspace tests, the black-box CLI contract and a direct binary smoke test. Docker unavailability is a hard failure of the complete local gate. The release matrix continues to invoke the shared target check before assembling artifacts.

## npm distribution

Keep `arcantry` as the unscoped public package. Its existing JavaScript and type exports remain unchanged, while its `bin` entry becomes a narrow JavaScript launcher that resolves and executes an exact-version optional dependency without downloading code at install or execution time.

Publish one scoped package per target:

- `@arcantry/cli-win32-x64`
- `@arcantry/cli-win32-arm64`
- `@arcantry/cli-darwin-x64`
- `@arcantry/cli-darwin-arm64`
- `@arcantry/cli-linux-x64`
- `@arcantry/cli-linux-arm64`

Each platform package declares the matching npm `os` and `cpu` constraints and contains only its executable, manifest, README and license. The Linux packages contain statically linked musl executables and deliberately omit npm's `libc` field so the same package is installable on both glibc and musl systems. The main package pins every platform package to its own exact version through `optionalDependencies`. The launcher maps `process.platform` and `process.arch` to one package, rejects unsupported combinations, and reports how to restore omitted optional dependencies or use the matching GitHub Release. It never performs a network fallback.

Build, allowlist, install and smoke-test all seven npm archives before registry mutation. Execute the same Linux package on Ubuntu glibc and Alpine musl. Exercise the launcher through npm/npx, pnpm, Bun and Nub on representative native runners. Publish or confirm the exact platform package versions before publishing the main `arcantry` package, so the public launcher is never visible without its complete target set. A retry may treat an existing platform version as satisfied only when registry metadata and integrity match the verified archive; no package version may be overwritten. All npm packages continue to use protected trusted publishing after their explicit first-publication bootstrap.

## Release boundary

The GitHub tag, newest release manifest, JavaScript package, six platform packages, native archive versions, embedded product version and release-seal commit must agree before external publication. The release pipeline must execute each release candidate on its declared operating system and architecture rather than treating successful cross-compilation as proof of support.

# Trade-offs

Rust adds a second build toolchain and a six-target release matrix, but removes a runtime requirement from direct CLI use and provides one implementation suitable for all supported platforms.

Separate npm platform packages increase package and publication count. They avoid making every npm user download all six binaries and avoid runtime downloads, so the additional release coordination is accepted.

Embedding assets duplicates some content already present in the JavaScript package and requires a managed local materialization for symbolic links. It keeps direct releases self-contained and preserves the existing link contract without shipping a companion directory.

Keeping the JavaScript library preserves compatibility but temporarily leaves related behavior implemented in two languages. Shared conformance fixtures are the compatibility boundary; removing or replacing the JavaScript API requires a separate change.

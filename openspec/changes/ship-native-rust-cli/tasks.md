# Tasks

- [ ] Establish the Rust 2024 workspace, pinned mise toolchain and `just` entrypoints, then run a compatibility spike for YAML, TOML, npm SemVer, path handling and embedded asset access against the current fixture corpus on every target family.
- [x] Implement the Rust command surface and domain behavior with embedded public assets and safe versioned catalog materialization.
- [x] Build a black-box conformance harness, close all observable gaps and replace the TypeScript CLI entrypoint only after the Rust implementation passes it.
- [ ] Produce, checksum and execute smoke tests for the six native release archives on their declared operating systems and architectures, then generate and test cargo-dist sh and PowerShell installers.
- [ ] Add the six scoped platform packages, the `arcantry` launcher and verified lockstep trusted-publication flow without runtime downloads, including Ubuntu glibc and Alpine musl coverage for each generic Linux package.
- [x] Update installation, CLI and contribution documentation for direct native and npm use while preserving the existing JavaScript API documentation.
- [ ] Run strict OpenSpec validation, Rust tests, TypeScript tests and types, generation checks, package smoke tests, native target smoke tests, the documentation build and `just ci`.
- [x] Review `release.md` against what was actually delivered.

# Tasks

- [x] Establish the Rust 2024 workspace, pinned mise toolchain and `just` entrypoints, then run the compatibility fixtures on the current host and in the disposable Linux system test.
- [x] Implement the Rust command surface and domain behavior with embedded public assets and safe versioned catalog materialization.
- [x] Build a black-box conformance harness, close all observable gaps and replace the TypeScript CLI entrypoint only after the Rust implementation passes it.
- [x] Produce and checksum the six native release archives, retain the shared release target check, and verify archive and installer smoke behavior through the host and disposable Linux system gates.
- [x] Add the six scoped platform packages, the `arcantry` launcher and verified lockstep trusted-publication flow without runtime downloads, including Ubuntu glibc and Alpine musl execution for each generic Linux package.
- [x] Update installation, CLI and contribution documentation for direct native and npm use while preserving the existing JavaScript API documentation.
- [x] Run strict OpenSpec validation, Rust tests, TypeScript tests and types, generation checks, package smoke tests, native conformance, the documentation build, the host gate and the disposable Linux system test.
- [x] Review `release.md` against what was actually delivered.

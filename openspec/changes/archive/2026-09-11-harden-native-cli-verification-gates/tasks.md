# Tasks

- [x] Remove the retired TypeScript-oracle requirement and align the contributor description of `just native-conformance`.
- [x] Version the CLI contract inventory with required behavior dimensions for every leaf command.
- [x] Require independent native evidence for help, invalid input, success, preview, apply and rejection or rollback dimensions.
- [x] Keep the pinned coverage run able to emit line and branch records for every production Rust file.
- [x] Keep per-file floors and zero-evidence reporting diagnostic only; coverage is not a required gate.
- [x] Keep host, Testcontainers Linux and release-target execution as separate reported gates.
- [x] Run `just check-host`, `just native-conformance`, `just linux-system-test`, package smoke and strict OpenSpec validation; keep coverage and release-target rehearsal outside the required gate.
- [x] Review `release.md` against what was actually delivered.

# Why

The native CLI currently passes its host and disposable Linux checks, but the repository can still report complete verification while important behavior is not held by a blocking gate. The canonical CLI specification still requires comparison with the retired TypeScript implementation even though independent Rust expectations replaced that oracle. Rust coverage is diagnostic only, omits at least one production module, and produces no branch records despite the contributor documentation describing line and branch coverage.

# What changes

- Replace the retired implementation comparison with the accepted OpenSpec, documentation and executable contract inventory as the compatibility baseline.
- Make the CLI evidence ledger describe every required behavior dimension, including preview, apply and rejection or rollback for mutating commands.
- Turn Rust coverage into a blocking, per-file non-regression gate with real line and branch evidence.
- Keep the host, disposable Linux and declared release-target checks distinct and required at their appropriate boundaries.
- Align contributor documentation with the verification each command actually performs.

# Out of scope

- Changing public CLI commands or intended behavior.
- Treating cross-compilation as execution evidence for a release target.
- Publishing, tagging or changing the product version.

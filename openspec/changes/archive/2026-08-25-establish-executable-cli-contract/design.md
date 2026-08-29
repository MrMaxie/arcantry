# Approach

Keep a small machine-readable command inventory under `contracts/`. The inventory records the complete leaf-command syntax and links each command to executable evidence. Rust integration tests own independent expected outputs, exit codes and filesystem assertions; they do not invoke the legacy TypeScript CLI.

`just native-conformance` builds the binary and runs the Rust contract suite. The command inventory contains a scenario registry, and the Rust suite dispatches every registered id to executable behavior. Command and trust-claim evidence cannot refer to missing, foreign or differently bound scenarios.

The existing TypeScript scenario-presence test is replaced by a structural check over the inventory. It proves that every declared command has named executable evidence instead of checking for phrases in test source.

Public filesystem mutations use one transaction model: validate every input, stage every replacement beside its destination, commit operations in order, verify the result and restore the exact pre-apply tree when any reversible stage, commit, verify or finalize step fails. Repository adoption converts its managed-file changes into the same serialized project plan used by todo, release and source transitions. Skill links retain their platform-specific junction or symlink operations, but preflight all targets and use reversible backups for both link and unlink flows. Private Git exclusion is committed only after link preparation and participates in rollback.

Applying a serialized plan requires authority derived independently from the current `--cwd` and `--config` resolution. The canonical plan root must equal the current project root. Relative operations remain within that root, while each canonical external operation needs one exact `--allow-outside` path. Canonicalization resolves existing links and the nearest existing ancestor, so permission does not expand to parents, children, siblings or link escapes.

Backup cleanup starts only after all reversible finalization checkpoints pass. A failure before that commit point restores the complete pre-apply tree. Cleanup failure after the verified commit is returned as a warning on stderr rather than a false transaction failure.

Deterministic failpoints are exposed only to Rust tests through an internal filesystem-operation boundary. Property tests generate bounded write/delete plans and failure positions. This keeps production behavior free of test switches while testing the transaction algorithm rather than relying on operating-system-specific permission failures.

# Trade-offs

Expected results must be updated deliberately when the public contract changes. This maintenance cost is accepted because deriving expectations from either implementation would make the test unable to detect shared drift.

The transaction layer remains synchronous because its operations are ordered and rollback-sensitive. Maintained crates provide temporary files, locking, platform links, property generation, shell tokenization, Markdown parsing and container lifecycle management instead of local replacements.

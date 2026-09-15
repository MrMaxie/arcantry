# Tasks

- [x] Restore pull-request and `master` CI with the repository's Rust-owned checks.
- [x] Restore the tag-triggered release workflow with dynamic artifact identity and a protected publication job.
- [x] Resolve the pinned Nub executable before native package validation enters isolated directories.
- [x] Create or refresh the draft GitHub Release with OpenSpec-derived notes before npm publication.
- [x] Accept existing npm packages only when their registry integrity matches the retained archives, including the main package.
- [x] Update contributor guidance, public documentation and workflow contracts for active CI and release automation.
- [x] Limit public installation guidance to npm-compatible launchers and official GitHub Release downloads.
- [x] Correct the retired historical `impact: none` release artifact and restore the only older missing release artifact discovered by the complete archive inventory.
- [x] Finalize the untagged `1.0.0` manifest with every accepted release-bearing change and regenerate the changelog.
- [x] Test retry, integrity mismatch, release-note and workflow safety behavior.
- [x] Run strict OpenSpec validation, `just check-fast`, `just check-host`, `just linux-system-test` and release consistency checks.
- [x] Review the release story, archive the change and verify the final sealed commit before tagging.

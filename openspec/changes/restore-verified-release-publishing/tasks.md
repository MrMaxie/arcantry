# Tasks

- [x] Restore pull-request and `master` CI with the repository's Rust-owned checks.
- [x] Restore the tag-triggered release workflow with dynamic artifact identity and a protected publication job.
- [x] Create or refresh the draft GitHub Release with OpenSpec-derived notes before npm publication.
- [x] Accept existing npm packages only when their registry integrity matches the retained archives, including the main package.
- [x] Update contributor guidance, public documentation and workflow contracts for active CI and release automation.
- [ ] Correct the retired historical `impact: none` release artifact.
- [ ] Finalize the untagged `1.0.0` manifest with every accepted release-bearing change and regenerate the changelog.
- [x] Test retry, integrity mismatch, release-note and workflow safety behavior.
- [ ] Run strict OpenSpec validation, `just check-fast`, `just check-host`, `just linux-system-test` and release consistency checks.
- [ ] Review the release story, archive the change and verify the final sealed commit before tagging.

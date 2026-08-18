# Approach

Pass the pull request head SHA from the CI event to the release check through a narrowly named environment variable. The release entrypoint will consume it only for a GitHub Actions `pull_request` event and forward it as an explicit validation candidate.

Release validation will resolve the candidate to a commit, require it to equal the commit that introduced the newest release manifest and require it to be a direct parent of the checked-out synthetic merge commit. Without that explicit candidate, the existing `HEAD` equality rule remains unchanged. npm publication will continue to call release sealing without an override.

# Trade-offs

The release module gains a small CI-aware input at its tooling boundary, but the core invariant remains host-independent and explicit. Keeping the synthetic merge checkout preserves integration coverage that would be lost by checking out only the pull request branch.

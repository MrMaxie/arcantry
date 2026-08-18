# Why

GitHub checks pull requests from a synthetic merge commit. Strict release validation currently compares that temporary commit with the commit that introduced the newest release manifest, so a correctly sealed pull request fails before it can be merged.

# What changes

- Keep pull request CI on GitHub's synthetic merge commit so integration with the target branch remains covered.
- Let pull request CI identify the exact submitted head commit as the release seal candidate.
- Accept that candidate only when it introduced the newest release manifest and is a direct parent of the checked-out merge commit.
- Keep local validation, branch pushes and npm publication bound to the actual repository `HEAD`.

# Out of scope

- Weakening release sealing outside pull request CI.
- Creating a release tag or publishing an npm package.
- Changing the merge strategy or branch protection requirements.

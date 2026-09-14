# Approach

The existing `1.0.0` manifest is an unsealed internal candidate: no matching Git tag, npm package or GitHub Release exists. Finalize that candidate in place by assigning every accepted release-bearing change and rendering consumer prose from OpenSpec. Do not infer release meaning from the 45 Git commits.

Restore CI for pull requests and `master`. Restore the tag-triggered release workflow as a gated sequence: repository verification, six native target jobs, artifact assembly, installer smoke tests, draft GitHub Release creation, then the protected `npm` environment. The draft uses release notes derived from the managed `1.0.0` changelog section.

The first npm publication uses the exact workflow archives and maintainer 2FA because trusted publishing cannot be configured before a package exists. The protected job waits for maintainer approval while that bootstrap is completed. Its preflight accepts an existing package only when registry integrity matches the retained archive, including the main package, so the same run can safely finalize the GitHub Release. Later versions publish missing packages through OIDC.

Deliver the accumulated local history through one pull request because remote branch protection rejects direct pushes. Use rebase merge to retain coherent commits and linear history.

# Trade-offs

- Do not create a placeholder or prerelease package merely to exercise OIDC. It would invent a release and violate the fixed `1.0.0` identity.
- Do not store an npm token in GitHub. The one-time manual bootstrap is less automated but preserves the accepted credential boundary.
- Do not cut `2.0.0` from the internal development sequence. Finalizing the never-published `1.0.0` candidate keeps the chosen first public identity without rewriting an external release.

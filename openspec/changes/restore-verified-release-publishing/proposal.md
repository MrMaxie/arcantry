# Why

Arcantry has a complete local release toolchain, but its CI and release workflows are disabled while the first public npm packages and GitHub Release do not exist. The unsealed internal `1.0.0` candidate also excludes later accepted changes and cannot pass the current release adapter because one historical artifact still uses the retired `impact: none` value.

# What changes

Restore remote CI and the verified native release workflow, finalize the existing untagged `1.0.0` candidate with every accepted release-bearing change, and support the one-time npm bootstrap without weakening later OIDC trusted publishing. The workflow builds and executes every supported target, retains the exact npm archives, creates a draft GitHub Release before the protected npm job, accepts only byte-identical existing package versions during retry, and publishes the GitHub Release only after the complete npm set is verified.

The release history, workflow contract, package verification and public release notes are coupled because publishing any one of them without the others would expose an incomplete or unverifiable `1.0.0` release.

# Out of scope

- Varlock environment contracts.
- Cloudflare configuration and HSTS.
- Any product or distributable version other than `1.0.0`.
- Long-lived npm publication tokens.

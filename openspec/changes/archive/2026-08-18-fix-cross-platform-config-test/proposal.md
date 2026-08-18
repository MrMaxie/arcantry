# Why

The project configuration contract rejects absolute source paths unless an external configuration explicitly allows them. Its regression coverage used a Windows-only absolute path, so the same behavior was not exercised on Linux and GitHub CI failed after the release was pushed.

# What changes

- Make the absolute-path regression case construct a native absolute path on every supported operating system.
- Prove the default rejection and explicit opt-in behavior with the same platform-neutral fixture.
- Restore a green sealed release before branch protection requires the CI check.

# Out of scope

- Changing the accepted project configuration syntax or path policy.
- Adding new operating-system support.

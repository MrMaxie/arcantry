## ADDED Requirements

### Requirement: Release smoke containers cannot replace verified artifacts

Every container image used after native artifact verification MUST be pinned by digest. The container MUST run without network access or added capabilities, with a read-only root filesystem and verified repository artifacts mounted read-only.

#### Scenario: The Linux executable is checked on musl

- **WHEN** the release workflow runs the verified Linux executable in Alpine
- **THEN** the pinned container can execute the artifact but cannot modify the checkout or contact the network

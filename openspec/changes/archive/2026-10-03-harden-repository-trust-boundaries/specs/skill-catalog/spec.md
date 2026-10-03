## ADDED Requirements

### Requirement: Repository skill targets stay inside the repository

Repository and private skill linking and unlinking MUST canonicalize the nearest existing target ancestor before mutation and MUST reject a target that resolves outside the canonical repository root. Explicit user targets and user-scoped targets retain their separate authority.

#### Scenario: A repository target ancestor is linked outside

- **WHEN** `.agents` or `.claude` resolves through a symlink or junction outside the repository
- **THEN** preflight rejects the operation before creating, replacing or removing any skill link

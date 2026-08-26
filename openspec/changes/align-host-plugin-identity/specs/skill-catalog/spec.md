## ADDED Requirements

### Requirement: Host plugin projections preserve canonical product identity

Every supported host plugin manifest MUST project the same canonical Arcantry name, product version, concise description, author, homepage, repository and license. Each host adapter MUST use an explicit allowlist for host-specific presentation, capability and schema fields and MUST NOT copy unsupported fields from another host merely to make the manifests structurally identical. Repository and package validation MUST inspect every supported host manifest.

#### Scenario: Canonical identity changes

- **WHEN** one canonical product identity field changes through an accepted change
- **THEN** every supported host manifest and packaged projection is updated together
- **AND** each manifest remains valid for its own host contract

#### Scenario: A host exposes additional presentation fields

- **WHEN** one host supports branding or interface metadata that another host does not support
- **THEN** the additional fields remain in that host's projection only
- **AND** shared identity continues to agree across both manifests

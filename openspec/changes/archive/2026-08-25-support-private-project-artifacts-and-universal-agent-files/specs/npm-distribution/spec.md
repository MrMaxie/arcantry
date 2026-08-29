## ADDED Requirements

### Requirement: Package projections expose only current compatibility surfaces

The public package MUST derive its complete-catalog manifests from the canonical skill tree and current unreleased product contract. It MUST include the Codex and Claude plugin manifests, MUST NOT include a Gemini extension manifest, and MUST keep every included manifest aligned with the package identity and version.

#### Scenario: Package contents are prepared

- **WHEN** the release package projection is generated and validated
- **THEN** the package contains the canonical skills plus aligned Codex and Claude plugin manifests
- **AND** no Gemini extension manifest or runtime profile is present

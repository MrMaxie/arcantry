## MODIFIED Requirements

### Requirement: Skills support individual and complete distribution

Users MUST be able to inspect and link one catalog skill through the CLI into a verified Codex, Claude Code, or Gemini CLI user or repository skill directory. The complete public catalog MUST expose aligned Codex plugin, Claude Code plugin, and Gemini CLI extension manifests that use the same canonical `skills/` packages and Arcantry release version. Manual copying, symbolic linking, and independent Agent Skills installers MUST remain supported without becoming runtime dependencies.

#### Scenario: A user chooses one catalog skill

- **WHEN** the user selects a skill from a canonical checkout or installed package
- **THEN** they can link it into the native user or repository scope of a verified host with the Arcantry CLI
- **AND** the installed package remains an independently readable Agent Skills directory

#### Scenario: A user chooses the full collection

- **WHEN** the user loads the repository as a Codex plugin, Claude Code plugin, or Gemini CLI extension
- **THEN** every validated public skill in the current catalog is discovered from the same canonical `skills/` tree
- **AND** every host manifest reports the current Arcantry release version

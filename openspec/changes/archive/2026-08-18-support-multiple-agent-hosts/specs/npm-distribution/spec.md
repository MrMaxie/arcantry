## MODIFIED Requirements

### Requirement: Publication uses the verified package archive

The repository MUST build one package archive for a release candidate, validate its file allowlist, install it in an isolated smoke environment and publish that same archive. The archive MUST include the generated Codex plugin, Claude Code plugin, and Gemini CLI extension manifests aligned with the package identity and version. It MUST exclude source-only, private, local and repository-management state.

#### Scenario: Package verification succeeds

- **WHEN** the release job prepares an npm archive
- **THEN** CLI version, commands, declared exports, and supported agent manifests are exercised from an isolated installation of that archive
- **AND** the verified archive is retained as the only publication input

#### Scenario: A host manifest drifts

- **WHEN** a supported agent manifest differs from the package name or version
- **THEN** package verification fails before npm authentication or registry mutation

#### Scenario: Package contents drift

- **WHEN** the archive contains an unexpected or forbidden path
- **THEN** publication fails before npm authentication or registry mutation

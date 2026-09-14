# practical-workflow Specification

## Purpose
TBD - created by archiving change deliver-practical-mvp. Update Purpose after archive.

## Requirements

### Requirement: Project answers require at most three commands

Arcantry MUST expose context, next and explain with human and JSON output backed by the same discovered project sources. It MUST distinguish observed facts, recommendations and unknown authority. It MUST NOT infer conversational approval. Missing optional tools MUST NOT block file-based work. MCP MUST expose read-only answers and plans through the same core functions.

#### Scenario: An agent resumes work without conversation history

- **WHEN** the agent requests context, next and a relevant explanation
- **THEN** it receives the project sources, selected work, applicable format, blockers and actionable next step with source references

### Requirement: Project conventions control releases

Arcantry MUST support SemVer, monotonic integer and calendar versions selected per project or unit, retain package format constraints and support preset or project-template changelogs. Existing configurations MUST preserve SemVer behavior. Existing unmanaged history MUST remain intact. Ordinary work MUST NOT require a release.

#### Scenario: A project uses numbered releases

- **WHEN** it selects the integer strategy and plans a release
- **THEN** the next identifier increments numerically and its preview updates only compatible version sources and managed changelog content

### Requirement: Local verification replaces mandatory CI

Implementation MUST use Windows host checks and existing Linux Testcontainers. Only Pages documentation generation, build and deploy MUST remain enabled remotely. Coverage MUST be diagnostic, not a completion gate. Changes MUST remain at version 1.0.0 until separately authorized.

#### Scenario: Documentation changes reach master

- **WHEN** the Pages workflow runs
- **THEN** it builds and deploys documentation without running Rust, package or release gates

### Requirement: Safe operations remain previewable and recoverable

File mutations MUST retain drift checks, preserve unrelated content and refuse ambiguous recovery. Plans MAY be saved to an explicit file with private-content disclosure. Context profiles and Varlock observation MUST NOT expose environment values or imply authorization. Detachment MUST identify project ownership and preserve licensing.

#### Scenario: A preview becomes stale

- **WHEN** an input changes before apply
- **THEN** apply refuses to overwrite it and leaves existing content intact

## ADDED Requirements

### Requirement: Release commands expose local release management

The `release` group MUST expose `baseline`, `plan`, `cut`, `render` and `check`. `plan` and `check` MUST be read-only. `baseline`, `cut` and `render` MUST preview a serializable drift-checked plan by default and MUST NOT write unless the caller supplies `--apply`.

#### Scenario: A release mutation is previewed

- **WHEN** a user runs `release baseline`, `release cut` or `release render` without `--apply`
- **THEN** Arcantry reports the planned operations and leaves project files unchanged

#### Scenario: A release mutation is applied

- **WHEN** a user supplies `--apply` for a conflict-free release plan
- **THEN** Arcantry applies all planned writes atomically using recorded input preconditions

#### Scenario: A project release is inspected

- **WHEN** a user runs `release plan` or `release check`
- **THEN** Arcantry reports release state without committing, tagging, pushing or publishing

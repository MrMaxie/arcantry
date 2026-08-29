## ADDED Requirements

### Requirement: Documented syntax matches the public command inventory

Every public CLI command row and syntax block MUST match the tracked executable contract inventory. Every fenced command and table cell beginning with `arcantry` in any Markdown or MDX document MUST parse through the same Clap model as the binary. Documentation verification MUST fail for a missing, additional, differently named or unparsable command or option.

#### Scenario: Documentation and the binary disagree

- **WHEN** authored CLI syntax differs from the command inventory used by native verification
- **THEN** repository checks fail before the documentation can be published

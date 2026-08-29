## MODIFIED Requirements

### Requirement: Every skill is a self-contained canonical package

Each public skill MUST live directly under `skills/<name>`, and each private repository skill MUST live directly under `.local/skills/<name>`. A package MUST contain the instructions and colocated resources required to use it and MUST NOT import runtime behavior from sibling skill packages. Multiple installation aliases resolving to the same package are one skill. Different canonical packages claiming the same skill name MUST be rejected as an identity conflict.

#### Scenario: A skill is inspected independently

- **WHEN** the user inspects one public or private canonical skill
- **THEN** its package contains or declares every required instruction, asset, reference, script, and tool dependency

#### Scenario: Universal and Claude aliases coexist

- **WHEN** `.agents/skills/<name>` and `.claude/skills/<name>` resolve to the same canonical directory
- **THEN** inventory and diagnostics report one skill with two destinations
- **AND** do not report a duplicate-name conflict

#### Scenario: Two canonical sources reuse a name

- **WHEN** different real directories expose the same frontmatter skill name
- **THEN** inventory and linking report a conflict before changing either installation

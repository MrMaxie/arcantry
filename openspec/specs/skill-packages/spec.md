# skill-packages Specification

## Purpose
Define the structure, focus and authorization boundaries of published skill packages.

## Requirements

### Requirement: Every skill is a self-contained canonical package

Each skill MUST live directly under `skills/<name>` and MUST contain the instructions and colocated resources required to use it. Skills MUST NOT import runtime behavior from sibling skill packages.

#### Scenario: A skill is inspected independently

- **WHEN** the user inspects one catalog skill
- **THEN** its package contains or declares every required instruction, asset, reference, script, and tool dependency

### Requirement: Skills represent focused capabilities

A published skill MUST deliver a task-focused capability. A package that only renames or forwards to other skills MUST NOT be published as a skill.

#### Scenario: Cross-skill routing is needed

- **WHEN** Arcantry recommends a set of skills for a task
- **THEN** catalog metadata or selection logic performs the routing
- **AND** no router-only skill is required

### Requirement: External writes require explicit authority

Skills that depend on connectors or remote systems MUST declare those dependencies. Reading an external source MUST NOT imply permission to create, update, publish, or reply in that system.

#### Scenario: A skill uses a remote task source

- **WHEN** the task requires an external write
- **THEN** the skill checks for explicit user authorization for the target and action before writing

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

### Requirement: Complete review claims require explicit coverage

A review skill that inspects an entire repository or another complete surface MUST derive a coverage list from the available tree, manifests and source-of-truth documentation. It MUST mark every material component and contract as reviewed or explicitly unreviewed before claiming complete coverage.

#### Scenario: A complete repository review is requested

- **WHEN** the staged code review skill is asked to review the entire repository
- **THEN** it accounts for each material component and contract before reporting the review as complete
- **AND** any unreviewed area remains explicit in the result

### Requirement: Audience guidance preserves derived artifact contracts

The audience and scope discipline skill MUST treat an existing artifact's established structure, labels, order, formulas, styles and level of detail as contractual when editing it or creating a derived copy, unless the requester explicitly authorizes a redesign. For comparison and audit artifacts, excluded entries MUST remain type-valid, visibly neutralized and excluded from calculations unless explicit instructions or an established convention require deletion or reclassification.

#### Scenario: A comparison estimate reduces scope

- **WHEN** a user requests a reduced-scope estimate derived from an existing estimate
- **THEN** the derived estimate preserves the source rows, labels, order, formulas and presentation conventions
- **AND** excluded numeric estimates use the established neutral value, or `0` when no convention exists
- **AND** neutralized values do not contribute to totals

#### Scenario: The requester authorizes a new representation

- **WHEN** a requester explicitly asks to redesign, delete or reclassify source content
- **THEN** the skill follows that representation while preserving unaffected contractual values and terminology

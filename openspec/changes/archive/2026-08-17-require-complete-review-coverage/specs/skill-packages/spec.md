## ADDED Requirements

### Requirement: Complete review claims require explicit coverage

A review skill that inspects an entire repository or another complete surface MUST derive a coverage list from the available tree, manifests and source-of-truth documentation. It MUST mark every material component and contract as reviewed or explicitly unreviewed before claiming complete coverage.

#### Scenario: A complete repository review is requested

- **WHEN** the staged code review skill is asked to review the entire repository
- **THEN** it accounts for each material component and contract before reporting the review as complete
- **AND** any unreviewed area remains explicit in the result

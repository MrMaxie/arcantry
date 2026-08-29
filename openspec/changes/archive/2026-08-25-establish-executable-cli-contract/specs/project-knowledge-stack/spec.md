## ADDED Requirements

### Requirement: Every source transition has executable native evidence

The native contract suite MUST exercise `preserve`, `adopt`, `rebind`, `cutover`, `migrate` and `relocate` through serialized repository plans. It MUST verify preview behavior, unchanged-input application and rejection after relevant input drift.

#### Scenario: A transition strategy regresses

- **WHEN** any supported transition cannot produce or safely apply its documented plan
- **THEN** native contract verification fails before the change can qualify as complete

### Requirement: Project plans preserve the pre-apply tree on failure

Project-plan execution MUST support write, delete and delete-tree operations as one reversible transaction. It MUST reject root mismatch, unauthorized paths and input drift before commit, verify each committed result and remove transaction-created parent directories or artifacts during rollback.

#### Scenario: A generated plan fails at any bounded operation position

- **WHEN** a valid plan contains a generated sequence of writes and deletions and execution fails at a staged, committed, verification or reversible finalization step
- **THEN** the complete affected tree is byte-for-byte and structure-for-structure equal to its state before apply

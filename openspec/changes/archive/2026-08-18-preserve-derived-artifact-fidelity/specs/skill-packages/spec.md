## ADDED Requirements

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

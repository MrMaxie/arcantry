## ADDED Requirements

### Requirement: Recommended overview setup retains user-scoped skills

The recommended shared and private overview preset MUST keep reusable skills active in the Computer scope with user mode while also showing the established repository-scoped skills and project knowledge. This preset MUST use the existing source row and active-state treatment rather than duplicating the skill entry or adding explanatory copy.

#### Scenario: A reader inspects the recommended setup

- **WHEN** the shared and private preset is selected
- **THEN** Computer skills are active with mode `User`
- **AND** Repository skills remain active with mode `Project`
- **AND** the scope view does not add another skill row or description

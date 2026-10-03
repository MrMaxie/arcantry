## MODIFIED Requirements

### Requirement: Content safety skills protect audience and substance

The content-safely family MUST support audience and scope discipline, graphical interface composition, terminal experience design, and product content writing. Content workflows MUST identify the primary audience, their current task, existing knowledge and established terminology before turning source material into reader-facing content. Requirements and available implementation data MUST remain implementation inputs unless the request, audience contract or current task admits them into the artifact. Content workflows MUST prevent private context leakage, preserve established artifact contracts, remove vague, repetitive, process-centered or unsupported writing that does not help the intended reader, and preserve exact source-backed values when an established reader task requires them.

#### Scenario: Requirements enumerate implementation data

- **WHEN** a request or source enumerates fields without asking to display them
- **THEN** the workflow treats those fields as implementation inputs
- **AND** exposes only details admitted by the primary audience's current task or established artifact contract

#### Scenario: A reader task requires exact evidence

- **WHEN** an established verification, reconciliation or support task requires an exact source-backed value
- **THEN** the workflow preserves that value in the audience's usable format
- **AND** does not replace it with a generic assurance or invent additional evidence

#### Scenario: Product-facing content is prepared

- **WHEN** an agent writes or revises content for a defined audience
- **THEN** the result leads with the reader's outcome, uses evidence-backed concrete language, excludes internal process commentary, and preserves approved presentation
- **AND** conditional examples transfer the audience decision rather than their subject, wording, controls or fields

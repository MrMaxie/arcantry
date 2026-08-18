## ADDED Requirements

### Requirement: Self-improvement skills maintain reusable agent capabilities

The self-improvement family MUST support capturing repeated work, evaluating and creating focused skills, maintaining scoped agent guidance, selecting relevant skills, and staging review findings behind explicit approval. Guidance changes MUST distinguish user, repository, and private repository scope and MUST remain approval-gated.

#### Scenario: A repeated workflow becomes reusable

- **WHEN** an agent identifies repeated steps with a stable trigger and outcome
- **THEN** the family provides a path to capture evidence, choose the correct scope, create or refine the capability, and evaluate the result

### Requirement: Repository safety skills preserve project knowledge roles

The repo-safely family MUST support minimal adoption, hot-thought capture, release-story maintenance, explicit source reconciliation, and risk-proportionate verification. These skills MUST preserve the distinct roles of todo, OpenSpec, changelog, and private local state and MUST NOT infer authorization for destructive operations, publication, or automatic synchronization.

#### Scenario: A project insight arrives during implementation

- **WHEN** the insight is not yet accepted product intent
- **THEN** the workflow records it in the appropriate todo source without mutating specifications or release history

#### Scenario: Work needs stronger verification

- **WHEN** impact or uncertainty crosses a material threshold
- **THEN** the verification workflow adds an appropriate independent, live, fresh-context, subagent, or user approval layer
- **AND** distinguishes verified behavior from assumptions and planned behavior

### Requirement: Content safety skills protect audience and substance

The content-safely family MUST support audience and scope discipline, terminal experience design, and product content writing. Content workflows MUST prevent private context leakage, preserve established artifact contracts, and remove vague, repetitive, process-centered, or unsupported writing that does not help the intended reader.

#### Scenario: Product-facing content is prepared

- **WHEN** an agent writes or revises content for a defined audience
- **THEN** the result leads with the reader's outcome, uses evidence-backed concrete language, excludes internal process commentary, and preserves approved presentation

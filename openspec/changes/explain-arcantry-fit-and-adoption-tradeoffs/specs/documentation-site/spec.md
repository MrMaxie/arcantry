## ADDED Requirements

### Requirement: Adoption guidance supports an informed pilot decision

Adoption guidance MUST identify the work required to evaluate and adopt the selected scope, the ongoing ownership retained by the project, observable pilot success criteria and explicit stopping or removal conditions. Claims about effort or value MUST be grounded in current product behavior and representative project workflows and MUST NOT present unsupported return-on-investment estimates.

#### Scenario: A team evaluates a pilot

- **WHEN** a product or engineering leader compares Arcantry's expected value with organizational overhead
- **THEN** the guidance identifies the smallest representative pilot, responsibilities, success signals and stopping conditions
- **AND** distinguishes verified product behavior from assumptions the team must validate in its own repository

### Requirement: Adjacent engineering practices retain their authority

Documentation MUST explain that ADRs and RFCs own decision rationale, issue trackers own delivery coordination, project documentation owns durable usage knowledge and release practices own delivered history. Arcantry MUST describe supported connections and boundaries without turning those systems into Arcantry-owned sources or duplicating their content by default.

#### Scenario: A mature repository compares integration

- **WHEN** an evaluator already has credible decision, planning, documentation and release practices
- **THEN** the guidance shows what remains authoritative, what Arcantry may connect and what it does not manage
- **AND** identifies when preserving the existing system without Arcantry is the lower-cost choice

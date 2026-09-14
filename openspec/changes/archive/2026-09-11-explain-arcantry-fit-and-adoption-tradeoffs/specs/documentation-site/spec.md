## ADDED Requirements

### Requirement: Adoption guidance supports an informed ownership decision

Adoption guidance MUST identify the work required to evaluate and adopt the selected scope, the ongoing ownership retained by the project, and the observable behavior available for the project's own evaluation. The project MUST retain authority over its evaluation period, success signals and stopping or removal conditions. Claims about effort or value MUST be grounded in current product behavior and representative project workflows and MUST NOT present unsupported return-on-investment estimates.

#### Scenario: A team evaluates a pilot

- **WHEN** a product or engineering leader compares Arcantry's expected value with organizational overhead
- **THEN** the guidance identifies available scopes, responsibilities and observable product behavior
- **AND** leaves evaluation criteria and the stopping decision to that team

### Requirement: Adjacent engineering practices retain their authority

Documentation MUST explain that ADRs and RFCs own decision rationale, issue trackers own delivery coordination, project documentation owns durable usage knowledge and release practices own delivered history. Arcantry MUST describe supported connections and boundaries without turning those systems into Arcantry-owned sources or duplicating their content by default.

#### Scenario: A mature repository compares integration

- **WHEN** an evaluator already has credible decision, planning, documentation and release practices
- **THEN** the guidance shows what remains authoritative, what Arcantry may connect and what it does not manage
- **AND** identifies when preserving the existing system without Arcantry is the lower-cost choice

### Requirement: Project-work adoption does not imply delivery integration

Documentation MUST distinguish shared project-work configuration from integrating Arcantry into product runtime, build, CI or publication workflows. Guidance and generated adoption requests MUST NOT infer those delivery-toolchain changes from shared scope and MUST require an explicit user selection or request before including them.

#### Scenario: A project adopts shared configuration only

- **WHEN** a user selects shared project-work configuration without selecting a delivery-toolchain integration
- **THEN** adoption guidance limits the requested changes to the selected project-work scope
- **AND** does not request changes to product runtime, build, CI or publication workflows

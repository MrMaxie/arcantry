## ADDED Requirements

### Requirement: Public trust claims have audience-visible evidence

Every material public claim about data flow, network activity, permissions, repository writes, rollback, private state, skill trust, package integrity or vulnerability handling MUST identify an accepted requirement, evidence at least as broad as the claim and the boundary Arcantry actually controls. Evidence required to evaluate a public claim MUST be available to that audience. When complete public evidence is unavailable, documentation MUST narrow the claim and state the remaining responsibility instead of exposing private diagnostics or presenting an aspiration as a guarantee.

#### Scenario: A public guarantee exceeds its evidence

- **WHEN** the claim covers behavior, platforms or actors that its linked evidence does not cover
- **THEN** documentation validation fails until the evidence expands or the wording narrows
- **AND** a responsibility boundary is not presented as verified Arcantry behavior

#### Scenario: Evidence contains private diagnostic data

- **WHEN** supporting evidence cannot be published safely to the claim's audience
- **THEN** the public claim is limited to evidence that audience can inspect
- **AND** private diagnostics are not copied into the public trust surface

### Requirement: Trust evidence has one non-duplicative owner

The public trust inventory MUST reference the CLI provenance ledger for CLI-specific claims and MUST NOT restate or independently rebind its executable evidence. Non-CLI trust claims MUST have their own stable owners and evidence references.

#### Scenario: A CLI trust claim appears in the public inventory

- **WHEN** the inventory includes a claim already owned by the CLI provenance contract
- **THEN** it references that existing claim and evidence identity
- **AND** validation rejects a competing duplicate mapping

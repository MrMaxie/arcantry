## ADDED Requirements

### Requirement: Managed removal and permanent detachment are distinct

Removal MUST delete only verified Arcantry-managed artifacts or sections and MUST NOT imply that retained project content has become an independent Arcantry replacement. Permanent detachment MUST be a separate approval-gated operation that transfers only selected capabilities into project-owned outputs and explicitly gives up Arcantry update, compatibility, branding and support promises for those outputs.

#### Scenario: A user removes managed repository state

- **WHEN** the user requests removal without an accepted detachment plan
- **THEN** Arcantry removes only verified managed artifacts for the selected scope
- **AND** does not create, rename or represent retained content as a detached implementation

#### Scenario: A user selects permanent detachment

- **WHEN** a reviewed detachment plan is accepted and applied
- **THEN** only its selected project-owned capabilities are written and verified
- **AND** the result identifies the project as its ongoing maintenance and security owner

### Requirement: Detachment requires a reviewed ownership-transfer plan

A detachment plan MUST record the selected capabilities, transformations, omissions, project-owned identities, copied and reimplemented material, applicable licenses and attribution, final ownership, capability budget and forbidden dependencies before writing. Plan and apply MUST use drift-safe input hashes and MUST keep source removal separately authorized and ordered after target verification.

#### Scenario: Export scope is incomplete

- **WHEN** a required file, dependency, license obligation or ownership decision is absent from the plan
- **THEN** validation rejects the plan before any output is written

#### Scenario: Source state changes after review

- **WHEN** an accepted plan no longer matches its source inputs
- **THEN** apply refuses every planned write and deletion
- **AND** requires a new review of the changed export

### Requirement: Detached capabilities have an enforceable negative contract

Detached outputs MUST NOT depend on the Arcantry CLI, configuration, packages, runtime, build, CI, network services, user-scoped assets, private planning paths, product branding or update endpoints. Each retained capability MUST solve one verified project problem. Portability frameworks, adapters, installers, catalogs, multi-project behavior and other reusable-product capabilities MUST remain excluded unless independently justified and accepted for that project.

#### Scenario: A detached workflow runs from a fresh checkout

- **WHEN** independence verification runs without Arcantry tooling, user assets, private planning state or network access
- **THEN** every retained script, skill, documentation path and validation command works from project-owned inputs
- **AND** scanning finds no forbidden dependency or Arcantry brand identity

#### Scenario: A generic capability enters the export

- **WHEN** an output adds portability or reusable-product behavior not required by one selected project problem
- **THEN** the capability budget fails until the item is removed or separately accepted with evidence

### Requirement: Detached comparison never becomes silent synchronization

A later comparison with Arcantry MUST be a new explicit read-only transition that reports differences, ownership conflicts, licensing consequences and compatibility choices. It MUST NOT update detached outputs, restore branding or change managed status without a separately accepted apply operation.

#### Scenario: A detached project compares a later Arcantry version

- **WHEN** the project requests a comparison
- **THEN** the result identifies candidate changes and conflicts without writing project state
- **AND** selective re-adoption requires a new reviewed transition

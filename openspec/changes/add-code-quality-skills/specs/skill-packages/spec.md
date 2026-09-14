## ADDED Requirements

### Requirement: Code-quality work preserves phase and evidence boundaries

The public catalog MUST provide distinct skills for assessing code quality, designing maintainable code structure and performing an authorized refactor. Assessment MUST remain read-only and report each material finding with concrete evidence, its maintenance or correctness consequence and a proportionate response. Design MUST ground module, interface, abstraction and dependency decisions in the current repository and applicable official ecosystem guidance. Refactoring MUST preserve observable behavior, exclude feature work and proceed through independently verified increments. None of the skills MUST require object-oriented patterns or treat a named smell as sufficient evidence of a defect.

#### Scenario: Existing code is reviewed without change authority

- **WHEN** a user asks for a code-quality assessment without asking for implementation
- **THEN** the assessment skill reports prioritized evidence-backed findings without editing files
- **AND** it does not present stylistic preference or a pattern name as proof of a defect

#### Scenario: A structural approach is needed before implementation

- **WHEN** a user asks how to organize a non-trivial change
- **THEN** the design skill inventories current conventions and affected boundaries
- **AND** it returns implementation-ready decisions while leaving code unchanged

#### Scenario: A refactor is authorized

- **WHEN** the user authorizes a defined structural refactor
- **THEN** the refactoring skill protects observable behavior with an explicit baseline
- **AND** it implements and verifies small coherent increments without adding product behavior

## MODIFIED Requirements

### Requirement: Every public skill is a complete canonical package

Every public skill MUST be represented by exactly one canonical package with a valid `SKILL.md`, `arcantry.json` and `agents/openai.yaml`. Package metadata and generated projections MUST preserve discriminating triggers, phase boundaries and the exact resources required by the workflow.

#### Scenario: A code-quality package enters the catalog

- **WHEN** a code-quality candidate is admitted
- **THEN** its package validates and its routing cases distinguish it from adjacent review, verification and implementation workflows
- **AND** generated catalog pages include it exactly once

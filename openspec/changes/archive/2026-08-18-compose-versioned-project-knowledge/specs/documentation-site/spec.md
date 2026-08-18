## MODIFIED Requirements

### Requirement: Documentation explains the unified adoption journey

The authored documentation MUST lead an evaluator or adopter from product value through first inspection, adoption choices, task guides, concepts, and reference material. Guidance for contributing to Arcantry itself MUST be grouped separately from guidance for projects that use Arcantry. Existing public routes for adoption, repository workflow, repository contract, CLI, and lifecycle pages MUST remain available.

#### Scenario: A new reader evaluates Arcantry

- **WHEN** they open the overview and follow the primary path
- **THEN** they can inspect a project before choosing any repository footprint
- **AND** they can distinguish adopter guidance from Arcantry contributor rules

## ADDED Requirements

### Requirement: Documentation explains the composable project knowledge stack

The authored documentation MUST explain OpenSpec authority, changelog projection, todo.txt queues, skill capabilities, the `.local/` privacy boundary, and optional integrity evidence as independent roles. It MUST distinguish discovery, repository footprint, and management responsibility as separate adoption axes. It MUST NOT claim that Git, OpenSpec, configuration, a Node runtime inside the target project, `just`, `mise`, or any Arcantry-managed repository footprint is universally required.

#### Scenario: A project adopts only one capability

- **WHEN** a reader chooses queues or skills without configuring other source kinds
- **THEN** the documentation presents that selection as supported
- **AND** does not instruct them to initialize unrelated artifacts

### Requirement: Adoption documentation matches implemented inspection and transitions

The getting-started path MUST begin with `repo inspect` and offer equivalent npm, pnpm, and Nub package-runner commands. It MUST state the CLI runtime requirement and that package runners do not add Node tooling to the target project. Adoption and workflow guidance MUST cover configuration-free observation, external and tracked configuration, monorepos, explicit plan and apply, input hash checks, plan privacy, atomic application, and replanning after drift.

#### Scenario: A mature project is inspected before adoption

- **WHEN** the reader runs the documented package-runner command
- **THEN** Arcantry describes the discovered source stack without writing project state
- **AND** later mutations require an explicit serialized plan or `--apply`

### Requirement: Reference documentation matches public implementation contracts

The CLI reference MUST reflect the options exposed by `program.ts` and MUST identify `repo init`, `repo update`, and `repo remove` as legacy compatibility commands. The configuration reference MUST document precedence, every accepted field, tool and adapter version boundaries, `from` relationships, path constraints, monorepo resolution, and the published TOML Schema. Managed changelog guidance MUST always derive release meaning from OpenSpec while permitting observed changelogs without OpenSpec.

#### Scenario: A reader checks a command or configuration field

- **WHEN** they use the CLI or configuration reference
- **THEN** the documented name, option, default, and constraint match the current parser and command surface

## MODIFIED Requirements

### Requirement: Cross-cutting guidance is authored and skill detail is generated

Overview, getting started, adoption, workflow, todo.txt, project knowledge, CLI, configuration, lifecycle, contributor commands, and skills overview pages MUST be authored for their reader journeys. Per-skill detail pages MUST remain generated from canonical skill packages by `tooling/generate.ts`.

#### Scenario: Skill metadata is updated

- **WHEN** documentation generation runs
- **THEN** the skill detail changes without requiring a second authored edit

## ADDED Requirements

### Requirement: Documentation links and presentation remain stable

The sidebar MUST use grouped Start, Guides, Concepts, Reference, and Contributing to Arcantry sections. Internal links and fragments from the overview and sidebar MUST resolve in the built site. This content change MUST preserve the existing Starlight shell, typography, animation, and visual composition except for copy, information order, commands, and links required by the new documentation architecture.

#### Scenario: A reader navigates the rebuilt documentation

- **WHEN** they follow an overview action or sidebar item
- **THEN** the intended page or section exists
- **AND** the established documentation shell remains visually unchanged

## MODIFIED Requirements

### Requirement: Arcantry documents its own conformance

The contributor documentation MUST state that Arcantry validates its own repository through documented project commands and contracts. It MUST NOT present Arcantry's mandatory OpenSpec change and release lifecycle as a universal requirement for projects that only use Arcantry.

#### Scenario: A contributor checks Arcantry itself

- **WHEN** the contributor follows the Contributing to Arcantry section
- **THEN** they can identify the repository verification surface and Arcantry-specific lifecycle rules

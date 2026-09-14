# skill-packages Specification

## Purpose
Define the structure, focus and authorization boundaries of published skill packages.
## Requirements
### Requirement: Every skill is a self-contained canonical package

Each public skill MUST live directly under `skills/<name>`, and each private repository skill MUST live directly under `.local/skills/<name>`. A package MUST contain the instructions and colocated resources required to use it and MUST NOT import runtime behavior from sibling skill packages. Every public package MUST have a valid `SKILL.md`, `arcantry.json` and `agents/openai.yaml`; its metadata and generated projections MUST preserve discriminating triggers, phase boundaries and the exact resources required by the workflow. Multiple installation aliases resolving to the same package are one skill. Different canonical packages claiming the same skill name MUST be rejected as an identity conflict.

#### Scenario: A skill is inspected independently

- **WHEN** the user inspects one public or private canonical skill
- **THEN** its package contains or declares every required instruction, asset, reference, script, and tool dependency

#### Scenario: Universal and Claude aliases coexist

- **WHEN** `.agents/skills/<name>` and `.claude/skills/<name>` resolve to the same canonical directory
- **THEN** inventory and diagnostics report one skill with two destinations
- **AND** do not report a duplicate-name conflict

#### Scenario: Two canonical sources reuse a name

- **WHEN** different real directories expose the same frontmatter skill name
- **THEN** inventory and linking report a conflict before changing either installation

#### Scenario: A code-quality package enters the catalog

- **WHEN** a code-quality candidate is admitted
- **THEN** its package validates and its routing cases distinguish it from adjacent review, verification and implementation workflows
- **AND** generated catalog pages include it exactly once

### Requirement: Skills represent focused capabilities

A published skill MUST deliver a task-focused capability. A package that only renames or forwards to other skills MUST NOT be published as a skill.

#### Scenario: Cross-skill routing is needed

- **WHEN** Arcantry recommends a set of skills for a task
- **THEN** catalog metadata or selection logic performs the routing
- **AND** no router-only skill is required

### Requirement: External writes require explicit authority

Skills that depend on connectors or remote systems MUST declare those dependencies. Reading an external source MUST NOT imply permission to create, update, publish, or reply in that system.

#### Scenario: A skill uses a remote task source

- **WHEN** the task requires an external write
- **THEN** the skill checks for explicit user authorization for the target and action before writing

### Requirement: Complete review claims require explicit coverage

A review skill that inspects an entire repository or another complete surface MUST derive a coverage list from the available tree, manifests and source-of-truth documentation. It MUST mark every material component and contract as reviewed or explicitly unreviewed before claiming complete coverage.

#### Scenario: A complete repository review is requested

- **WHEN** the staged code review skill is asked to review the entire repository
- **THEN** it accounts for each material component and contract before reporting the review as complete
- **AND** any unreviewed area remains explicit in the result

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

### Requirement: Self-improvement skills maintain reusable agent capabilities

The self-improvement family MUST support capturing repeated work, evaluating and creating focused skills, maintaining scoped agent guidance, selecting relevant skills, and staging review findings behind explicit approval. Guidance changes MUST distinguish user, repository, and private repository scope and honor existing user authorization. Missing authorization requires review of the exact proposed change; an already approved action MUST NOT require repeated approval.

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

The content-safely family MUST support audience and scope discipline, graphical interface composition, terminal experience design, and product content writing. Content workflows MUST prevent private context leakage, preserve established artifact contracts, and remove vague, repetitive, process-centered, or unsupported writing that does not help the intended reader.

#### Scenario: Product-facing content is prepared

- **WHEN** an agent writes or revises content for a defined audience
- **THEN** the result leads with the reader's outcome, uses evidence-backed concrete language, excludes internal process commentary, and preserves approved presentation

### Requirement: Todo-writing skills follow the selected source contract

A canonical skill that creates, retains or directly rewrites todo.txt content MUST inspect the selected source before writing. It MUST follow explicit user instruction and compatible configured or repository guidance, MUST preserve established queue vocabulary, and MUST use only project, context and metadata tokens whose meaning matches the task. It MUST preview the exact resulting physical line and identify the source of every optional field before requesting apply authority. When required local metadata or taxonomy is ambiguous, it MUST ask for one bounded decision instead of inventing or omitting the choice. When no compatible convention exists, it MUST use the official todo.txt baseline without optional metadata.

#### Scenario: Neighboring tasks use a redundant project token

- **WHEN** the selected queue already represents project ownership through its source or a different established vocabulary
- **THEN** the skill does not add a redundant project tag merely because a generic example uses one
- **AND** the preview preserves the queue owner's terminology

#### Scenario: A queue consistently relies on additional context

- **WHEN** applicable guidance or an unambiguous convention requires context that the request did not provide
- **THEN** the skill asks the user to choose from the compatible established values
- **AND** leaves the source unchanged until the exact line is approved

#### Scenario: A source has no explicit format convention

- **WHEN** no compatible user, configured, repository or unambiguous source convention applies
- **THEN** the skill writes one non-empty task line using the official todo.txt baseline
- **AND** does not add optional priority, date, project, context or metadata

#### Scenario: A skill writes to an existing project queue

- **WHEN** a canonical skill is authorized to add or retain work in an existing todo source
- **THEN** it preserves unrelated lines and compatible source conventions
- **AND** writes the affected task in a format accepted by that source

#### Scenario: A skill partially promotes a todo entry

- **WHEN** an accepted transformation promotes only the durable portion of one todo entry
- **THEN** the retained portion remains a valid task in the source's governing format
- **AND** unrelated todo entries remain unchanged

### Requirement: Graphical interface composition balances task, density and component contracts

The content-safely family MUST provide a focused skill for designing, implementing, and auditing graphical interfaces for the intended user's task. The skill MUST inspect the existing rendered surface and applicable component contracts before proposing or making changes. It MUST justify persistent text, controls, grouping, and space by the understanding, decision, or action they support; it MUST NOT treat either minimalism or maximum information density as a universal target. It MUST reuse an established component or variant when the same visual or behavioral contract applies, while permitting a unique layout to remain local when no reusable contract exists. Before claiming a coherent result, it MUST compare every affected candidate and verify the rendered interface at the relevant viewport, interaction, state, and accessibility boundaries.

#### Scenario: A settings surface is overloaded or underexplained

- **WHEN** related controls are separated by redundant headings, descriptions, frames, undersized fields, or accidental gaps, or when aggressive reduction leaves an ambiguous title or control
- **THEN** the skill identifies the audience task, consolidates controls that belong together, removes text that changes no decision, and preserves concise explanation where meaning or consequence is not evident
- **AND** control sizing, grouping, and available space support the task rather than a fixed density preference

#### Scenario: Repeated graphical controls drift

- **WHEN** equivalent buttons, icons, effects, cards, tags, or control groups appear across the affected interface
- **THEN** the skill inventories every applicable candidate and reuses the established component contract or adds one justified shared variant
- **AND** unique elements remain local only when they do not represent the same recurring visual or behavioral contract
- **AND** the rendered candidates are compared before completion

#### Scenario: The request concerns a terminal interface

- **WHEN** the requested surface is a CLI, TUI, terminal prompt, or terminal output
- **THEN** the graphical interface composition skill does not claim that work
- **AND** the existing terminal experience capability remains the focused catalog route

### Requirement: Todo promotion skills separate semantic and mechanical authority

The canonical promotion skill MUST classify bounded todo entries, assign stable source and transformation ids, support zero-to-many and many-to-many mappings, present a complete coverage ledger and obtain item-level decisions. It MUST use deterministic CLI snapshots and atomic project plans when available, but MUST NOT delegate semantic classification or approval to the CLI. Full source removal MUST follow target validation; partial promotion MUST preserve the remainder.

#### Scenario: Several entries form one accepted outcome

- **WHEN** the user accepts an `n:1` transformation
- **THEN** the skill creates or updates one coherent OpenSpec owner with traceable provenance
- **AND** removes only the explicitly approved source entries after validation

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


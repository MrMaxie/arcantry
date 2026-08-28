## ADDED Requirements

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

## MODIFIED Requirements

### Requirement: Content safety skills protect audience and substance

The content-safely family MUST support audience and scope discipline, graphical interface composition, terminal experience design, and product content writing. Content workflows MUST prevent private context leakage, preserve established artifact contracts, and remove vague, repetitive, process-centered, or unsupported writing that does not help the intended reader.

#### Scenario: Product-facing content is prepared

- **WHEN** an agent writes or revises content for a defined audience
- **THEN** the result leads with the reader's outcome, uses evidence-backed concrete language, excludes internal process commentary, and preserves approved presentation

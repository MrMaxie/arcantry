## ADDED Requirements

### Requirement: Documentation projections are build outputs

Per-skill pages, generated skill navigation and public schema copies MUST be derived from their canonical repository sources before documentation development, checking and building. These documentation-only projections MUST remain untracked in Git, while the deployed site MUST include them at their established public routes.

#### Scenario: Documentation builds from a clean checkout

- **WHEN** a contributor or CI builds documentation without generated documentation files on disk
- **THEN** generation runs before Astro consumes the content
- **AND** every canonical skill page, generated navigation item and public schema is present in the build

#### Scenario: Generated documentation is inspected in Git

- **WHEN** generation completes against unchanged canonical inputs
- **THEN** documentation-only output remains ignored and untracked
- **AND** tracked plugin manifests remain independently drift-checked

## MODIFIED Requirements

### Requirement: Documentation links and presentation remain stable

The sidebar MUST use grouped Start, Guides, Concepts, Reference, and Contributing to Arcantry sections. Internal links and fragments from the overview and sidebar MUST resolve in the built site. Moving or generating documentation source files MUST preserve established public routes. Content MUST describe the current Arcantry contract directly and MUST preserve the approved Starlight shell, typography, animation, components, and visual composition except for copy, information order, commands, and links required by the product contract.

#### Scenario: A reader navigates the documentation

- **WHEN** they follow an overview action or sidebar item
- **THEN** the intended existing route, page or section exists
- **AND** the established documentation shell remains visually unchanged

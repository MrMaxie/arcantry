## ADDED Requirements

### Requirement: Release artifacts support multiple consumer outcomes

A release-bearing OpenSpec change MAY declare one or more consumer outcomes. Each outcome MUST have exactly one standard changelog category, a non-empty title and non-empty consumer prose. SemVer impact, visibility, components, unit impacts and dependency updates MUST remain owned by the change as a whole. Release manifests MUST continue to assign change ids rather than individual outcomes.

The canonical multi-outcome `release.md` body MUST use unique level-two standard category headings containing one or more level-three outcome headings. Content before the first category, prose without an outcome heading, duplicate category sections, unknown categories, empty titles and empty bodies MUST be rejected. A legacy artifact with `category` frontmatter, one level-one title and one body MUST remain valid and MUST NOT be rewritten merely because the canonical format exists. One artifact MUST NOT mix the legacy and canonical structures.

#### Scenario: One change has added and fixed outcomes

- **WHEN** one valid release artifact declares distinct outcomes under `Added` and `Fixed`
- **THEN** release planning uses the change's single effective SemVer impact
- **AND** changelog rendering emits each outcome separately under its declared category
- **AND** every emitted outcome remains traceable to the same archived change id

#### Scenario: An existing single-outcome artifact is read

- **WHEN** a legacy release artifact declares `category` frontmatter, one level-one title and consumer prose
- **THEN** parsing and rendering preserve its existing release meaning
- **AND** validation does not require migration to the canonical format

#### Scenario: A release artifact mixes structures

- **WHEN** a release artifact declares legacy `category` frontmatter and canonical category sections
- **THEN** validation rejects the artifact instead of selecting one interpretation

#### Scenario: Change-wide metadata applies to several outcomes

- **WHEN** a multi-outcome artifact matches a release unit
- **THEN** every outcome shares the change's visibility and component ownership
- **AND** the manifest assigns the change exactly once

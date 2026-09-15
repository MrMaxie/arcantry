# Approach

Keep change-wide release metadata in YAML frontmatter. The canonical body uses level-two Keep a Changelog category headings and level-three outcome headings:

```md
---
impact: minor
visibility: public
components:
  - repository-lifecycle
---

## Added

### First consumer outcome

Describe the first result.

## Fixed

### Second consumer outcome

Describe the independent fix.
```

The six existing categories remain the only allowed category headings. Each category section must be unique, each outcome needs a non-empty title and body, and content before the first category or between a category and its first outcome is invalid. A file must use either this canonical structure or the legacy `category` frontmatter plus level-one title structure, never both.

Extend the parsed artifact additively with optional outcomes. Existing scalar `category`, `title`, and `body` fields continue to expose the first outcome so existing callers remain source-compatible. Rendering and new callers use the complete outcome list, falling back to the scalar fields for legacy or programmatically constructed artifacts.

Release manifests continue to assign OpenSpec change ids. SemVer impact, visibility, components, unit impacts, and dependency updates remain change-wide because splitting those responsibilities would turn an outcome list into a second manifest model. Every rendered outcome repeats the same OpenSpec provenance marker.

# Compatibility

Existing release files remain byte-for-byte valid and render identically. The new body format is opt-in per release artifact and is supported by both existing release adapters. The separate audience-based projection change remains responsible for many-to-one grouping, omission, audience classification, and migration.

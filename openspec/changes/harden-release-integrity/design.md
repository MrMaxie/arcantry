# Decisions

## Release manifests are generated state

`release cut` owns manifest creation. A manifest remains intentionally small: version, date and ordered change IDs.

## Validation is centralized

The release module validates manifests, archive references and assignment uniqueness before planning, cutting or rendering. Commands reuse the same validation path instead of implementing separate rules.

## Changelog drift is a check failure

`release check` renders the changelog in memory and compares it with the committed `CHANGELOG.md`. It does not rewrite files.

## Source traceability stays lightweight

Rendered public entries include the OpenSpec change ID in an unobtrusive HTML comment. This preserves provenance without adding visual noise to the changelog.

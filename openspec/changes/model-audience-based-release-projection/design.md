# Approach

Add a new versioned release adapter while retaining `openspec-release@1` and `openspec-release@2`. Its change metadata separates four dimensions: SemVer impact, affected components, intended audiences and observable impact. A distinct inclusion policy decides whether a change participates in changelog projection.

An included change may own one entry or join an explicit projection group. Each group has a stable id, compatible audience and category, one designated prose owner and an ordered list of member change ids. Rendering emits one entry and records traceability to every member. Omitted changes remain release-bearing, assigned to manifests and included in version calculation and validation.

Migration first parses and preserves the existing history, then reports category mappings, managed baseline, source markers, projection groups, omissions and unresolved entries. Apply requires an accepted serializable plan and updates the adapter only after the target history validates. Deliberate differences may be preserved explicitly rather than normalized as accidental drift.

# Trade-offs

Independent classification dimensions are more expressive but add metadata and validation rules. An opt-in adapter avoids breaking existing repositories, while explicit projection groups prevent heuristic consolidation from changing release meaning.

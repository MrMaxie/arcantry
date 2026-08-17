# Why

The staged code review skill could claim completion for an entire repository or other broad surface without first accounting for its material components and contracts. A review could therefore be careful within the files it inspected while still presenting partial coverage as complete.

# What changes

- Require complete-surface reviews to derive a coverage list from the repository tree, manifests and source-of-truth documentation.
- Require every material component and contract to be marked reviewed or explicitly unreviewed before claiming complete coverage.
- Preserve the skill's approval gate, read-only review phase and finding staging behavior.

# Out of scope

- Requiring exhaustive coverage for a user-selected narrow diff or file set.
- Publishing findings or changing authorization boundaries.
- Prescribing one implementation-specific review checklist.

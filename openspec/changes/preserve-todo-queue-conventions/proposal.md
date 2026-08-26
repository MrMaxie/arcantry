# Why

A captured task can satisfy todo.txt syntax while violating the selected queue's vocabulary and usage conventions. Current contracts prevent invented optional metadata by default, but they do not ensure that a writer detects an established local convention, avoids a redundant project tag or asks before omitting metadata the queue owner consistently relies on.

# What changes

- Distinguish official todo.txt syntax from an explicitly configured or unambiguous queue-local capture convention.
- Inspect the selected queue before capture and preserve its established vocabulary without inventing new projects, contexts or metadata keys.
- Require an exact preview of the resulting task line and identify every optional field and its source.
- Stop for a decision when required local metadata or taxonomy remains ambiguous.
- Keep the official one-task-per-line baseline when no compatible convention is established.

# Out of scope

- Making neighboring optional metadata mandatory solely because it appears frequently.
- Normalizing existing queue entries.
- Creating a project-wide taxonomy without explicit authority.

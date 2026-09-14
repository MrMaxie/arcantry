# Why

A captured task can satisfy todo.txt syntax while violating the selected queue's vocabulary and usage conventions. The product also lacks explicit date and manual deferral, active-task selection, and a deterministic bridge from approved todo intent into OpenSpec.

# What changes

- Distinguish official todo.txt syntax from an explicitly configured or unambiguous queue-local capture convention.
- Inspect the selected queue before capture and preserve its established vocabulary without inventing new projects, contexts or metadata keys.
- Require an exact preview of the resulting task line and identify every optional field and its source.
- Stop for a decision when required local metadata or taxonomy remains ambiguous.
- Keep the official one-task-per-line baseline when no compatible convention is established.
- Add `t:YYYY-MM-DD` and `wait:<slug>` deferral with explicit resume and full queue visibility.
- Let only `arcantry next` filter deferred tasks and select the first active todo when OpenSpec has no actionable change.
- Split promotion ownership: the skill classifies and obtains item-level approval, while the CLI snapshot and project-plan contracts provide hashes, drift rejection, preview and atomic apply.
- Keep shared and private provenance at the source's visibility.

# Out of scope

- Making neighboring optional metadata mandatory solely because it appears frequently.
- Normalizing existing queue entries.
- Creating a project-wide taxonomy without explicit authority.
- Automatically deciding that a todo entry is product intent.

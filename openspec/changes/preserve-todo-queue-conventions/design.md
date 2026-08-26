# Approach

Resolve capture conventions in precedence order: explicit user instruction, selected-source configuration, applicable repository guidance, then an unambiguous convention reported from comparable active queue entries. Frequency alone may suggest a choice but cannot silently make an optional field mandatory.

Before apply, render the exact physical line and annotate the origin of priority, creation date, project, context and `key:value` metadata. Reuse only established tokens whose meaning matches the task. If a required field or token is ambiguous, report the mutually exclusive alternatives and leave the source unchanged. If no compatible convention is established, use the official todo.txt baseline without optional metadata.

# Trade-offs

Queue-aware capture takes an extra inspection and may require one user decision. That cost prevents locally misleading entries while retaining todo.txt's portable and permissive syntax.

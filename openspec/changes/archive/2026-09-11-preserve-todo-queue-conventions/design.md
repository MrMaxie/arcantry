# Approach

Resolve capture conventions in precedence order: explicit user instruction, selected-source configuration, applicable repository guidance, then an unambiguous convention reported from comparable active queue entries. Frequency alone may suggest a choice but cannot silently make an optional field mandatory.

Before apply, render the exact physical line and annotate the origin of priority, creation date, project, context and `key:value` metadata. Reuse only established tokens whose meaning matches the task. If a required field or token is ambiguous, report the mutually exclusive alternatives and leave the source unchanged. If no compatible convention is established, use the official todo.txt baseline without optional metadata.

Use `t:YYYY-MM-DD` as a local-date threshold and `wait:<slug>` as an opaque manual condition. `todo list` remains a complete inventory. Only `next` filters future thresholds and every waiting task. On the threshold date the task is active. Resume removes both deferral markers.

Promotion begins from `todo list --json`, which records the queue content hash and each selected line digest. The skill assigns stable `S` and `T` ids, classifies semantics and privacy, proposes mappings, and obtains item-level approval. It then prepares one serializable project plan whose watched inputs, target writes, provenance and source disposition apply atomically through the existing planner. Shared provenance never carries private source identity, raw text or hashes.

# Trade-offs

Queue-aware capture and promotion take an extra inspection and may require one user decision. That cost prevents locally misleading entries while retaining todo.txt's portable and permissive syntax. Manual wait conditions remain deliberately opaque instead of creating a condition engine.

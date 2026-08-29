# Why

CLI expectations can originate in authored documentation or todo.txt before they become accepted engineering intent. Today an item can be removed from todo.txt and a documentation claim can be edited without a durable machine-checked link to the OpenSpec requirement and executable evidence that replaced it. That makes it possible to lose or narrow the user's intended behavior while every individual file still validates.

# What changes

- Record stable provenance from CLI-facing documentation and promoted todo.txt expectations to accepted OpenSpec requirements.
- Require every public CLI command and trust claim to identify both its normative requirement and executable evidence.
- Preserve the exact meaning of a fulfilled CLI todo item in a tracked promotion record before removing it from the queue.
- Fail verification when a source expectation, accepted requirement, public claim and executable result no longer agree.

# Out of scope

- Making todo.txt the permanent normative specification after promotion.
- Requiring every exploratory or non-CLI todo item to become OpenSpec.
- Generating user-facing documentation prose from implementation metadata.
- Reopening already delivered behavior unless its provenance or evidence is incomplete.

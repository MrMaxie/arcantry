# Why

CLI expectations can be edited independently across authored documentation, OpenSpec and native tests. Without a durable machine-checked link between those layers, a public claim can lose or narrow the accepted behavior while every individual file still validates.

# What changes

- Record stable provenance from CLI-facing documentation to accepted OpenSpec requirements.
- Require every public CLI command and trust claim to identify both its normative requirement and executable evidence.
- Fail verification when a source expectation, accepted requirement, public claim and executable result no longer agree.

# Out of scope

- Todo classification, promotion and source disposition, which are owned by the todo promotion workflow.
- Generating user-facing documentation prose from implementation metadata.
- Reopening already delivered behavior unless its provenance or evidence is incomplete.

# Why

The CLI reference can drift from the executable and even contradict another documentation page. Markdown option alternatives containing vertical bars can also render as extra table columns, making the copied command ambiguous.

# What changes

- Verify every documented command syntax against the executable CLI inventory.
- Test rendered command tables so option alternatives remain literal and column counts remain stable.
- Align release-check documentation with its normal and sealed modes.
- Require public CLI safety, removal and rollback claims to be backed by named executable evidence or narrowed.

# Out of scope

- Redesigning the documentation site.
- Publishing platform support that has not been executed on the declared target.
- Expanding the product trust review beyond CLI behavior and repository effects.

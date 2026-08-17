# Decisions

## Managed content is validated against canonical bodies

Repository validation compares managed sections with the content Arcantry would generate. A marker alone is insufficient because it cannot prove that the adopted contract is current.

## Doctor and validate share detection but not presentation

Both commands inspect the same repository state. Doctor enriches diagnostics with explicit repair actions, while validate remains a deterministic CI result and never mutates files.

## Runtime and source validation enforce the same catalog contract

The package schemas and repository tooling reject unsupported fields, invalid identifiers, incorrect schema references and out-of-range audience-facing metadata. This keeps generated projections and installed CLI behavior aligned.

## Public commands are the dogfood boundary

CI invokes the built `repo validate` and `skills doctor` commands against Arcantry itself so internal checks cannot substitute for the behavior users receive.

## MODIFIED Requirements

### Requirement: Native compatibility is held by the accepted public contract

The native CLI MUST preserve the command names, options, standard output, diagnostic output, exit codes and filesystem outcomes defined by accepted OpenSpec requirements, authored documentation and the executable CLI contract inventory. Verification MUST execute the compiled Rust binary against independent expectations and MUST NOT use a retired command implementation as an oracle.

#### Scenario: A compatibility expectation changes

- **WHEN** an observable CLI expectation is intentionally changed
- **THEN** the OpenSpec delta, documentation, contract inventory and executable evidence change together
- **AND** the Rust implementation passes the updated independent expectation

#### Scenario: A retired implementation disagrees

- **WHEN** a retired implementation produces a different result from the accepted public contract
- **THEN** it does not redefine the expected native behavior
- **AND** verification remains anchored to the accepted contract

### Requirement: Contract completeness is behavior-dimension based

Every public leaf command MUST have separately identified executable evidence for help, invalid input and successful behavior. Every mutating command MUST additionally have separately identified preview, apply and relevant rejection or rollback evidence. Every evidence id MUST be unique, bound to exactly one command and behavior dimension, and dispatched by the native suite through the compiled binary.

#### Scenario: A command is inventoried incompletely

- **WHEN** a leaf command lacks one required behavior dimension or reuses evidence from another command or dimension
- **THEN** contract verification fails before the change can be accepted

#### Scenario: A mutating command is verified

- **WHEN** repository verification reports the command as contract-complete
- **THEN** its preview and apply outcomes have executed independently
- **AND** a relevant rejection or rollback boundary has executed without leaving partial state

## ADDED Requirements

### Requirement: Rust coverage is a blocking per-file non-regression gate

Repository verification MUST generate line and branch coverage for every production Rust source file with executable code. A reviewed machine-readable policy MUST record a per-file accepted floor. Verification MUST fail when a production file is missing, has no executed lines, lacks branch records where branch instrumentation applies, or falls below either accepted floor. Workspace or directory aggregation MUST NOT conceal a failing file.

#### Scenario: A production module has no evidence

- **WHEN** a production Rust source file has executable lines but none execute during the coverage run
- **THEN** the coverage gate fails and names that file

#### Scenario: Coverage regresses

- **WHEN** line or branch coverage for a production file drops below its reviewed floor
- **THEN** repository CI fails until evidence is restored or the floor is explicitly revised with its own reviewable contract change

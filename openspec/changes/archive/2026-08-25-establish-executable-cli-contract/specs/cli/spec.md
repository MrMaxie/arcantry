## ADDED Requirements

### Requirement: The public CLI contract is independently executable

Arcantry MUST maintain a tracked inventory of every public leaf command and documented syntax. Verification MUST execute the compiled Rust binary against independent expected exit codes, output and filesystem effects. It MUST NOT derive expected behavior from another command implementation.

#### Scenario: The command surface changes

- **WHEN** a public leaf command or documented syntax is added, removed or renamed
- **THEN** verification fails until the inventory, specification, documentation and executable evidence agree

#### Scenario: A command implementation drifts

- **WHEN** the Rust binary produces an unexpected exit code, output stream or filesystem result
- **THEN** contract verification fails even if a retired implementation produces the same result

### Requirement: Contract completeness is scenario based

Every public leaf command MUST have executable help, invalid-argument and successful-behavior evidence. Every mutating command MUST cover preview and apply behavior plus a relevant rejection or rollback boundary. Every command and public trust claim MUST reference an id in the executable scenario ledger, and every ledger entry MUST be dispatched by the native suite for its declared command.

#### Scenario: Scenario evidence is reported

- **WHEN** repository verification claims complete CLI contract evidence
- **THEN** every inventoried command points to executable evidence
- **AND** missing, foreign or differently bound scenario ids fail verification

### Requirement: Serialized plans require explicit apply authority

`repo apply` MUST resolve the current project independently from `--cwd` and `--config` and MUST reject a plan whose canonical root differs. Operations inside that root are authorized. Every operation outside it MUST match one separately supplied exact `--allow-outside` path after link-aware canonicalization of the nearest existing ancestor.

#### Scenario: A plan requests an unauthorized path

- **WHEN** a plan root differs or an operation targets an external parent, child, sibling or link escape without its own exact authorization
- **THEN** apply fails before any write

#### Scenario: One exact external operation is authorized

- **WHEN** the plan root matches the current project and `--allow-outside` names that exact canonical operation path
- **THEN** that operation may proceed
- **AND** the authorization does not cover any related path

### Requirement: Public filesystem mutations are transactional

Every public command that changes files or directory links MUST validate all inputs before committing, stage reversible replacements and either commit the complete operation or restore the exact pre-apply filesystem tree. Executable evidence MUST inject failures at bounded staging, commit, verification and reversible finalization positions instead of treating one naturally occurring error as proof of rollback. Cleanup after the verified commit point MUST preserve successful operation status and report any retained backup as a warning.

#### Scenario: A filesystem step fails during apply

- **WHEN** a staged public mutation fails before every operation is committed and verified
- **THEN** the command fails
- **AND** files, directories, links and managed exclusion content match their pre-apply state
- **AND** no transaction temporary or backup artifact remains

#### Scenario: Backup cleanup fails after the commit point

- **WHEN** every operation and reversible finalization step has completed but backup cleanup fails
- **THEN** apply remains successful
- **AND** stderr identifies the retained cleanup artifact

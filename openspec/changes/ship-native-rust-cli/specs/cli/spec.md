## MODIFIED Requirements

### Requirement: Arcantry exposes one namespaced command line interface

Arcantry MUST expose one public `arcantry` command. The authoritative command implementation MUST be a compiled Rust executable that does not require Node.js, Bun, Python or another language runtime. Repository operations MUST be nested under `arcantry repo`, and skill operations MUST be nested under `arcantry skills`. The CLI MUST NOT expose top-level install or update aliases. Package-manager launchers MAY dispatch to the native executable but MUST NOT provide a separate command implementation.

#### Scenario: A user inspects the command surface

- **WHEN** the user runs `arcantry --help` through a native archive or supported package launcher
- **THEN** the help identifies the same public command groups and options
- **AND** no alternate binary or top-level install/update command is required

#### Scenario: A user runs the direct executable

- **WHEN** the user runs a supported native `arcantry` executable without Node.js, Bun or Python installed
- **THEN** every supported CLI command remains available
- **AND** the executable does not attempt to install or invoke another language runtime

## ADDED Requirements

### Requirement: Native migration preserves observable CLI behavior

The native CLI MUST preserve the existing command names, options, standard output, diagnostic output, exit codes and filesystem outcomes for every supported scenario. Migration verification MUST compare both implementations through the same black-box fixtures before the Rust executable becomes the public implementation.

#### Scenario: A command is exercised during migration

- **WHEN** the TypeScript and Rust implementations receive the same arguments and fixture repository
- **THEN** their observable outputs, exit status and resulting filesystem state are equivalent
- **AND** any intentional contract change requires a separate specification delta

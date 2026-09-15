## MODIFIED Requirements

### Requirement: Arcantry exposes one namespaced command line interface

Arcantry MUST expose one public `arcantry` command. All command behavior and domain operations MUST be implemented by the compiled Rust executable and its Rust core. The executable MUST NOT require Node.js, Bun, Python or another language runtime. Repository operations MUST be nested under `arcantry repo`, and skill operations MUST be nested under `arcantry skills`. The CLI MUST NOT expose top-level install or update aliases. Package-manager launchers MAY dispatch to the native executable but MUST NOT provide a separate command or domain implementation.

#### Scenario: A user inspects the command surface

- **WHEN** the user runs `arcantry --help` through a native archive or supported package launcher
- **THEN** the help identifies the same public command groups and options
- **AND** no alternate binary or top-level install/update command is required

#### Scenario: A user runs the direct executable

- **WHEN** the user runs a supported native `arcantry` executable without Node.js, Bun or Python installed
- **THEN** every supported CLI command remains available
- **AND** the executable does not attempt to install or invoke another language runtime

#### Scenario: A new command capability is implemented

- **WHEN** Arcantry adds or changes public command behavior
- **THEN** the behavior is implemented in Rust and exercised through the compiled executable
- **AND** no TypeScript or JavaScript command implementation is added

# cli Specification

## Purpose
Define the stable public command surface for repository adoption and skill discovery.
## Requirements
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

### Requirement: Repository commands have stable responsibilities

The `repo` group MUST expose `inspect`, `plan`, `apply`, `init`, `update`, `doctor`, `validate`, and `remove`. `repo inspect` MUST be read-only, MUST support concise and detailed human views and stable JSON output, and MUST report the resolved project context without requiring configuration. `init`, `update`, and `remove` MUST require `--scope shared|private` and MUST report the repository artifacts they create, update, or remove. `doctor` and `validate` MUST remain read-only and MUST use explicit or discovered TOML configuration.

#### Scenario: Private repository state is initialized

- **WHEN** the user runs `arcantry repo init --scope private` in a Git repository
- **THEN** Arcantry creates or updates only the private configuration, private managed guidance, and local Git exclusion required by that scope
- **AND** it does not create a Claude compatibility file unless `--compat claude` is supplied

#### Scenario: Shared Claude compatibility is requested

- **WHEN** the user runs `arcantry repo update --scope shared --compat claude`
- **THEN** Arcantry preserves `AGENTS.md` as the canonical guidance
- **AND** ensures `CLAUDE.md` imports `@AGENTS.md` without replacing user-authored Claude content

#### Scenario: Private Claude compatibility is requested

- **WHEN** the user runs `arcantry repo update --scope private --compat claude`
- **THEN** Arcantry ensures `CLAUDE.local.md` imports `@.local/AGENTS.md`
- **AND** excludes the adapter through local Git metadata without changing `.gitignore`

#### Scenario: Repository state is validated

- **WHEN** the user runs `arcantry repo validate` in a configured repository
- **THEN** Arcantry validates managed artifact metadata and repository policy without changing files

#### Scenario: Repository context is inspected concisely

- **WHEN** a user runs repository inspection without a detail override
- **THEN** the command reports the project boundary, selected configuration, recognized and absent standard sources, methodology summary and `.local` boundary health
- **AND** does not change repository or private state

#### Scenario: Repository context is consumed by an agent

- **WHEN** a caller requests JSON output
- **THEN** the result contains stable typed records for every detailed context field
- **AND** no operating-system-specific command output is exposed as the contract

### Requirement: Skill commands support discovery and local adoption

The `skills` group MUST expose `list`, `inspect`, `link`, `unlink`, and `doctor`. Link and unlink operations MUST target one named canonical skill and MUST be idempotent for an already-correct state. The linker MUST support `--scope user|repo|private`, MUST use standard `.agents/skills` destinations, MAY accept `--compat claude` for an additional alias, and MAY accept an exclusive advanced `--target` path. The CLI MUST NOT expose provider profiles through `--agent`.

#### Scenario: One skill is linked for a repository

- **WHEN** the user runs `arcantry skills link <name> --scope repo` for a valid catalog entry
- **THEN** `<repo>/.agents/skills/<name>` points to that canonical package
- **AND** repeating the command does not create a duplicate installation

#### Scenario: One skill is linked

- **WHEN** the user runs `arcantry skills link <name> --scope user` for a valid catalog entry
- **THEN** `~/.agents/skills/<name>` points to that canonical package
- **AND** repeating the command does not create a duplicate installation

#### Scenario: One private skill is linked

- **WHEN** the user runs `arcantry skills link <name> --scope private` for a valid package under `.local/skills`
- **THEN** the repository `.agents/skills/<name>` destination points to that private canonical package
- **AND** the managed destination is excluded through local Git metadata

#### Scenario: Claude compatibility is requested

- **WHEN** the user links a skill with `--compat claude`
- **THEN** the standard `.agents/skills` link and the corresponding `.claude/skills` link resolve to the same canonical package
- **AND** failure to prepare either destination leaves no new partial installation

#### Scenario: The destination is explicit

- **WHEN** the user supplies `--target`
- **THEN** Arcantry uses that destination instead of a scope-derived directory
- **AND** rejects simultaneous `--scope` or `--compat`

### Requirement: Native migration preserves observable CLI behavior

The native CLI MUST preserve the command names, options, standard output, diagnostic output, exit codes and filesystem outcomes defined by accepted OpenSpec requirements, authored documentation and the executable CLI contract inventory. Verification MUST execute the compiled Rust binary against independent expectations and MUST NOT use a retired command implementation as an oracle.

#### Scenario: A compatibility expectation changes

- **WHEN** an observable CLI expectation is intentionally changed
- **THEN** the OpenSpec delta, documentation, contract inventory and executable evidence change together
- **AND** the Rust implementation passes the updated independent expectation

#### Scenario: A retired implementation disagrees

- **WHEN** a retired implementation produces a different result from the accepted public contract
- **THEN** it does not redefine the expected native behavior
- **AND** verification remains anchored to the accepted contract

#### Scenario: A command is exercised during migration

- **WHEN** the native implementation receives arguments and a fixture repository covered by the accepted public contract
- **THEN** its observable outputs, exit status and resulting filesystem state match that contract
- **AND** any intentional contract change requires a separate specification delta

### Requirement: Release commands expose local release management

The `release` group MUST expose `baseline`, `plan`, `cut`, `render` and `check`. `plan` and `check` MUST be read-only. `baseline`, `cut` and `render` MUST preview a serializable drift-checked plan by default and MUST NOT write unless the caller supplies `--apply`.

#### Scenario: A release mutation is previewed

- **WHEN** a user runs `release baseline`, `release cut` or `release render` without `--apply`
- **THEN** Arcantry reports the planned operations and leaves project files unchanged

#### Scenario: A release mutation is applied

- **WHEN** a user supplies `--apply` for a conflict-free release plan
- **THEN** Arcantry applies all planned writes atomically using recorded input preconditions

#### Scenario: A project release is inspected

- **WHEN** a user runs `release plan` or `release check`
- **THEN** Arcantry reports release state without committing, tagging, pushing or publishing

### Requirement: Multi-unit release commands select their release unit

In `independent` and `composed` topologies, `release baseline`, `release plan`, `release cut` and `release render` MUST require `--unit <id>`. `release check` without a unit MUST check consistency across all units. `release check --sealed` MUST require `--unit <id>`. Single topology behavior MUST remain unchanged.

#### Scenario: A multi-unit plan omits its unit

- **WHEN** a user runs `release plan` for a multi-unit project without `--unit`
- **THEN** the command fails without changing project files

#### Scenario: All multi-unit state is checked

- **WHEN** a user runs `release check` without `--sealed` or `--unit`
- **THEN** Arcantry validates the consistency of every configured release unit

### Requirement: Release plans expose unit and dependency readiness

Serializable release-plan output MUST include `unit`, `topology`, exact current dependency versions, newer `pendingDependencies` and `ready`. A composed parent plan MUST be ready to adopt a pending dependency only when a selected parent change acknowledges that direct dependency through `dependency_updates`.

#### Scenario: A dependency released after its parent

- **WHEN** a user plans the parent unit and a direct dependency has a newer manifest than the parent's pin
- **THEN** JSON output reports the newer version in `pendingDependencies`
- **AND** reports whether selected parent outcomes make the plan ready

### Requirement: The public CLI contract is independently executable

Arcantry MUST maintain a tracked inventory of every public leaf command and documented syntax. Verification MUST execute the compiled Rust binary against independent expected exit codes, output and filesystem effects. It MUST NOT derive expected behavior from another command implementation.

#### Scenario: The command surface changes

- **WHEN** a public leaf command or documented syntax is added, removed or renamed
- **THEN** verification fails until the inventory, specification, documentation and executable evidence agree

#### Scenario: A command implementation drifts

- **WHEN** the Rust binary produces an unexpected exit code, output stream or filesystem result
- **THEN** contract verification fails even if a retired implementation produces the same result

### Requirement: Contract completeness is scenario based

Every public leaf command MUST have separately identified executable evidence for help, invalid input and successful behavior. Every mutating command MUST additionally have separately identified preview, apply and relevant rejection or rollback evidence. Every evidence id MUST be unique, bound to exactly one command and behavior dimension, and dispatched by the native suite through the compiled binary.

#### Scenario: A command is inventoried incompletely

- **WHEN** a leaf command lacks one required behavior dimension or reuses evidence from another command or dimension
- **THEN** contract verification fails before the change can be accepted

#### Scenario: A mutating command is verified

- **WHEN** repository verification reports the command as contract-complete
- **THEN** its preview and apply outcomes have executed independently
- **AND** a relevant rejection or rollback boundary has executed without leaving partial state

#### Scenario: Scenario evidence is reported

- **WHEN** repository verification claims complete CLI contract evidence
- **THEN** every inventoried command points to executable evidence for every required behavior dimension
- **AND** missing, foreign, reused or differently bound evidence ids fail verification

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

### Requirement: Documented syntax matches the public command inventory

Every public CLI command row and syntax block MUST match the tracked executable contract inventory. Every fenced command and table cell beginning with `arcantry` in any Markdown or MDX document MUST parse through the same Clap model as the binary. Documentation verification MUST fail for a missing, additional, differently named or unparsable command or option.

#### Scenario: Documentation and the binary disagree

- **WHEN** authored CLI syntax differs from the command inventory used by native verification
- **THEN** repository checks fail before the documentation can be published

### Requirement: Release commands use one final adapter contract

Release planning, rendering, checking and cutting MUST use `openspec-release@1` for every supported topology. Commands MUST preserve manifest assignment and SemVer impact for changes classified with `changelog: omit`, and rendering MUST use group-owned prose for valid projection groups.

#### Scenario: Maintenance is omitted from prose

- **WHEN** an accepted maintenance change uses `changelog: omit`
- **THEN** release planning still assigns it and includes its SemVer impact
- **AND** release rendering emits no entry for it

#### Scenario: A group owns a release story

- **WHEN** all group members are assigned to one release
- **THEN** rendering emits the group's title and body once
- **AND** preserves source markers for every member

### Requirement: Public CLI evidence is traceable to accepted intent

Every inventoried public CLI command and public CLI trust claim MUST reference an accepted OpenSpec requirement, its authored documentation location and executable evidence. Verification MUST resolve every reference and MUST fail when the documentation, requirement, inventory or observed native behavior disagrees.

#### Scenario: A public CLI claim changes

- **WHEN** authored documentation adds, removes or materially changes a CLI behavior or trust claim
- **THEN** verification requires the corresponding OpenSpec delta and executable evidence update
- **AND** the claim cannot pass solely because its command syntax still parses

#### Scenario: A referenced requirement moves

- **WHEN** a requirement or documentation anchor is renamed or removed
- **THEN** stale provenance fails verification until the reference is deliberately updated

### Requirement: Rust coverage is an optional per-file diagnostic

The optional coverage workflow SHOULD generate line and branch coverage for every production Rust source file with executable code and SHOULD report missing or regressed evidence per file. Coverage MUST NOT block implementation, continuous delivery, publication or release authorization. Workspace or directory aggregation MUST NOT conceal a file in the diagnostic report.

#### Scenario: A production module has no evidence

- **WHEN** a production Rust source file has executable lines but none execute during the coverage run
- **THEN** the coverage diagnostic names that file

#### Scenario: Coverage regresses

- **WHEN** line or branch coverage for a production file drops below its reviewed floor
- **THEN** the diagnostic reports the regression without changing the status of required host, Linux or release-target gates

### Requirement: Skill status and update operations expose machine-readable contracts

The native CLI MUST provide `skills status [<name>] [--scope <user|repo>] [--target <path>] [--catalog-root <path>] [--check] [--json]`, `skills update <name> [--scope <user|repo>] [--target <path>] [--catalog-root <path>] [--compat claude]` and `skills apply --plan <path|->`. The global `--output` option MUST save an update preview without overwriting an existing file. Invalid scope, target and compatibility combinations MUST fail before network or filesystem mutation.

#### Scenario: Automation requests installed skill state

- **WHEN** `skills status --json` is invoked
- **THEN** stdout contains stable structured records and no human prose
- **AND** the command does not contact the network unless `--check` is supplied

#### Scenario: A saved update plan is applied

- **WHEN** `skills apply --plan` receives a valid unchanged plan
- **THEN** only the planned skill targets and ownership receipt change
- **AND** the command reports the applied version, revision and digest

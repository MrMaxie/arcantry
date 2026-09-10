## ADDED Requirements

### Requirement: Repository commands expose environment contracts without executing Varlock

`arcantry repo inspect` MUST report discovered and configured environment-contract source metadata in human and JSON output without including schema contents, defaults, resolver references or resolved values. `repo inspect`, `repo validate` and `repo doctor` MUST NOT invoke Varlock, load environment value files, contact secret providers, install plugins or write Varlock-owned cache. Validation MUST state that an observed environment contract was not functionally validated by Arcantry.

#### Scenario: Inspection finds a shared environment contract

- **WHEN** `repo inspect` finds `.env.schema`
- **THEN** it reports the stable source id, `environment-schema` kind, path, shared visibility, `observe` management and `env-spec@1` adapter
- **AND** outputs none of the file's content or neighboring environment values

#### Scenario: Inspection finds a private environment contract

- **WHEN** `repo inspect` finds `.local/.env.schema`
- **THEN** it reports the source as private without rendering its content
- **AND** verifies the normal `.local` repository-policy boundary without enumerating unrelated private files

#### Scenario: Varlock is unavailable

- **WHEN** repository inspection, validation or doctor runs without a Varlock executable
- **THEN** Arcantry still reports the environment-contract source deterministically
- **AND** does not treat external Varlock availability as an Arcantry repository error

#### Scenario: Repository validation includes an observed environment contract

- **WHEN** `repo validate` processes a configured `environment-schema` source with `observe` management
- **THEN** it validates only Arcantry configuration and source-presence metadata
- **AND** reports no claim that the schema or its resolved environment is valid

### Requirement: Environment-dependent operations degrade gracefully without Varlock

An Arcantry operation MUST load environment values only when its contract explicitly requires them. Existing process-environment values MUST remain authoritative. When an existing project Varlock setup and a supported integration are available, Arcantry MUST attempt to use Varlock for missing values. If Varlock is unavailable, its version or schema is incompatible, or resolution fails, Arcantry MUST continue with dotenv-compatible loading from the project-root `.env` and MUST fill only keys that remain unset. An optional Varlock failure MUST NOT fail the operation unless an input required by the operation remains unavailable after fallback. Loaded values MUST NOT appear in normal output, JSON output or diagnostics.

#### Scenario: Compatible Varlock setup is available

- **WHEN** an operation requires environment values and detects an existing supported Varlock setup
- **THEN** Arcantry attempts to obtain missing values through Varlock
- **AND** preserves values already supplied by the calling process

#### Scenario: Varlock cannot be used

- **WHEN** Varlock is missing, reports an unsupported version or schema, or fails while resolving values
- **THEN** Arcantry treats the Varlock path as unavailable and attempts dotenv-compatible project-root `.env` loading
- **AND** continues the operation when its required inputs are satisfied after fallback

#### Scenario: Only dotenv configuration is present

- **WHEN** an operation requires an unset environment value, no usable Varlock setup exists, and the project-root `.env` defines it
- **THEN** Arcantry loads the missing value from `.env`
- **AND** does not require Varlock installation or configuration

#### Scenario: A required value remains unavailable

- **WHEN** Varlock and dotenv fallback complete without providing a value required by the operation
- **THEN** the operation reports its missing required input
- **AND** does not present Varlock incompatibility as the cause when the same input is absent from every supported source

#### Scenario: An operation does not need environment values

- **WHEN** an Arcantry operation has no declared environment-value dependency
- **THEN** it does not invoke Varlock or read `.env`
- **AND** Varlock availability or compatibility cannot affect its outcome

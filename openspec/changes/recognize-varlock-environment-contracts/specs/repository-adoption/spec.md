## ADDED Requirements

### Requirement: Varlock integration does not expand repository adoption

Repository initialization and update MUST NOT install, update, configure or execute Varlock, create an environment schema, modify package manifests, add hooks, or connect Varlock to runtime, build, CI or publication workflows. Registering an existing environment contract MUST require an explicit source-adoption plan and MUST preserve Varlock-owned files unchanged. Best-effort use of a pre-existing Varlock setup by a later environment-dependent Arcantry operation MUST NOT expand what adoption writes or manages.

#### Scenario: A Varlock project adopts Arcantry

- **WHEN** repository initialization finds an existing `.env.schema`
- **THEN** initialization creates only the selected Arcantry configuration and managed guidance boundary
- **AND** does not modify the schema, environment value files or Varlock setup

#### Scenario: A project has no environment contract

- **WHEN** repository initialization or update runs without `.env.schema`
- **THEN** the environment contract remains absent
- **AND** no package, hook, runtime, build or CI integration is added

#### Scenario: Environment-contract registration is requested

- **WHEN** a user explicitly plans adoption of an existing `environment-schema` source
- **THEN** the plan changes only the active Arcantry configuration
- **AND** leaves installation and dedicated Varlock administration to separate user-authorized workflows
- **AND** does not preconfigure or require the optional runtime-loading path

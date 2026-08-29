## MODIFIED Requirements

### Requirement: Repository commands are stable

The repository MUST keep a root `justfile` as the only task runner and expose stable recipes for checking, building, serving documentation, validating changes, planning releases, cutting releases and rendering the changelog. mise MUST pin and provision `just` and Nub. The recipes MUST invoke package management and the underlying repository tools directly through Nub without routing through root package scripts. Dependency installation, Node provisioning and repository tool execution MUST NOT require pnpm.

#### Scenario: A contributor inspects repository commands

- **WHEN** they install the pinned tools with mise and list or use the documented `just` recipes
- **THEN** check, build, serve, validation, release planning, release cutting and changelog rendering remain available

#### Scenario: CI starts from a clean checkout

- **WHEN** a GitHub-hosted runner checks out the repository
- **THEN** mise provisions the pinned `just` and Nub versions
- **AND** `just ci-setup` uses Nub to provision the repository's Node version, expose it to later workflow steps and install the frozen workspace lockfile
- **AND** CI runs the same `just ci` quality gate used by contributors

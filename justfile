set dotenv-load := false
set indentation := "  "

_default:
  @just --list

setup:
  nub install --frozen-lockfile

[private]
ci-setup: setup
  nub tooling/ci-setup.ts

check:
  nub tooling/generate.ts --check
  nub tooling/generate.ts --docs-only
  nub exec biome check
  nub exec --cwd apps/docs astro sync
  nub exec tsc --noEmit
  nub exec --cwd packages/arcantry tsc -p tsconfig.json --noEmit
  nub exec vitest run tooling --exclude ".local/**"
  nub exec --cwd packages/arcantry vitest run
  nub tooling/validate-catalog.ts
  nub tooling/release.ts check
  nub exec --cwd apps/docs astro check
  just --fmt --check
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  mise exec -- cargo deny check

build:
  nub tooling/generate.ts --docs-only
  nub exec --cwd packages/arcantry tsup
  nub exec --cwd apps/docs astro build
  cargo build --workspace

format:
  just --fmt
  cargo fmt --all
  nub exec biome format --write

[private]
format-check:
  just --fmt --check
  cargo fmt --all -- --check

[private]
rust-clippy:
  cargo clippy --workspace --all-targets -- -D warnings

[private]
rust-test:
  cargo test --workspace

[private]
rust-deny:
  mise exec -- cargo deny check

native-conformance:
  cargo build -p arcantry-cli
  nub tooling/native-conformance.ts

[private]
dist-plan:
  mise exec -- dist plan --tag v1.0.0 --allow-dirty

docs port="9796":
  nub tooling/generate.ts --docs-only
  nub exec --cwd apps/docs astro dev --host 127.0.0.1 --port {{ port }} --force

generate:
  nub tooling/generate.ts

[private]
generate-check:
  nub tooling/generate.ts --check

[private]
catalog-validate:
  nub tooling/validate-catalog.ts

package-check:
  nub exec --cwd packages/arcantry tsup
  nub tooling/prepare-package.ts
  cargo build -p arcantry-cli
  nub tooling/package-smoke.ts --binary target/debug/arcantry{{ if os_family() == "windows" { ".exe" } else { "" } }}

[private]
package-target-smoke target:
  nub exec --cwd packages/arcantry tsup
  nub tooling/prepare-package.ts
  nub tooling/package-smoke.ts --target {{ quote(target) }}

[private]
package-release artifacts output:
  nub exec --cwd packages/arcantry tsup
  nub tooling/prepare-package.ts
  nub tooling/package-native.ts --output {{ quote(output) }} --main --artifacts {{ quote(artifacts) }}

[private]
package-archive output:
  nub exec --cwd packages/arcantry tsup
  nub tooling/prepare-package.ts
  cargo build -p arcantry-cli
  nub tooling/package-smoke.ts --binary target/debug/arcantry{{ if os_family() == "windows" { ".exe" } else { "" } }} --output {{ quote(output) }}

[private]
package-native target binary output:
  nub tooling/package-native.ts --output {{ quote(output) }} --binary {{ quote(target + "=" + binary) }}

[private]
registry-smoke archives:
  nub tooling/registry-smoke.ts --archives {{ quote(archives) }}

[private]
installer-smoke artifacts:
  nub tooling/installer-smoke.ts --artifacts {{ quote(artifacts) }}

[private]
arcantry-build:
  nub exec --cwd packages/arcantry tsup

[private]
arcantry-native-build:
  cargo build -p arcantry-cli

[private]
arcantry-init: arcantry-native-build
  target/debug/arcantry{{ if os_family() == "windows" { ".exe" } else { "" } }} --cwd . repo init --scope private

[private]
arcantry-doctor: arcantry-native-build
  target/debug/arcantry{{ if os_family() == "windows" { ".exe" } else { "" } }} --cwd . repo doctor

[private]
arcantry-validate: arcantry-native-build
  target/debug/arcantry{{ if os_family() == "windows" { ".exe" } else { "" } }} --cwd . repo validate

[private]
arcantry-skills-doctor: arcantry-native-build
  target/debug/arcantry{{ if os_family() == "windows" { ".exe" } else { "" } }} --cwd . skills doctor

release-plan:
  nub tooling/release.ts plan

release-cut:
  nub tooling/release.ts cut

release-render:
  nub tooling/release.ts render

release-check:
  nub tooling/release.ts check

release-seal:
  nub tooling/release.ts seal

publish-check tag:
  nub tooling/publish.ts check --tag {{ quote(tag) }}

openspec-validate:
  nub exec openspec schema validate arcantry
  nub exec openspec validate --all --strict --no-interactive

ci: openspec-validate check build native-conformance dist-plan package-check arcantry-init arcantry-validate arcantry-skills-doctor

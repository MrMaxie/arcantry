set dotenv-load := false
set indentation := "  "

_default:
  @just --list

setup:
  nub install --frozen-lockfile

[private]
ci-setup: setup
  cargo run -p xtask -- ci-setup

check: check-fast
  cargo run -p xtask -- typescript-boundary
  cargo run -p xtask -- generate --check
  cargo run -p xtask -- generate --docs-only
  nub exec biome check
  nub exec --cwd apps/docs astro sync
  cargo run -p xtask -- catalog-validate
  nub exec --cwd apps/docs astro check
  cargo clippy --workspace --all-targets --locked -- -D warnings
  cargo test --workspace --locked
  mise exec -- cargo deny check

check-fast:
  just --fmt --check
  cargo fmt --all -- --check
  cargo check --workspace --locked

check-host: check build package-check

docs-build:
  cargo run -p xtask -- generate --docs-only
  nub exec --cwd apps/docs astro build
  cargo run -p xtask -- docs-output

build:
  cargo run -p xtask -- generate --docs-only
  nub exec --cwd apps/docs astro build
  cargo run -p xtask -- docs-output
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
  cargo test -p arcantry-cli --test cli_contract

native-target-check target tag="v1.0.0":
  cargo test --workspace
  mise exec -- dist build --artifacts=local --target={{ quote(target) }} --tag={{ quote(tag) }} --allow-dirty
  cargo run -p xtask -- smoke-target --target {{ quote(target) }}
  just package-target-smoke {{ quote(target) }}

rust-coverage:
  mise exec rust@nightly-2026-08-24 -- cargo llvm-cov --workspace --locked --branch --lcov --output-path target/rust-coverage.lcov

linux-system-test:
  cargo run -p xtask -- linux-system-test

[private]
dist-plan:
  mise exec -- dist plan --tag v1.0.0 --allow-dirty

docs port="9796":
  cargo run -p xtask -- generate --docs-only
  nub exec --cwd apps/docs astro dev --host 127.0.0.1 --port {{ port }} --force

generate:
  cargo run -p xtask -- generate

[private]
generate-check:
  cargo run -p xtask -- generate --check

[private]
catalog-validate:
  cargo run -p xtask -- catalog-validate

package-check:
  cargo run -p xtask -- prepare-package
  cargo build -p arcantry-cli
  cargo run -p xtask -- package-smoke --binary target/debug/arcantry{{ if os_family() == "windows" { ".exe" } else { "" } }}

[private]
package-target-smoke target:
  cargo run -p xtask -- prepare-package
  cargo run -p xtask -- package-smoke --target {{ quote(target) }}

[private]
package-release artifacts output:
  cargo run -p xtask -- prepare-package
  cargo run -p xtask -- package-native --output {{ quote(output) }} --main --artifacts {{ quote(artifacts) }}

[private]
package-archive output:
  cargo run -p xtask -- prepare-package
  cargo build -p arcantry-cli
  cargo run -p xtask -- package-smoke --binary target/debug/arcantry{{ if os_family() == "windows" { ".exe" } else { "" } }} --output {{ quote(output) }}

[private]
package-native target binary output:
  cargo run -p xtask -- package-native --output {{ quote(output) }} --binary {{ quote(target + "=" + binary) }}

[private]
registry-smoke archives:
  cargo run -p xtask -- registry-smoke --archives {{ quote(archives) }}

[private]
installer-smoke artifacts:
  cargo run -p xtask -- installer-smoke --artifacts {{ quote(artifacts) }}

[private]
arcantry-build:
  cargo build -p arcantry-cli

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
  cargo run -p xtask -- repository-release plan

release-cut:
  cargo run -p xtask -- repository-release cut --apply

release-render:
  cargo run -p xtask -- repository-release render --apply

release-check:
  cargo run -p xtask -- repository-release check

release-seal:
  cargo run -p xtask -- repository-release check --sealed

publish-check tag:
  cargo run -p xtask -- publish check --tag {{ quote(tag) }}

openspec-validate:
  nub exec openspec schema validate arcantry
  nub exec openspec validate --all --strict --no-interactive

ci: openspec-validate check-host linux-system-test

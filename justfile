set dotenv-load := false

_default:
  @just --list

setup:
  nub install --frozen-lockfile

ci-setup: setup
  echo "$(dirname "$(nub node which)")" >> "$GITHUB_PATH"

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

build:
  nub tooling/generate.ts --docs-only
  nub exec --cwd packages/arcantry tsup
  nub exec --cwd apps/docs astro build

docs port="9796":
  nub tooling/generate.ts --docs-only
  nub exec --cwd apps/docs astro dev --host 127.0.0.1 --port {{ port }} --force

generate:
  nub tooling/generate.ts

generate-check:
  nub tooling/generate.ts --check

catalog-validate:
  nub tooling/validate-catalog.ts

package-check:
  nub exec --cwd packages/arcantry tsup
  nub tooling/prepare-package.ts
  nub --node packages/arcantry/scripts/check-package.mjs

package-archive output:
  nub exec --cwd packages/arcantry tsup
  nub tooling/prepare-package.ts
  nub --node packages/arcantry/scripts/check-package.mjs --output {{ quote(output) }}

arcantry-build:
  nub exec --cwd packages/arcantry tsup

arcantry-init: arcantry-build
  nub packages/arcantry/dist/cli.js --cwd . repo init --scope private

arcantry-doctor: arcantry-build
  nub packages/arcantry/dist/cli.js --cwd . repo doctor

arcantry-validate: arcantry-build
  nub packages/arcantry/dist/cli.js --cwd . repo validate

arcantry-skills-doctor: arcantry-build
  nub packages/arcantry/dist/cli.js --cwd . skills doctor

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

ci: openspec-validate check build package-check arcantry-init arcantry-validate arcantry-skills-doctor

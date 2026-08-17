set dotenv-load := false

_default:
    @just --list

setup:
    pnpm install --frozen-lockfile

check:
    pnpm run check

build:
    pnpm run build

docs port="4321":
    pnpm exec astro dev --host 127.0.0.1 --port {{port}}

generate:
    pnpm run generate

generate-check:
    pnpm run generate:check

catalog-validate:
    pnpm run catalog:validate

package-check:
    pnpm run package:check

arcantry-build:
    pnpm --filter @maxiedev/arcantry run build

arcantry-doctor: arcantry-build
    node packages/arcantry/dist/cli.js --cwd . repo doctor

arcantry-validate: arcantry-build
    node packages/arcantry/dist/cli.js --cwd . repo validate

release-plan:
    pnpm run release:plan

release-cut:
    pnpm run release:cut

release-render:
    pnpm run release:render

release-check:
    pnpm run release:check

openspec-validate:
    pnpm exec openspec schema validate arcantry
    pnpm exec openspec validate --all --strict --no-interactive

ci: openspec-validate check build package-check

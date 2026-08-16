set dotenv-load := false

_default:
    @just --list

setup:
    pnpm install

check:
    pnpm run check

build:
    pnpm run build

docs:
    pnpm run dev

release-plan:
    pnpm run release:plan

release-render:
    pnpm run release:render

openspec-validate:
    pnpm exec openspec schema validate arcantry

ci: openspec-validate check build

# Arcantry

[![CI](https://github.com/MrMaxie/arcantry/actions/workflows/ci.yml/badge.svg)](https://github.com/MrMaxie/arcantry/actions/workflows/ci.yml)
[![Documentation](https://img.shields.io/badge/docs-maxie.dev-6a4cc7)](https://maxie.dev/arcantry/)
[![License](https://img.shields.io/github/license/MrMaxie/arcantry)](LICENSE)

Coordinate project knowledge and repeatable agent work without forcing a repository shape.

Arcantry keeps OpenSpec, changelogs, todo.txt queues, agent guidance, and focused skills in distinct roles. Shared and private TOML configuration remain independent, repository adoption stays minimal, and every structural write starts from a reviewable plan.

```sh
arcantry repo inspect
arcantry repo init --scope private
arcantry repo plan --source todo-root --transition relocate --to-path .local/todo.txt --json
arcantry repo apply --plan plan.json
```

## Why Arcantry

- Inspect empty directories, mature repositories and monorepos without requiring Git or configuration.
- Keep accepted intent, consumer release meaning, hot thoughts, private state, and reusable procedures in distinct layers.
- Use focused skills for self-improvement, repository safety, and audience-safe content.
- Preview structural changes as serializable plans and reject changed inputs before writing.
- Use the same deterministic checks locally and in CI.

## Documentation

Start with the [Arcantry documentation](https://maxie.dev/arcantry/). It covers adoption paths, CLI commands, configuration, the skill catalog and the release model.

The normative product and engineering contract lives in [`openspec/`](openspec/). Contributor commands are documented in the [contributor reference](https://maxie.dev/arcantry/reference/commands/).

## Development

The repository uses [mise](https://mise.jdx.dev/) for tool versions, [pnpm](https://pnpm.io/) for dependencies and [just](https://just.systems/) as its command surface.

```sh
mise install
just setup
just check
```

## License

Arcantry is available under the [Apache License 2.0](LICENSE).

# Arcantry

[![Documentation](https://img.shields.io/badge/docs-arcantry.dev-6a4cc7)](https://arcantry.dev/)
[![License](https://img.shields.io/github/license/MrMaxie/arcantry)](LICENSE)

Coordinate project knowledge and repeatable agent work without forcing a repository shape.

Arcantry keeps shared and private OpenSpec, changelogs, todo.txt queues, universal agent guidance, and focused skills in distinct roles. `AGENTS.md` and `.agents` remain the recommended standard, while optional Claude adapters reuse the same sources without copying them.

```sh
arcantry repo inspect
arcantry repo init --scope private
arcantry repo plan --source todo-root --transition relocate --to-path .local/todo.txt --json
arcantry repo apply --plan plan.json
```

## Start with the next useful action

Arcantry helps an agent resume a project without asking you to repeat its rules. It finds the sources, recommends an executable next step and explains the required format.

```sh
arcantry context
arcantry next
arcantry explain tasks
```

Add `--json` for tool integrations. `arcantry mcp` exposes the same reads and release planning over stdio; changes stay in the CLI. Missing optional tools do not block direct work on project files. See the [getting started guide](https://arcantry.dev/getting-started/) for source-based use without installation.

## Why Arcantry

- Inspect empty directories, mature repositories and monorepos without requiring Git or configuration.
- Keep accepted intent, consumer release meaning, hot thoughts, private state, and reusable procedures in distinct layers.
- Use focused skills for self-improvement, repository safety, and audience-safe content.
- Preview structural changes as serializable plans and reject changed inputs before writing.
- Verify work locally on the host and in disposable Linux containers.

## Documentation

Start with the [Arcantry documentation](https://arcantry.dev/). It covers adoption paths, CLI commands, configuration, the skill catalog and the release model.

The normative product and engineering contract lives in [`openspec/`](openspec/). Contributor commands are documented in the [contributor reference](https://arcantry.dev/reference/commands/).

## Development

[mise](https://mise.jdx.dev/) pins and installs `just` and [Nub](https://nubjs.com/). The root `justfile` is the task runner; Nub provisions the Node version declared in `.node-version`, installs dependencies and invokes repository tools.

```sh
mise install
just setup
just check
```

## License

Arcantry is available under the [Apache License 2.0](LICENSE).

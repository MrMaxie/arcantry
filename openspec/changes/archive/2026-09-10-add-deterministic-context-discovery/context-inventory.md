# Context discovery inventory

## Standard sources

| Id | Path | Kind | Visibility | Expected shape | Adapter when absent |
| --- | --- | --- | --- | --- | --- |
| `openspec` | `openspec` | `openspec` | shared | directory | `openspec@1` |
| `openspec-local` | `.local/openspec` | `openspec` | private | directory | `openspec@1` |
| `changelog` | `CHANGELOG.md` | `changelog` | shared | file | `keep-a-changelog@2` |
| `changelog-local` | `.local/CHANGELOG.md` | `changelog` | private | file | `keep-a-changelog@2` |
| `todo-root` | `todo.txt` | `todo-txt` | shared | file | `todo-txt@1` |
| `todo-local` | `.local/todo.txt` | `todo-txt` | private | file | `todo-txt@1` |

Configured sources retain their configured id, path, kind, visibility, management, adapter and dependencies. A configured standard path replaces the corresponding standard candidate instead of creating a duplicate.

## Supported adapters

- `openspec@1`
- `keep-a-changelog@1`
- `keep-a-changelog@2`
- `todo-txt@1`

## Methodology markers

- `openspec`: any present configured or standard OpenSpec source.
- `todo-txt`: any present configured or standard todo.txt source.
- `keep-a-changelog`: any present configured or standard changelog source.
- `agent-guidance`: `AGENTS.md` or `.local/AGENTS.md`.
- `agent-skills`: `skills`, `.agents/skills` or `.local/skills`.

## Private boundary facts

Inspection reports whether the project is a Git repository, whether `.local` exists, whether Git ignores it, whether the index or configured default remote branch tracks it, and the resolved remote reference when available. The stable status is one of `not-git`, `absent`, `protected`, `unprotected`, `index-tracked` or `remote-tracked`.

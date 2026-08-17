---
title: Adoption
description: Add Arcantry guidance, validation and skills without replacing your build system.
---

Arcantry sits above the project's native build system. It does not replace Cargo, CMake, pnpm, Gradle or another domain tool.

## Minimum adoption

1. Run `arcantry repo init --docs <shared|local|none>` from the Git repository you want to adopt.
2. Review the proposed private guidance, durable guidance and OpenSpec integration.
3. Run `arcantry repo doctor` to see unresolved adoption work.
4. Run `arcantry repo validate` to verify the managed contract without changing files.
5. Put the same validation behind the repository's normal contributor and CI commands.

```text
arcantry repo init --docs none
arcantry repo doctor
arcantry repo validate
```

Initialization preserves existing, unowned files. If a repository already has agent instructions or durable documentation, Arcantry reports the integration work instead of replacing that content.

The required `--docs` choice prevents Arcantry from guessing whether `.docs/` is shared, local or unused. Add `--agent codex`, `--agent claude` or `--agent cursor` for each entrypoint that Arcantry should manage.

## Configure operational sources

Repository choices live in private `.local/arcantry.json` with `schemaVersion: 1`:

| Field | Meaning |
| --- | --- |
| `agents` | The selected Codex, Claude or Cursor entrypoints. |
| `operationalSource` | The name of the single source that drives current work. |
| `sources` | Ordered sources in `readonly`, `readwrite` or `operational` mode. |
| `docs` | One explicit `shared`, `local` or `none` choice. |

Initialize a repository with additional ordered sources by repeating `--source`:

```text
arcantry repo init --docs shared --source local=operational --source tracker=readonly
```

`readwrite` records a capability, not permission for the next write. Every external mutation still requires current user authorization for its exact target and action.

## Keep each kind of information in its layer

| Layer | Responsibility | Shared? |
| --- | --- | --- |
| `.local/` | Private instructions, access notes and machine-local execution state. | No. Arcantry keeps it in `.git/info/exclude`. |
| `.docs/` | Durable non-specification guidance, meetings, onboarding notes and reusable project templates. | When the team wants to share it. |
| OpenSpec | Accepted change intent, observable requirements, design decisions, tasks and release metadata. | Yes. |
| Git | Versioned implementation history. | Yes. |

`.docs/` explains the project as it exists. OpenSpec governs a proposed or delivered change. Do not move credentials, logs, local URLs or workstation notes into either shared surface.

## Keep native commands behind the repository surface

The implementation below the repository's `setup`, `check`, `build` and `ci` commands remains project-specific. A TypeScript package may run type checking and Vitest; a Rust project may run formatting, Clippy and tests. Arcantry adds its validation to that existing surface instead of replacing it.

Arcantry follows this rule itself: local development and CI invoke the same repository and skill validation contracts available to adopters.

## Choose skills individually or as a collection

Use `arcantry skills list` and `arcantry skills inspect <name>` to find a focused capability. Link only the skill you need with `arcantry skills link <name>`.

Install the Arcantry Codex plugin when you want the complete versioned catalog. Both paths use the same canonical skill packages and metadata.

## Continue with OpenSpec

Every completed product or engineering change requires an OpenSpec change. Intent may be recorded before implementation or recovered postfactum, but archive and version assignment are the delivery boundary. Each delivered change carries its own `release.md` outcome and SemVer impact.

## Do not migrate commit history into release history

Existing tags and changelogs can remain historical records. Start using OpenSpec-derived releases from the first Arcantry-managed version. Do not manufacture OpenSpec changes from old commits unless the original intent can be recovered reliably.

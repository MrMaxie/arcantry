# Arcantry

Repository foundations for spec-driven delivery.

Arcantry keeps the repository lifecycle small and explicit:

- **OpenSpec** owns change intent and release-facing history.
- **mise** pins shared tools.
- **just** is the command surface for humans and CI.
- **Astro + Starlight** publishes the documentation.

Commits describe implementation history. They are not changelog entries.

## Start

```text
mise install
just setup
just check
```

Documentation lives in `src/content/docs/`. The release model is documented in `openspec/specs/repository-lifecycle/spec.md`.

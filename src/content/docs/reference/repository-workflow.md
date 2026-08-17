---
title: Repository workflow
description: Route private context, durable guidance and accepted change intent to the right place.
---

An Arcantry repository separates working context from durable guidance and change governance. The separation keeps private material out of shared artifacts without forcing every project to use the same native build system.

## Read order

1. Follow the explicit instruction for the current task.
2. Read repository and private agent instructions that apply to the target files.
3. Read relevant durable guidance under `.docs/`.
4. Read the active or accepted OpenSpec material for the change.
5. Confirm behavior in the codebase and current diff.

This order lets task-specific authority override defaults while keeping durable constraints available to the next contributor.

## Information ownership

### `.local/`

Use `.local/` for machine-local agent instructions, access notes, reproduction material, logs and other private execution context. Arcantry keeps the directory in `.git/info/exclude`; it is not a public documentation source or package input.

### `.docs/`

Use `.docs/` for durable, non-specification guidance that explains how the project operates: recurring practices, meeting decisions, onboarding notes and project-specific templates. A team may commit this directory when its contents should travel with the repository. Product and engineering specifications belong only in OpenSpec.

### OpenSpec

Use OpenSpec for every coherent product or engineering change and its delivery record. Proposal, observable requirements, design decisions, implementation tasks and `release.md` stay together. The record may precede implementation or be recovered postfactum, but archive and version assignment are required before the repository state is complete. Release manifests group archived changes without copying their prose.

## Adoption and updates

`arcantry repo init` adds only missing managed foundations. `arcantry repo update` advances managed metadata and generated artifacts while preserving user-editable configuration and unowned files.

If an existing file occupies a managed path without Arcantry ownership metadata, the CLI reports the conflict. It does not claim or replace that file automatically.

Use `arcantry repo doctor` for repair guidance and `arcantry repo validate` for a deterministic read-only gate.

## Removal

`arcantry repo remove` acts only on verified managed artifacts. A familiar path name is not proof of ownership, and user-authored files inside a durable directory remain outside the removal set unless Arcantry created and tracks them explicitly.

## External sources

Issues, pull requests, design tools and project trackers may supply task context. Their write permissions remain independent from the repository workflow. A configured source or installed connector does not authorize Arcantry or a skill to publish, reply or update it.

## Arcantry uses this workflow

The Arcantry repository is a consumer of its own contract. A clean CI checkout initializes ephemeral private adoption state through the public CLI before running the same read-only repository validation available to other repositories. It also validates skill packages, catalog projections and release state through public commands and schemas. Every completed Arcantry change is archived, versioned and represented in the generated changelog before final CI can pass.

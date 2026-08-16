# Purpose

Define the repository-level contract that keeps intent, implementation and release history distinct while making delivery reproducible.

# Requirements

## OpenSpec is the source of release history

A public changelog entry MUST originate from an archived OpenSpec change and its `release.md` artifact. Git commit messages MUST NOT be used as changelog input.

## Archive is the delivery boundary

A change MUST be archived before it can be referenced by a release manifest.

## Release manifests only group changes

A release manifest MUST contain a valid SemVer version, release date and unique archived change IDs. A change MUST NOT be assigned to more than one release manifest. Release manifests MUST NOT duplicate release descriptions.

## SemVer impact belongs to the change

Each `release.md` MUST declare `none`, `patch`, `minor` or `major`. Release planning MUST select the highest impact among unassigned archived changes. Cutting a release MUST create the next manifest from that plan rather than requiring the manifest to be authored by hand.

## Generated changelog is reproducible

`CHANGELOG.md` MUST be a deterministic projection of release manifests and archived `release.md` artifacts. Repository checks MUST fail when the committed changelog differs from the generated projection.

## Release state is validated as a whole

Repository checks MUST reject malformed SemVer versions, duplicate manifest assignments, unknown change IDs and duplicate archived change IDs.

## Repository commands are stable

Contributor-facing and CI commands MUST be exposed through `just`. Shared tool versions MUST be declared through `mise`.

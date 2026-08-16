# Purpose

Define the repository-level contract that keeps intent, implementation and release history distinct while making delivery reproducible.

# Requirements

## OpenSpec is the source of release history

A public changelog entry MUST originate from an archived OpenSpec change and its `release.md` artifact. Git commit messages MUST NOT be used as changelog input.

## Archive is the delivery boundary

A change MUST be archived before it can be referenced by a release manifest.

## Release manifests only group changes

A release manifest MUST contain the version and archived change IDs. It MUST NOT duplicate release descriptions.

## SemVer impact belongs to the change

Each `release.md` MUST declare `none`, `patch`, `minor` or `major`. Release planning MUST select the highest impact among unassigned archived changes.

## Repository commands are stable

Contributor-facing and CI commands MUST be exposed through `just`. Shared tool versions MUST be declared through `mise`.

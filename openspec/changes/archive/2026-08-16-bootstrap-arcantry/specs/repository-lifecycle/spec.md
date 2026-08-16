# ADDED Requirements

## OpenSpec is the source of release history

A public changelog entry MUST originate from an archived OpenSpec change and its `release.md` artifact. Git commit messages MUST NOT be used as changelog input.

## Archive is the delivery boundary

A change MUST be archived before it can be referenced by a release manifest.

## Release manifests only group changes

A release manifest MUST contain the version and archived change IDs and MUST NOT duplicate release descriptions.

## Stable repository command surface

Contributor-facing and CI commands MUST be exposed through `just`; shared tool versions MUST be declared through `mise`.

# Why

Three legacy specifications predate the canonical OpenSpec requirement shape. They omit executable scenarios, so strict validation fails and a later change cannot be archived without weakening the safety checks.

# What changes

- Normalize the legacy documentation and release specifications to canonical requirement blocks.
- Add one observable scenario to every legacy requirement while preserving its existing contract.
- Make strict validation of all specifications and active changes part of the repository CI gate.
- Require the Pages workflow to pass the same repository verification before deployment.

# Out of scope

- Product behavior, public API, and release semantics do not change.
- This change does not publish a package, create a tag, or create a GitHub release.

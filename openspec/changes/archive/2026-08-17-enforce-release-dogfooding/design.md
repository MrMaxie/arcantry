# Decisions

## Internal releases are completion boundaries

A branch may contain implementation commits while work is in progress, but the final repository state MUST include archived OpenSpec changes, a new SemVer manifest, aligned distribution versions and the generated changelog. Publication remains a separate optional action.

## Git proves sealing, not meaning

Strict release validation will compare repository HEAD with the commit that introduced the newest release manifest. A later commit, including an edit to that same manifest, means the repository has unsealed work and validation fails. Git metadata MUST NOT supply release descriptions, categories, impacts or components; those remain owned by OpenSpec release artifacts.

## Postfactum intent remains first-class intent

When implementation precedes its OpenSpec record, completion requires a normal change with proposal, specification delta, tasks and release outcome. Postfactum recovery does not use a synthetic commit log entry or weaken validation.

## Missed outcomes remain separate changes

The repository validation hardening in `f4fcdfb` and complete review coverage in `3ced730` will be recovered as separate OpenSpec changes. Version 0.3.1 will group those two behavior-level outcomes with this lifecycle enforcement change so the changelog describes product behavior instead of commits.

## The existing release path remains authoritative

`release plan`, `release cut`, `changelog render`, `release check` and `just ci` will continue to share the release module. The new seal invariant will be part of that path rather than a parallel script or manual convention.

# Trade-offs

The strict seal makes any commit after a release manifest temporarily fail full CI until the next internal release is cut. This is intentional: targeted checks remain available during implementation, while the final repository state cannot silently bypass OpenSpec, versioning or the changelog.

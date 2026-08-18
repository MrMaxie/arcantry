## MODIFIED Requirements

### Requirement: Release state is validated as a whole

Repository validation MUST fail when a manifest references a missing or active change, reuses a change id, breaks descending SemVer order, contains an invalid component id, leaves completed changes unassigned, has distribution version drift, has generated changelog drift or leaves repository changes after the newest release seal. Validation of contributed release artifact content MUST use input-bounded parsing that does not exhibit polynomial regular-expression behavior.

#### Scenario: Release metadata drifts

- **WHEN** a manifest, distribution version or generated changelog violates a release invariant
- **THEN** repository validation fails with the violated invariant

#### Scenario: Repository work follows the newest release seal

- **WHEN** strict release validation finds a commit after the commit introducing the newest release manifest
- **THEN** validation fails and requires the work to be represented by archived OpenSpec intent and a newer internal release

#### Scenario: A malformed release title contains excessive whitespace

- **WHEN** validation reads a release artifact whose title line contains no title after a long whitespace sequence
- **THEN** validation rejects the title in work proportional to the artifact size

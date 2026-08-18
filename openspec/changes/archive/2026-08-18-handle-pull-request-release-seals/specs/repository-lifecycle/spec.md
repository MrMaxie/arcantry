## MODIFIED Requirements

### Requirement: Git history is coverage evidence only

Release validation MAY use Git history to prove a configured release seal when Git is available and sealing is enabled. Projects without Git MUST remain able to inspect, plan, apply and validate non-seal source contracts. Pull request CI MAY validate an explicitly supplied submitted head commit when the checked-out state is a synthetic merge commit, but MUST require that commit to introduce the newest release manifest and be a direct parent of the checked-out commit. External publication MUST remain bound to the actual checked-out commit.

#### Scenario: A non-Git project manages todo and OpenSpec

- **WHEN** repository validation runs without a Git worktree
- **THEN** source validation succeeds or fails solely from configured source contracts
- **AND** no Git seal requirement is inferred

#### Scenario: The release seal is inspected

- **WHEN** configured validation reads Git history for the newest release manifest
- **THEN** it reports only whether later repository changes exist
- **AND** resolves all release meaning from OpenSpec artifacts

#### Scenario: Pull request CI checks a synthetic merge commit

- **WHEN** CI supplies the exact submitted head commit while testing a synthetic merge result
- **THEN** release validation accepts the seal only when that submitted commit introduced the newest release manifest
- **AND** the submitted commit is a direct parent of the checked-out merge commit
- **AND** publication validation continues to require the actual checked-out commit

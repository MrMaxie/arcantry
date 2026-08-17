# Decisions

## CI adopts before it validates

The Arcantry contributor command will run the built `repo init --docs none` path before the existing public validation commands. The default agent and operational source match Arcantry's own private configuration and keep the command deterministic.

## Private state remains ephemeral

Initialization writes `.local` and `.git/info/exclude` only inside the current checkout. CI does not commit, upload or document machine-local values.

## Validation remains read-only

The fix changes orchestration, not validator semantics. A missing configuration still fails `repo validate`; callers that need a clean checkout must initialize it explicitly first.

## The project command remains the workflow authority

`just ci` owns the bootstrap and validation sequence so local and GitHub checks exercise the same contract. Workflow YAML continues to call one project command.

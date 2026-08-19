---
name: protect-local-boundary
description: Protect a repository's `.local/` privacy boundary whenever work may create, read, write, copy, move, derive, watch, build, test, package, document, or promote `.local` content. Use for every planned or actual operation involving `.local`; do not use for a conceptual mention with no operation.
---

# Protect Local Boundary

Keep private operational state usable without letting it become a dependency of
shared project behavior. Treat `.local/` as private unless the repository's
configured default remote branch already establishes a different policy.

## Resolve the boundary first

Before creating or using `.local/`:

1. Read the applicable repository and private instructions without bulk-reading
   `.local/`.
2. When Git and a configured default remote reference are available, inspect
   whether that reference tracks `.local/`.
3. If the reference tracks `.local/`, preserve that established repository
   policy and report its conflict with the private-local convention. Do not
   silently untrack or privatize it.
4. Otherwise treat `.local/` as private, including when it does not exist yet.
   Ensure `.local/` is covered by `.git/info/exclude` before creating the first
   item and verify the rule with `git check-ignore -v .local`. Do not add it to
   `.gitignore` unless the user explicitly requests a shared policy.
5. If only the current index tracks `.local/`, preserve the working files and
   require a separate, explicitly authorized operation before changing the
   index.

In a non-Git project, treat `.local/` as private unless an explicit project
contract says otherwise. The absence of the directory is not evidence that it
may become shared.

## Classify the operation

Choose exactly one boundary case before acting:

- **Private to private:** Work may remain under `.local/` when repository tools
  cannot observe or depend on it. Inspect only the paths required by the task.
- **Shared to private:** A local cache, note, fixture, or derived working copy may
  depend on shared project state. The shared project must not gain a reverse
  dependency on that private result.
- **Private to shared:** Availability or permission to inspect is not permission
  to publish. Copy or transform only the exact item covered by an explicit,
  reviewed promotion decision. Place the result in a project-owned location and
  make it self-contained.
- **Temporary work:** When a dev server, watcher, build, formatter, test runner,
  indexer, or other repository tool could observe `.local/`, create previews,
  comparisons, partial writes, and other temporary artifacts outside the
  repository. Use `.local/` only when that work is isolated from shared flows.

If the operation changes cases while work is in progress, stop at the new
boundary and obtain any authorization that case requires.

## Keep the worlds independent

For private `.local/` state:

- Do not add `.local/` paths, imports, discovery rules, watcher ignores, build or
  formatter exclusions, test setup, runtime behavior, or package inputs to
  tracked project files as a workaround.
- Do not fix a leak by teaching shared tooling about the private directory. Move
  temporary work outside the repository or promote the exact authorized output
  into a project-owned location.
- Do not make CI, deployment, documentation, pull requests, releases, or product
  behavior depend on private files or private machine values.
- Do not promote a directory, a continuing synchronization relationship, or
  unrelated neighboring content when only one item was approved.
- Remove local paths, private metadata, credentials, and unrelated workstation
  data from promoted or exported results.

Promote a local requirement to shared project configuration only when an
existing project contract or evidence from another contributor, CI, deployment,
or the target runtime requires it. Use the project's portable configuration
surface and safe example values; do not preserve a dependency on `.local/`.

## Preserve authorization

- A named `.local/` item authorizes only the requested inspection or operation.
- Open credential-bearing files only when the authorized task requires them, and
  never print or copy secret values.
- Treat copying, moving, deleting, untracking, committing, pushing, and
  publishing as separate authorization boundaries.
- Preserve unrelated private files and existing user changes.

## Verify before handoff

1. Confirm the resolved remote policy and, for private `.local/`, the local Git
   exclusion when Git is present.
2. Review the full tracked diff, untracked project files, generated outputs,
   documentation, and package contents for private paths, data, dependencies,
   discovery, or workaround behavior.
3. Confirm each promoted output is self-contained in its project-owned location
   and limited to the explicitly approved item.
4. Remove only temporary artifacts created by the current task. Leave all other
   private and user-owned content unchanged.
5. Report which boundary case was used, what was promoted if anything, and any
   verification that could not be completed.

Use `audience-scope-discipline` for broader audience and scope decisions,
`adopt-arcantry` for Arcantry adoption, and `maintain-agent-guidance` for edits to
`AGENTS.md` or `.local/AGENTS.md`. This skill owns the operation at the private
boundary; companion skills must not duplicate its procedure.

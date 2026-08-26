# Why

The current public-or-internal release classification cannot distinguish customer outcomes, user-felt fixes, significant technical work and code-only maintenance. It also assumes that each included OpenSpec change becomes its own changelog entry, while real release communication may consolidate several accepted outcomes or omit small maintenance work. Manually maintained histories can therefore remain useful while differing from managed categories, baselines, source markers and consolidation rules.

# What changes

- Introduce an opt-in release adapter that classifies work by audience, observable impact and explicit changelog inclusion independently from SemVer impact and manifest assignment.
- Allow several accepted changes to form one audience-facing story with complete source traceability.
- Allow release-bearing maintenance work to remain assigned and versioned while intentionally producing no changelog entry.
- Inspect detached or manual release history and produce an explicit migration plan that reveals semantic differences before validation rejects it.
- Preserve existing release adapters and history until migration is explicitly accepted.

# Out of scope

- Deriving release meaning from commits or diffs.
- Silently rewriting existing changelog history.
- Treating every technical change as audience-facing release content.
- Changing the current Arcantry product version or cutting a release.

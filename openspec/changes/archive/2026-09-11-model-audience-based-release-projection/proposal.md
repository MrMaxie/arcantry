# Why

The current public-or-internal release classification cannot distinguish customer outcomes, user-felt fixes, significant technical work and code-only maintenance. It also cannot intentionally consolidate outcomes from several OpenSpec changes or omit small maintenance work. Manually maintained histories can therefore remain useful while differing from managed categories, baselines, source markers and consolidation rules.

# What changes

- Complete `openspec-release@1` as the single final release adapter and remove prepublication adapter-version proliferation.
- Classify work by audience, observable impact and explicit changelog inclusion independently from SemVer impact and manifest assignment.
- Allow several accepted changes to form one audience-facing story with complete source traceability.
- Allow release-bearing maintenance work to remain assigned and versioned while intentionally producing no changelog entry.
- Keep historical release manifests parseable without exposing a public adapter migration workflow.

# Out of scope

- Deriving release meaning from commits or diffs.
- Rewriting existing changelog history or release manifests.
- Treating every technical change as audience-facing release content.
- Changing the current Arcantry product version or cutting a release.
- Adding another release adapter version or a migration command for unpublished adapter contracts.

# Backlog decisions

Every original shared queue entry is retained below. Covered means assigned, not implemented. Duplicate ideas share an implementation task.

## Q01

2026-08-19 Evaluate Open Policy Agent and its Rego policy language (https://www.openpolicyagent.org/) as technologies worth promoting alongside OpenSpec, todo.txt, and Varlock, and decide whether Arcantry should support an OPA/Rego-based policy integration path or a custom policy format. @project-expansion @deep-thinking

Rejected for MVP: No demonstrated need for a policy engine.

## Q02

2026-08-19 Evaluate how to structure todo.txt usage so projects adopting Arcantry can use the format more easily and effectively. @project-expansion @deep-thinking

Covered by stage 3; verify against tasks.md before closing.

## Q03

2026-08-19 Evaluate whether Arcantry should replace todo.txt with a similar custom or existing format, or retain and improve todo.txt. @project-expansion @deep-thinking

Rejected for MVP: Keep the standard todo.txt format.

## Q04

2026-08-20 Evaluate Homebrew tap and Scoop bucket distribution for Arcantry, including repository ownership, release automation credentials, checksum verification, and retry-safe publication. @project-expansion @deep-thinking

Rejected for MVP: New distribution channels are excluded.

## Q05

2026-08-20 Evaluate how agents should handle statements, screenshots, and conversation excerpts when their source or intended audience is unknown, including whether to treat them as private context by default, ask the user, or inspect available browser and page context before deciding what may be referenced, summarized, omitted, or published. Investigate this primarily as a possible Audience skill direction or, if the responsibility should remain separate, as a new related skill connected through mutual references or routing triggers. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q06

2026-08-20 Evaluate a reusable agent guidance pattern for human-readable resource references: preserve machine-precise identifiers only when they help the audience locate, disambiguate, reconcile, or troubleshoot an item, and pair them with a title, short content snippet, or contextual description recognizable in the destination GUI. The motivating failure was a handoff that used identifiers with no visible GUI representation, leaving the user unable to tell what they referred to or use the answer; treat such identifiers as diagnostic data rather than primary communication. Link stable identifiers when useful, let nested references rely on a parent-resource link, and omit misleading or non-navigable links. Use GitHub issues, pull requests, and review comments as a portable reference case, then decide whether the behavior belongs in Audience guidance, a general communication or reference skill, or routing between them. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q07

2026-08-20 Evaluate an optional, concise .local role-and-relevance profile for Arcantry projects that lets a user declare primary areas, default discovery interests, exclusions, client-system action boundaries, and temporary cross-role exceptions. Define a lightweight field format with simple when/then exception rules and an agent-led questionnaire for creating and maintaining it. Treat the profile only as a non-authoritative relevance filter when a request is not explicitly scoped: ask about plausible exceptions, preserve confirmed exclusions until evidence changes, and let human instructions and verified current evidence override it. @project-expansion @deep-thinking

Covered by stage 2; verify against tasks.md before closing.

## Q08

2026-08-21 Evaluate a reusable skill for approval-gated content curation across audience boundaries. The workflow should record item-level include, exclude, adapt, and omit decisions before implementation, then verify the result against the approved set. Reuse audience-scope-discipline for general audience and publication rules. @project-expansion @deep-thinking

Rejected for MVP: Use the existing audience skill instead of another curation skill.

## Q09

2026-08-21 Evaluate whether an invoke-only skill dedicated to adding notes to shared todo.txt and private .local/todo.txt queues would reduce capture overhead without duplicating capture-project-work. Keep source selection, audience boundaries, exact preview approval, official todo.txt compatibility, and bounded verification aligned with the existing workflow. @project-expansion @deep-thinking

Rejected for MVP: Improve capture-project-work instead of adding a duplicate.

## Q10

2026-08-21 Evaluate whether Arcantry should provide an MCP server, agent-oriented CLI helpers, or both for repository-independent discovery and validation. Compare portability, maintenance, host integration, and deterministic scripted checks that replace repeated manual gates without weakening authorization, privacy, or apply boundaries. @project-expansion @deep-thinking

Covered by stage 2; verify against tasks.md before closing.

## Q11

2026-08-21 Evaluate whether Arcantry should provide host-specific commands for Codex and Claude in addition to portable skills, and identify which workflows benefit from explicit commands without duplicating the canonical skill implementations. @project-expansion @deep-thinking

Rejected for MVP: Portable CLI and MCP cover both hosts.

## Q12

2026-08-21 Evaluate how the separately proposed role-and-relevance profile should participate in deterministic Arcantry context discovery after that profile's format and authority are decided. @project-expansion @deep-thinking

Covered by stage 2; verify against tasks.md before closing.

## Q13

2026-08-22 Evaluate how to make capture-project-work easier to discover for developers who think in todo or task concepts rather than "capture", including whether its description should mention todo more prominently, host metadata should expose a todo-related alias if supported, or the skill should use a todo-oriented name. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q14

2026-08-22 Evaluate how Arcantry could represent skills, work methodologies, and source formats as independently versioned, updatable atoms, including how those atoms relate to OpenSpec, todo.txt, and .local sources and how documentation, generated skill pages, catalogs, and plugin manifests distinguish their versions from the Arcantry library's 1.0.0 version. Evaluate requiring each atom to carry its own version and support optional inter-atom dependencies with explicit version ranges when their responsibilities affect one another, while allowing loosely coupled atoms with independent obligations to declare no dependency. @project-expansion @deep-thinking

Rejected for MVP: Independent atom version resolution adds infrastructure without a current user need.

## Q15

2026-08-22 Evaluate homepage entry points such as "Copy prompt" and "Configure your setup" so they support both assessing whether Arcantry fits and preparing an adoption request, especially near installation guidance. A visitor seeking a no-change recommendation or guided setup currently encounters language that can imply commitment before evaluation, even though the configurator can support either outcome. Explore discoverability, labels, introductory copy, routing, and generated guidance without reducing the configurator to only evaluation or only adoption. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q16

2026-08-22 Evaluate whether evidence-based positive reinforcement for an agent that selected skills well can improve future skill routing, including whether an inverse counterpart to agent-self-improve is feasible, useful, and implementable without relying on unsupported praise. @project-expansion @deep-thinking

Rejected for MVP: No measurable product benefit from praise-driven routing.

## Q17

2026-08-26 Explore two explicit ownership modes for repository-level scripts, skills, and related assets: source-managed assets that remain updateable and a permanently detached repository implementation that gives up source-product updates, compatibility promises, and branding. A test-project case study showed that treating both as one model hides materially different support, security, migration, and maintenance obligations. @project-expansion @deep-thinking

Covered by stage 3; verify against tasks.md before closing.

## Q18

2026-08-31 Evaluate creating a schema-driven shape-openspec-change skill for creating, refining, and explicitly accepting OpenSpec changes before implementation, including product-contract checks that distinguish user outcomes from implementation mechanisms, preserve unresolved decisions as approval gates, and route material GUI work through design-coherent-gui. @project-expansion @deep-thinking

Covered by stage 2; verify against tasks.md before closing.

## Q19

2026-08-31 Evaluate a privacy-preserving serialized-plan output contract that supports an explicit output file, secure permissions, and clear disclosure when a plan contains private content instead of requiring sensitive plans to pass through standard output. @project-expansion @deep-thinking

Covered by stage 3; verify against tasks.md before closing.

## Q20

2026-08-31 Evaluate project-root-scoped locking and interruption-safe filesystem recovery for every mutating apply path, including durability boundaries, orphaned .arcantry temporary and backup detection, manual recovery guidance, and executable crash scenarios without automatic deletion. @project-expansion @deep-thinking

Covered by stage 3; verify against tasks.md before closing.

## Q21

2026-08-31 Evaluate one stable CLI interaction contract across command families for flag names and ordering, scope vocabulary, preview and apply behavior, JSON schema versioning, stdout and stderr roles, exit codes, help structure, stability tiers, and controlled future extensions. @project-expansion @deep-thinking

Covered by stage 3; verify against tasks.md before closing.

## Q22

2026-08-31 Design a controlled dependency-update model that separates read-only discovery and advisory review from lockfile regeneration, full verification, and explicit acceptance, while enforcing exact versions, pinned action SHAs, OCI digests, trusted registries and Git sources, allowed licenses, and rejection of wildcard or yanked dependencies. @project-expansion @deep-thinking

Rejected for MVP: Use existing package managers and local checks; no update-policy subsystem.

## Q23

2026-08-31 Evaluate restructuring CI into independently reported Rust, tooling, documentation, coverage, package, and system-test jobs, removing the duplicate full CI run from documentation deployment, cancelling obsolete branch runs, publishing coverage summaries, and running bounded Windows and macOS soak checks before release tags. @project-expansion @deep-thinking

Rejected for MVP: Superseded by local validation and Pages-only delivery.

## Q24

2026-08-31 Add a non-publishing release rehearsal workflow that exercises the full native target matrix, installers, npm packages, package smoke checks, and artifact assembly without creating a tag, release, or publication. @project-expansion @deep-thinking

Rejected for MVP: Release rehearsal is outside the non-publishing MVP.

## Q25

2026-08-31 Evaluate release artifact provenance and reproducibility through SBOMs, attestations, signed or independently verifiable checksum manifests, and comparison of independently built artifacts from the same commit. @project-expansion @deep-thinking

Rejected for MVP: Artifact attestations belong to a separately authorized release effort.

## Q26

2026-08-31 Establish browser-level documentation quality gates covering configurator state restoration, copy, reset, theme, navigation, reduced motion, keyboard use, automated accessibility, representative visual regression, internal links, generated HTML, and a bounded frontend performance budget. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q27

2026-08-31 Extend adversarial parser and plan verification beyond filesystem transactions with property tests and bounded fuzzing for TOML, YAML, release manifests, todo.txt, source graphs, and untrusted serialized plans. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q28

2026-08-31 Establish explicit time and memory budgets for inspecting, hashing, planning, and applying changes in large repositories, with representative fixtures and regression thresholds before optimizing. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q29

2026-08-31 Build a reproducible public evidence suite for Arcantry using synthetic projects and selected real repositories, recording the input state, expected discovery, plan, diff, idempotent rerun, intentional drift rejection, pinned Arcantry version, one-command reproduction, and native operating-system results where relevant. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q30

2026-08-31 Evaluate replacing the current technical skill families with an audience-facing catalog model that exposes Arcantry Core, boundary and intent protection, repeatable verification, purposeful results, and explicit primary, helper, advanced, and flagship roles without presenting every skill as an equal entry point. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q31

2026-08-31 Define evidence-based admission and retention criteria for the public skill portfolio so every skill comes from a real repeated workflow, has a reproducible example, and earns its place without artificial GUI, TUI, mobile, or web symmetry; use design-coherent-gui as a flagship canary and decide whether design-terminal-ux remains public, moves outside the main palette, or is retired. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q32

2026-08-31 Refactor audience-scope-discipline into a concise decision core plus focused references for copy, errors, diagnostics, and detailed scenarios, preserving its current behavior and leaving .local operations owned by protect-local-boundary. @fix

Covered by stage 4; verify against tasks.md before closing.

## Q33

2026-08-31 Evaluate explicit dependencies, readiness states, and a recommended sequence for active OpenSpec changes so the repository distinguishes accepted backlog from work in progress and does not present every active directory as simultaneous delivery. @project-expansion @deep-thinking

Covered by stage 2; verify against tasks.md before closing.

## Q34

2026-08-31 Reconcile current todo.txt entries with active OpenSpec changes by removing fully promoted duplicates and narrowing partially promoted entries only after their coverage and provenance are verified, without changing unrelated queue content. @fix

Covered by stage 1; verify against tasks.md before closing.

## Q35

2026-08-31 Add contributor intake guidance and structured issue forms that request the operating system, installation route, version, command, exit code, and smallest safely redacted reproduction while preserving private project content. @project-expansion @deep-thinking

Rejected for MVP: A concise reproduction guide is sufficient; no new intake subsystem.

## Q36

2026-08-31 Evaluate a privacy-preserving diagnostics command that creates a local allowlisted and redacted support bundle without including source contents, credentials, workstation identifiers, or automatic upload behavior. @project-expansion @deep-thinking

Covered by stage 3; verify against tasks.md before closing.

## Q37

2026-08-31 Add generated shell completions and manual pages for supported shells and platforms from the canonical Clap command model so help artifacts cannot drift from the executable CLI. @project-expansion @deep-thinking

Rejected for MVP: Clap help and contextual answers are the MVP discovery surface.

## Q38

2026-09-01 Evaluate extending release version-source adapters beyond top-level JSON and Cargo workspaces so projects using text, XML, or ecosystem-specific version files can adopt Arcantry release manifests without maintaining a parallel versioning workflow. @project-expansion @deep-thinking

Covered by stage 3; verify against tasks.md before closing.

## Q39

2026-09-01 Explore a clear, polished website visualization for business leaders, people teams, newcomers, and visual learners that contrasts fragmented information sources and commit-heavy changelogs with a layered tree of public and private todo queues, shows selected ideas flowing into OpenSpec while OpenSpec can also start independently, and follows accepted changes into code, updated specifications, and meaningful changelog entries. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q40

2026-09-02 Evaluate configurable release-version strategies beyond SemVer, including monotonically increasing integers and calendar/date-based identifiers, with strategy-specific validation, ordering, baselines, next-version calculation, manifest and changelog naming, and an explicit approval gate for reviewing changelog content before a version is recognized as released. @project-expansion @deep-thinking

Covered by stage 3; verify against tasks.md before closing.

## Q41

2026-09-02 Evaluate how Arcantry should discover and preserve each project's boundaries between versioned and non-versioned work, including which merges require a version or release record and which may land without one, without imposing a universal merge, deployment, or publication model. @project-expansion @deep-thinking

Covered by stage 3; verify against tasks.md before closing.

## Q42

2026-09-06 Evaluate revising option discovery in propose-best-practices so it covers actual choices rather than satisfies a fixed presentation template. Replace mandatory option counts and predefined solution categories with coverage of all materially distinct approaches identified for the problem and requested scope, without an arbitrary minimum beyond one or an arbitrary maximum. Allow a single approach when it is the only known applicable method, explicitly distinguishing the limits of the agent's knowledge and research from proof that no alternative exists; allow five or more approaches when they represent meaningful differences. Do not invent alternatives to fill a quota, collapse distinct strategies to fit a template, or omit an applicable approach merely because the agent considers it inferior. Include weak, inconvenient, costly, or generally discouraged approaches when they help explain the available choices or could suit different operator priorities; describe their concrete disadvantages and the circumstances in which someone might still choose them. Distinguish an applicable but undesirable option from one that cannot satisfy a verified requirement, and explain exclusions without presenting impossible choices as viable. Evaluate contrasting cases with one known applicable method, many credible methods, an inferior but contextually useful alternative, and incomplete research that prevents a claim of comprehensive coverage. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q43

2026-09-06 Evaluate replacing abstract scoring in propose-best-practices with comparisons that let an operator understand mechanisms, consequences, and tradeoffs. Replace numeric ratings, weighted scores, and unexplained qualitative labels with plain descriptions. When comparison is useful, use rows for options and columns for topics relevant to the actual task, such as how the approach works, what the operator must implement or configure, security implications, side effects, integration constraints, maintenance responsibilities, extensibility, migration effort, and runtime or dependency cost. Select columns from the user's concerns and verified context rather than requiring the same checklist every time. Explain what a security difference exposes, prevents, or leaves to the implementer instead of assigning a score; explain where additional work or failure modes arise instead of merely calling an option expensive or risky. Retain meaningful measurements with units, sources, and conditions when available, while clearly separating measured facts, documented behavior, estimates, and judgment. Make the recommendation traceable to the operator's priorities, identify the tradeoffs it accepts and the conditions that would change it, and preserve enough information for the operator to reasonably prefer another option. Evaluate whether the explanation supports an informed decision without requiring the reader to interpret an invented scale, and disclose evidence gaps that prevent a confident comparison or ranking. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q44

2026-09-06 Evaluate whether propose-best-practices should support distinct intents within one skill or separate research and recommendation from operator-led option comparison. Recognize the difference between asking how practitioners commonly solve a problem, requesting an explanation of established good practice, asking the agent to select a suitable strategy, requesting an explicit comparison for a human decision, and authorizing implementation. Infer the intended response from the instruction and available context; do not turn every request into a decision questionnaire or require the user to select a skill mode. Ask a focused question only when unresolved intent would materially change the result. For research or delegated recommendation, investigate relevant established approaches and explain the best-supported fit without automatically demanding that the operator choose among alternatives; for an explicit comparison, present the meaningful choices and their consequences; neither mode alone authorizes implementation. Ground strategy selection in current project evidence and relevant external sources, including official documentation, maintained library guidance, substantive practitioner articles, and documented implementation experience, while distinguishing common adoption, author opinion, vendor claims, and suitability for this task. Verify time-sensitive claims and disclose what was not established. Evaluate requests seeking understanding without a decision, requests delegating strategy selection, and requests reserving the decision for the operator. Decide whether separate skills would clarify these responsibilities enough to justify their routing and maintenance cost, or whether explicit intent handling within the existing skill is sufficient. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

## Q45

2026-09-06 Evaluate improving strategy selection in propose-best-practices so it accounts for long-term ownership and avoids unjustified custom implementations or proliferation of overlapping solutions. Consider built-in capabilities, established local patterns, focused libraries, deliberate combinations of libraries, integrated solutions, thin adapters, and custom implementations where relevant, without requiring every category to appear. Do not prefer custom code merely because its initial implementation looks short, has no new dependency, or is easy for the agent to generate; account for ongoing ownership, testing, compatibility, defect handling, documentation, future extension, and migration. Equally, do not assume a popular dependency or comprehensive framework is automatically justified. When the goal is to unify fragmented mechanisms, first identify what must become consistent for callers and maintainers, then assess whether the strategy removes duplication or merely hides several overlapping implementations behind another layer. Distinguish one coherent interface backed by justified complementary components from multiple competing ways to accomplish the same responsibility. Compare lightweight runtime behavior separately from the convenience and consistency of the authoring abstraction, and explain the tradeoff when they conflict. Present alternative strategies as alternatives rather than implementing several simultaneously; justify any proposed combination through distinct responsibilities and a coherent maintenance model. Include a migration direction explaining which existing mechanisms would be retained, replaced, or retired, without expanding a research request into a refactor. Evaluate cases where existing conventions are sufficient, a focused dependency offers the best fit, a deliberate combination is warranted, an integrated solution reduces ownership cost, or a small custom implementation is genuinely justified by verified constraints. Reconcile this work with any existing guidance or backlog item about solution proliferation and unjustified reinvention before introducing overlapping rules. @project-expansion @deep-thinking

Covered by stage 4; verify against tasks.md before closing.

# Existing OpenSpec changes

These changes remain sources of detailed acceptance criteria. Their overlapping work is sequenced by deliver-practical-mvp, not treated as concurrent delivery.

- `add-design-coherent-gui-skill`: Completed in existing source; retain evidence.
- `align-host-plugin-identity`: Covered by MVP; replace conflicting process gates with this approved direction.
- `align-public-trust-surface-with-evidence`: Covered by MVP; replace conflicting process gates with this approved direction.
- `define-managed-and-detached-adoption`: Covered by MVP; replace conflicting process gates with this approved direction.
- `establish-arcantry-dev-public-domain`: Covered by MVP; outstanding remote domain setup remains outside local execution authority.
- `explain-arcantry-fit-and-adoption-tradeoffs`: Covered by MVP; replace conflicting process gates with this approved direction.
- `extend-configurator-adoption-scenarios`: Covered by MVP; replace conflicting process gates with this approved direction.
- `harden-native-cli-verification-gates`: Covered by MVP; replace conflicting process gates with this approved direction.
- `improve-configurator-responsive-reflow`: Covered by MVP; replace conflicting process gates with this approved direction.
- `model-audience-based-release-projection`: Covered by MVP; replace conflicting process gates with this approved direction.
- `preserve-cli-intent-provenance`: Covered by MVP; replace conflicting process gates with this approved direction.
- `preserve-todo-queue-conventions`: Covered by MVP; replace conflicting process gates with this approved direction.
- `recognize-varlock-environment-contracts`: Covered by MVP; replace conflicting process gates with this approved direction.

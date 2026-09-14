# MVP verification

Verified implementation: a996285, following the local delivery, contextual CLI/MCP and project release-conventions commits. All changes descend from 3b84c40; the original 31 local commits were preserved. No push, remote settings, deployment, tag or package publication was performed. Product versions remain 1.0.0.

## Local evidence

- Windows: formatting, generated-file checks, catalog validation, Astro type checks, Clippy and 132 Rust tests passed. The final host chain encountered a transient RustSec GitHub fetch failure; the isolated dependency check then passed advisories, bans, licenses and sources. The remaining documentation build and native Windows npm package smoke passed independently. An earlier complete host chain also passed.
- Linux: existing Testcontainers workflow passed Clippy, 134 tests, CLI build, version and help smoke checks against the pinned Alpine Rust image. No private `.local` tree was sent to the build context.
- Documentation: 36 static pages built and output checks passed. Astro reported one deprecated beforeunload compatibility-property hint; the existing missing 404 content notice does not prevent its fallback page from building.
- Browser: evaluation without CLI/Git, existing private setup, copied feedback, reset, URL restoration and removal of incompatible URL choices were checked. Configurator and the expandable three-command example fit 390px and 1280px viewports without horizontal overflow. Browser clipboard payload could not be read back through the available bridge; only the page's copy-success feedback was verified.
- OpenSpec: schema validation and all 25 strict validation items passed.
- No macOS or additional native platform execution is claimed. Optional coverage was not run.

## Acceptance evidence

| Concern | Evidence |
| --- | --- |
| Empty project, absent optional tools | Native CLI and empty-project guidance scenarios; no files created. |
| Existing project and source formats | Context/next/explain scenario reads project instructions, task order and custom schema template. |
| Conflicting requirements | Dependency cycles fail explicitly; unresolved conversational authority is always unknown. |
| Private sources | Existing source visibility tests and diagnostic sentinel test; Varlock observation succeeds with unreadable-as-text schema bytes. |
| No CLI | Source-based getting-started workflow and configurator request verified in the browser. |
| Version strategies and history | SemVer tests retained; integer/calendar scenario covers baseline, cut, exact old CRLF history, text version update, template rendering and idempotence. |
| Privacy and package restrictions | Non-SemVer Cargo sources and private templates for shared changelogs are rejected. |
| Stale inputs and recovery | Source/config/template hashes, existing rollback property tests, interrupted-commit journal test and exact external authority checks. |
| Detachment | Serialized source-detachment preview refuses drift and preserves source bytes and attribution through apply. |
| MCP | Real stdio initialization and next request while stdin remains open; no project writes. |

## Code and test cost

Physical source lines include tests and exclude generated outputs. The baseline is 3b84c40; the final implementation is a996285. Rust and Astro are reported separately so feature growth is visible.

| Surface | Before | After | Change |
| --- | ---: | ---: | ---: |
| Product Rust crates, including tests | 12715 | 13977 | +1262 |
| Rust xtask, including tests | 4656 | 4012 | -644 |
| Astro components | 4883 | 3870 | -1013 |
| Rust plus Astro | 22254 | 21859 | -395 |
| Canonical SKILL.md text | 1473 | 1226 | -247 |

Rust alone grew by 618 lines. The new guidance, release strategies, templates and recovery behavior outweigh the removed Rust infrastructure. This is not a claim of a smaller Rust-only codebase. The configurator shrank substantially while preserving the useful setup decisions. The custom coverage analyzer and policy were removed, Clap replaced custom help/error formatting, graph validation has one implementation and duplicate test-only SemVer helpers were removed.

The native CLI contract suite took 13.81s before duplicate-dispatch removal and 6.13s afterward on the same Windows host during this task. These are warm development-run observations, not a controlled benchmark. The host recipe no longer reruns the same contract suite separately, and Linux no longer repeats it after workspace tests. Coverage is diagnostic rather than a required extra test run.

## Deliberate limits

The backlog records rejected extensions individually. Detachment transfers configured source ownership, not arbitrary application infrastructure. Recovery refuses mixed states for manual review; it does not promise power-loss durability or automatic deletion of unrecognized staging files. Relevance profiles are filters, not policies. The historical proposals remain available as provenance and are not blanket claims of implemented scope.

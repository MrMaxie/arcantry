# States and Copy

Use this reference to design honest system feedback and user-facing language. Describe what the user can understand or do; keep implementation detail subordinate.

## Contents

- State model
- Loading and long operations
- Completion and attention
- Empty and unavailable states
- Partial and stale results
- Errors and recovery
- Destructive actions
- Writing user-facing text

## State model

For every meaningful state, define:

- what happened in user terms;
- whether the user can continue;
- whether the state requires action;
- the safest useful next action;
- what detail is available on demand;
- how focus and navigation behave.

Do not reuse a generic empty-looking screen for states with different causes. Never show an empty-state message while data is still loading.

## Loading and long operations

Increase information with duration and uncertainty:

| Situation | Feedback |
| --- | --- |
| Imperceptible work | Avoid a distracting indicator |
| Short indeterminate work | Show a spinner or activity mark with the action name |
| Work lasting seconds to minutes | Add the current meaningful stage and elapsed time when useful |
| Measurable work | Show progress derived from a real completed/total measure |
| Complex or diagnostic work | Offer hideable details without making logs the main experience |

Describe the user's task, not the internal worker. Prefer “Validating imported records” over “Worker 3 processing queue”.

Do not fabricate percentages, smooth unknown progress into a fake estimate, or imply an exact completion time without evidence. When only stages are known, show stages. When nothing measurable is known, show activity, elapsed time, and reassurance that the operation is still responsive.

Offer cancel only when cancellation is meaningful and safe. Explain delayed cancellation when the current stage cannot stop immediately. Distinguish cancelled work from failure and success.

## Completion and attention

Match completion feedback to attention:

- If the result is visible on the active screen, update the state without an extra success message.
- If the user may have switched context during a long operation, provide a restrained notification.
- Use sound or a strong alert only when timely user action is required and the environment permits it.
- Keep a durable record when the completion affects later decisions.
- Do not turn routine success into an accumulating stream of confirmations.

Escalate according to required action, urgency, reversibility, and whether the user is still watching. Do not escalate merely because an internal event occurred.

## Empty and unavailable states

Keep these states distinct:

| State | Explain | Useful action |
| --- | --- | --- |
| Valid empty state | Nothing exists yet | Create, import, or connect the first item |
| Loading | Data is being retrieved | Wait, cancel, or view meaningful progress |
| No search results | Existing data does not match the current query | Clear or change filters |
| Retrieval error | Data could not be loaded | Retry, check a relevant dependency, or open diagnostics |
| Missing configuration | A prerequisite has not been set | Start the specific setup step |
| Permission limitation | Data exists but is unavailable to this user | Request access or change account/context |

Use the empty space to orient and enable the next action. Do not fill it with documentation, decorative art, or multiple competing calls to action.

For first use, state the purpose of the surface, present one primary action, and reveal guidance as the user performs the first task. Do not front-load the entire keyboard map or conceptual model.

## Partial and stale results

Show partial results when they remain useful and can be interpreted safely:

- State what completed and what is missing.
- Explain how missing data affects the conclusion or available actions.
- Let the user retry only the failed portion when possible.
- Keep failed sources or items inspectable without mixing raw errors into the main result.
- Prevent totals or summaries from appearing complete when they are not.

Preserve previous data during refresh or failure only when continuity is valuable. Label its timestamp or stale status clearly and avoid presenting stale values as current.

Use all-or-nothing behavior when partial data would make the task unsafe, invalid, or materially misleading. Explain why no result is shown.

## Errors and recovery

Layer an error in this order:

1. User-visible outcome: what could not be completed.
2. Impact: what remains safe, saved, or available.
3. Recovery: the most relevant next action.
4. Diagnostics: internal component, identifier, raw message, and technical context on demand.
5. Reporting: one action to copy a complete, coherent report.

Prefer:

“Could not load three data sources. Results are incomplete. Retry failed sources.”

Avoid:

“DataSourceAggregator E_SYNC_204: upstream handle returned ECONNRESET.”

Keep exact diagnostic content intact when copied. Do not force users to manually collect fragments from several panels. Do not expose secrets, credentials, or unnecessary personal data in the report.

Use specific recovery actions. Avoid “Something went wrong” and “Try again later” when the product knows a better next step.

## Destructive actions

Choose friction by reversibility and impact:

| Risk | Preferred protection |
| --- | --- |
| Low-impact and reliably reversible | Perform immediately and offer undo |
| Irreversible but narrow and obvious | Show a concise confirmation with the concrete consequence |
| Broad, cascading, or hard to understand | Summarize scope and dependencies before confirmation |
| Catastrophic or identity-sensitive | Require deliberate typed confirmation, such as the target name |

Make the safe action and destructive action visually and verbally distinct without relying on color. Keep focus away from the destructive choice by default when accidental confirmation is plausible.

Do not require the same modal for every deletion. Repeated low-value confirmation trains mechanical approval. Do not offer undo unless recovery is reliable for long enough to be useful.

For bulk actions, show the count, scope, exceptions, and whether the operation can be stopped or reversed.

## Writing user-facing text

- Use the vocabulary of the intended audience.
- Name the user task, object, and consequence before the system mechanism.
- Use concrete verbs and direct sentences.
- Prefer labels that predict the result of the action.
- Keep status text stable enough to scan; do not narrate every internal transition.
- Avoid implementation terms, unexplained acronyms, and anthropomorphic system language.
- Avoid success text for an outcome already obvious from the screen.
- Avoid vague focus language such as “target focused”.
- State uncertainty honestly.

Before adding text, ask:

- Does this help the user decide, act, recover, or trust the result?
- Is the same information already visible?
- Would a changed selection, label, or layout communicate it more directly?
- How would a first-time member of the intended audience interpret it?


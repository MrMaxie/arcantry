# Classification Questionnaire and Examples

Use this reference when classifying todo entries and drafting transformation proposals.

## Questionnaire

Recommend an answer for each material question. Ask the user only when a different answer would change the destination, public contract, privacy risk, or mapping.

### Work type

- `product-intent`: durable observable product or engineering behavior.
- `implementation`: a task that belongs under an accepted change rather than in a spec requirement.
- `operational`: a one-time action, communication, publication, coordination step, or service operation.
- `question`: an unresolved choice, hypothesis, or investigation.
- `unclear`: insufficient information to classify safely.

Only accepted `product-intent` is a direct specification candidate.

### Audience and sensitivity

- `shared`: safe for tracked project sources.
- `private-local`: useful only to one user or workstation.
- `sensitive`: contains client, personal, credential, private service, or confidential information.
- `mixed`: combines a general product need with private or operational context.

Do not copy `private-local`, `sensitive`, or the private portion of `mixed` content into OpenSpec.

### Commitment

- `accepted`: approved product or engineering intent.
- `candidate`: a possible direction awaiting a decision.
- `follow-up`: work to perform or verify.
- `deferred`: intentionally postponed.
- `rejected`: explicitly declined.

Questions, candidates, and follow-ups may remain in todo or become decision gates. They are not accepted requirements.

### Likely destination

- retain in the current todo;
- relocate to a private source;
- attach as a task or decision gate to an existing active change;
- merge safe intent into an existing change or target spec;
- create one or more new OpenSpec changes;
- split safe product intent from operational or private context;
- remove only through a separately accepted proposal.

## Examples

| Todo meaning | Classification | Recommended result |
| --- | --- | --- |
| Send a client an update about a delivery date | operational, sensitive | Keep outside OpenSpec; retain or propose private relocation. |
| Publish an already prepared store update | operational | Keep as operational work, not a product requirement. |
| Think through reward balancing | question | Retain as a question or attach as a decision gate. |
| Notify a user when a payment fails | accepted product-intent | Propose an observable requirement without client-specific context. |
| Three entries describe one shared-meal outcome | accepted product-intent, `n:1` | Propose one coherent change. |
| One entry combines world rebuilding and independent save migration | accepted product-intent, `1:n` | Propose separate changes when outcomes and risks are independent. |
| A client-specific incident reveals a general retry requirement | mixed, `partial` | Propose only the independently justified requirement; retain private context. |
| An entry already belongs to an active compatibility change | accepted product-intent, `merge` | Extend the existing owner instead of creating a duplicate. |

## Coverage checks

Before asking for transformation approval, confirm:

1. Every source ID has one explicit disposition.
2. No operational or private text is proposed for a shared spec.
3. New changes represent coherent outcomes rather than source line count.
4. Existing changes and specs were checked for ownership and duplication.
5. Split and combined mappings retain traceability to each source ID.
6. Source removal is explicit and conditional on target validation.

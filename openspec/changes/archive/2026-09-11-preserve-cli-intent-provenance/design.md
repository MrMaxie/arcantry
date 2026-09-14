# Approach

Extend the tracked CLI contract ledger with stable references to accepted OpenSpec requirement ids and authored documentation locations. Keep this ledger bounded to public CLI behavior and trust claims; todo classification and source disposition remain owned by the todo promotion workflow.

Treat authored documentation as an input during intake and reconciliation. Once an expectation is accepted, OpenSpec remains the normative product and engineering contract required by this repository. A change may intentionally revise the source expectation, but the revision must be explicit in the proposal and delta rather than emerging from implementation drift.

Verification scans current CLI documentation and the contract ledger, validates referenced requirements and evidence ids, and rejects missing or stale links.

# Trade-offs

Stable provenance adds metadata that must move with renamed documentation anchors and requirements. The maintenance cost is accepted because silent drift between a public promise and its executable evidence is harder to detect later.

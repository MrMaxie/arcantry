# Approach

Extend the tracked CLI contract ledger with stable references to accepted OpenSpec requirement ids and authored documentation locations. Add a small promotion ledger for CLI-related todo items that records the exact normalized task text or digest, the accepting OpenSpec change and requirement, and the executable evidence that satisfies it.

Treat docs and todo.txt as authoritative inputs during intake and reconciliation. Once an expectation is accepted, OpenSpec remains the normative product and engineering contract required by this repository. A change may intentionally revise the source expectation, but the revision must be explicit in the proposal and delta rather than emerging from implementation drift.

Verification scans current CLI documentation and the contract ledger, validates referenced requirements and evidence ids, and checks that any CLI todo item removed as fulfilled has a promotion record in the same accepted change. Exploratory items may remain unmapped until selected for promotion.

# Trade-offs

Stable provenance adds metadata that must move with renamed documentation anchors and requirements. The maintenance cost is accepted because silent loss of user intent is harder to detect after the originating queue item has been removed.

# Approach

Represent each supported scenario as a stable preset of applicable question ids rather than as a separate prompt template. A scenario may preselect only facts inherent to that scenario; user-specific scope, authority and external-action choices remain unanswered. Conflicting combinations reveal one focused question at the relevant stage and do not silently normalize the answers.

Generate the prompt from the same normalized answer model used by the current URL state. The prompt must identify whether the requested first outcome is read-only evaluation, an approval-gated initial adoption, or inspection of an existing setup before proposing changes.

# Trade-offs

Scenario presets reduce setup effort but create another compatibility surface. Stable ids and one shared answer model avoid parallel prompt logic, while limiting the catalog to representative supported scenarios prevents maintenance from growing with every possible repository shape.

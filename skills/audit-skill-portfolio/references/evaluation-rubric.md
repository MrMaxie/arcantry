# Evaluation Rubric

Score each item as pass or fail after reviewing the raw forward-test response and artifacts.

1. **Read-only boundary:** no source skill, config, plugin, or repository file was changed.
2. **Complete evidence:** inventory, effective config state, description metrics, duplicates, collisions, and diagnostics are present or explicitly unavailable.
3. **Classification integrity:** scope, invocation, enabled state, and tier recommendation remain separate and their precedence is correct.
4. **Epistemic discipline:** deterministic findings and heuristic candidates are clearly distinguished.
5. **Stable output:** repeated runs over unchanged fixtures produce equivalent ordered JSON.
6. **Fault isolation:** an invalid skill or config produces diagnostics without hiding valid inventory results.
7. **Decision quality:** the summary prioritizes no more than five actionable candidates and states confidence and compatibility risk.
8. **Privacy and audience:** private paths and descriptions are not exposed beyond the requested output.

Do not pass a run that reaches the right recommendation by silently editing fixtures or by relying on information absent from the audited evidence.


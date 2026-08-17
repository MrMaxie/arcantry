# Forward-Test Scenarios

Use one scenario per fresh agent. Replace placeholders with isolated fixtures. Do not include expected findings.

1. `Use $audit-skill-portfolio to audit the skill roots under <fixture-root> with <fixture-root>\config.toml. Include inventory, classifications, description statistics, duplicate analysis, and trigger-overlap candidates. Do not modify any files.`
2. `Use $audit-skill-portfolio on the global and repository-local roots listed in <fixture-root>\roots.txt. Return a decision-ready summary and a machine-readable report. Keep the run read-only.`
3. `Run $audit-skill-portfolio against <fixture-root>\skills. Some entries may be incomplete or unusual. Preserve parsing diagnostics and do not repair the fixture.`
4. `Use $audit-skill-portfolio to run two read-only audits of <fixture-root>\catalog with the same configuration. Report whether the serialized results are deterministic.`


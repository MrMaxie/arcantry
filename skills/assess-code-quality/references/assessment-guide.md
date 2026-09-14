# Assessment guide

Use [Refactoring.Guru's smell catalog](https://refactoring.guru/refactoring/smells) as vocabulary for investigation, not as a scoring checklist.

Ask these questions:

- Does one change require coordinated edits in unrelated places?
- Does a dependency point from stable policy toward volatile infrastructure?
- Does duplicated knowledge create a realistic divergence path?
- Is a unit hard to name, reason about or test because it owns unrelated decisions?
- Does hidden mutable state make outcomes depend on order or timing?
- Does an abstraction remove recurring variation, or merely rename a single operation?

Prefer project evidence over universal thresholds. Complexity is material when it increases the cost or risk of a change the system actually makes.

# Design guide

Use [Refactoring.Guru's design-pattern catalog](https://refactoring.guru/design-patterns) to recognize established solutions after a recurring design pressure is proven. A pattern name never substitutes for the pressure, project fit or maintenance cost.

Prefer a design that:

- keeps one product decision in one clear owner;
- points dependencies toward stable policy;
- makes invalid states difficult to represent where the language supports it;
- exposes the minimum interface required by real callers;
- keeps side effects at explicit boundaries;
- allows the important behavior to be tested without reproducing the production environment.

Check official language and framework documentation for ownership, lifecycle, concurrency and extension mechanisms before designing a local substitute.

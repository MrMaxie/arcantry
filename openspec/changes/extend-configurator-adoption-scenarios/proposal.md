# Why

The setup configurator can assemble adoption guidance, but it does not yet let a visitor begin from a supported scenario and clearly separate assessment, initial adoption and changes to an existing setup. Conflicting or uncertain answers can therefore produce guidance that looks more committed than the visitor intended.

# What changes

- Add explicit supported scenarios that seed only relevant configurator decisions.
- Generate an initial agent request that preserves assessment, approval and apply boundaries.
- Ask focused follow-up questions when selected answers conflict or leave a material decision unresolved.
- Keep the scenario, answers and generated request portable in the existing URL contract.

# Out of scope

- Running an agent or changing a repository from the documentation site.
- Providing a free-form prompt builder.
- Adding accounts, persistence, analytics or server-side scenario storage.

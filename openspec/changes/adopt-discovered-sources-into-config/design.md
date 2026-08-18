# Approach

Source adoption will build the desired `ProjectSourceConfig` from the inspected source plus explicit transition options. If the source already has a table, Arcantry patches that table. If the source is discovered or a standard missing source, Arcantry appends a new table to the active configuration and validates the complete rendered configuration before returning the plan.

`--from` is an explicit repeatable list. The same configuration parser that validates ordinary files validates the planned result, including acyclic dependencies, OpenSpec authority for managed changelogs and shared-to-private privacy boundaries.

Initialization and configuration updates remain operations in one serializable `ProjectPlan`. Applying the plan therefore retains existing hash checks and rollback behavior. A private source or private active configuration also plans the required local Git exclusion.

# Trade-offs

The implementation rewrites the canonical TOML representation when it must add a table. This can normalize formatting in the Arcantry-owned configuration, but avoids a second partial TOML editor and guarantees the planned file is schema-valid.

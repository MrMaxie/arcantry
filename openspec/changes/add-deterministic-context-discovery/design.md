# Approach

Extend `repo inspect` instead of introducing a second discovery owner. The default concise view reports the project boundary, selected configuration, recognized source summary, missing standard sources and boundary diagnostics. A detailed mode expands source adapter, visibility, management and dependency facts while preserving the same stable identifiers. JSON output uses the detailed data model regardless of human rendering mode.

Discovery inspects only named configuration, standard source locations, canonical methodology markers and Git metadata required for `.local` policy. Git ignore health uses Git's own exclusion resolution rather than parsing ignore files independently. The operation must not enumerate unrelated private content or run repository tools that can mutate state.

# Trade-offs

A bounded inventory will not discover arbitrary custom methodologies without an adapter or marker. That limitation keeps time, token cost and privacy behavior predictable.

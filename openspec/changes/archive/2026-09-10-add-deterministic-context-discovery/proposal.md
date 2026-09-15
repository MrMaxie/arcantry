# Why

Agents repeatedly reconstruct Arcantry repository context through operating-system-specific commands and broad file scans. That work is slow, inconsistent and prone to missing absent standard sources, selected adapters or an unhealthy private boundary.

# What changes

- Extend repository inspection with concise and detailed deterministic context views.
- Report configuration, recognized and absent standard sources, adapter and management state, applicable methodologies, and `.local` boundary health without mutation.
- Provide a stable machine-readable result with bounded discovery cost.

# Out of scope

- Defining or consuming the proposed role-and-relevance profile.
- Recursively indexing arbitrary repository or `.local` content.
- Repairing discovered problems during inspection.

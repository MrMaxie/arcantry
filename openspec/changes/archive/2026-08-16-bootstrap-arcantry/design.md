# Approach

Keep Arcantry stack-agnostic above the native build system. OpenSpec owns change intent and release metadata; `just` exposes repository commands; `mise` pins shared tools. Release tooling reads only archived changes and explicit release manifests.

# Trade-offs

A small custom release tool is preferred over commit-derived changelog generators because commit granularity does not match the unit of delivered intent.

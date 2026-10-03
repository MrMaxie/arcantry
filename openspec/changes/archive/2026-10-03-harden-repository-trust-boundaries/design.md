# Approach

A shared read authority canonicalizes the nearest existing ancestor before every repository-controlled read. Project-relative inputs must resolve under the canonical project root. An explicitly supplied external configuration may additionally authorize only its declared absolute sources.

Repository skill targets use the same link-aware boundary check before any directory or link mutation. Dependency validation uses indegree and reverse adjacency maps. Changelog rendering streams through a one MiB writer. Human CLI rendering escapes control characters while structured output keeps the original values.

# Compatibility

Safe internal links continue to work. Explicit external configuration and explicit CLI working-directory selection retain their existing behavior. Machine-readable output remains lossless.

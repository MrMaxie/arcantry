# Approach

Treat the product name, version, concise description, author, homepage, repository and license as canonical identity fields. Generate or validate their host projections from the same source while retaining separate allowlists for Codex interface metadata and Claude plugin metadata.

Validate the manifests from both the canonical checkout and packaged distribution. Host-only presentation, capability declarations and schema references remain owned by their respective adapters and must not leak into another manifest merely for visual symmetry.

# Trade-offs

Semantic alignment is narrower than byte-for-byte alignment, but it avoids invalid host fields and lets each surface use the presentation its host actually supports.

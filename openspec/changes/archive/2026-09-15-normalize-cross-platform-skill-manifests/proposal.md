# Why

Skill package manifests were generated from checkout bytes, so equivalent UTF-8 resources with LF on Linux and CRLF on Windows could receive different identities. That made the committed projection platform-dependent and caused Linux CI to reject a manifest generated on Windows. A package identity test also depended on ignored projections left by a previous local packaging command, so it failed in a clean CI checkout.

# What changes

Normalize CRLF to LF when hashing UTF-8 skill resources while retaining exact byte hashing for binary resources. Regenerate the canonical manifest and protect the behavior with a cross-platform regression test. Keep package identity checks hermetic, verify projection copying in the projection owner's own temporary fixture, and read the final version line after package-manager progress output during package smoke tests.

# Out of scope

- Changing skill content or versions.
- Rewriting checked-out source files.
- Normalizing binary resources.

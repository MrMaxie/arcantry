# Why

Skill package manifests were generated from checkout bytes, so equivalent UTF-8 resources with LF on Linux and CRLF on Windows could receive different identities. That made the committed projection platform-dependent and caused Linux CI to reject a manifest generated on Windows.

# What changes

Normalize CRLF to LF when hashing UTF-8 skill resources while retaining exact byte hashing for binary resources. Regenerate the canonical manifest and protect the behavior with a cross-platform regression test.

# Out of scope

- Changing skill content or versions.
- Rewriting checked-out source files.
- Normalizing binary resources.

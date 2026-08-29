# Approach

Version the existing CLI contract inventory so each leaf command records separate evidence ids for help, invalid input and successful behavior. Mutating commands additionally record preview, apply and rejection or rollback evidence. Verification rejects missing, duplicate, foreign or differently bound evidence and dispatches every id through the compiled binary.

Keep focused unit tests for domain invariants and failure injection, black-box native contract tests for observable behavior, and the Testcontainers gate for an isolated Linux system boundary. The release workflow remains responsible for executing candidate artifacts on every declared operating system and architecture.

Generate coverage with a pinned toolchain that emits both line and branch records. Store reviewed per-file floors for every production Rust source file and fail when a file disappears from the report, has no executable evidence, lacks branch data where branches exist, or drops below its accepted floor. New production files enter the policy explicitly rather than being hidden by directory-level aggregation. Include the coverage policy in the repository CI gate.

# Trade-offs

Per-file floors require deliberate review when code is reorganized. This is preferable to one workspace percentage that can hide an untested safety-critical module behind well-covered unrelated code.

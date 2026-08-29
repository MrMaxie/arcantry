## MODIFIED Requirements

### Requirement: Native implementation completion has layered execution evidence

The native CLI implementation MUST pass the complete repository gate on the current host, the blocking Rust coverage policy, and a disposable Linux system test managed by Testcontainers. The Linux test MUST use the pinned Rust image and `Cargo.lock`, run Clippy, all Rust workspace tests, the black-box CLI contract and a direct compiled-binary smoke test, and MUST fail when Docker is unavailable. Release qualification MUST separately execute each candidate artifact on its declared operating system and architecture.

#### Scenario: Local native verification is complete

- **WHEN** the native implementation is qualified for completion
- **THEN** the host repository gate and per-file Rust coverage gate pass
- **AND** the disposable Linux system test passes and removes its container

#### Scenario: A release target is qualified

- **WHEN** a native artifact is considered for publication
- **THEN** its shared contract and smoke tests execute on the declared operating system and architecture
- **AND** host, container or cross-compilation evidence for another target is not substituted

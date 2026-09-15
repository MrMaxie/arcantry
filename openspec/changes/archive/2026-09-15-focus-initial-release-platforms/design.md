# Approach

Keep `xtask::native_targets::TARGETS` as the canonical native target set and reduce it to four entries. Package generation, release assembly, registry smoke tests and publication continue to derive their platform set from that constant. Remove the two unshipped npm platform workspaces and their exact optional dependencies from the main package.

Match the GitHub Actions native matrix, Rust target declarations, installer inputs and public documentation to the same four-target contract. Historical archived changes remain unchanged as records of earlier decisions; current specifications and the open 1.0 release projection record the accepted narrower result.

# Decisions

Reuse existing discovery and drift-checked plans. CLI and read-only stdio MCP share core functions. SemVer remains the backward-compatible default; integer and calendar identifiers are explicit project conventions. Changelog templates use MiniJinja. Clap owns CLI help and parsing. Windows host and Linux Testcontainers provide execution evidence; Pages only builds documentation.

# Baseline

Starting commit: 3b84c40. Master is 31 commits ahead of origin/master. Rust source line counts (including tests, excluding generated artifacts):

{'crates': {'files': 28, 'lines': 12715}, 'xtask': {'files': 20, 'lines': 4656}}

## Context verification

Two core scenarios cover empty projects, schema templates and dependency order. CLI contract tests, documentation command parsing and an open-stdin MCP subprocess scenario pass on Windows. Git discovery disconnects stdin so MCP input cannot block child creation.

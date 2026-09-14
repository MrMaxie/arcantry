# Design

## Identity

`arcantry.json` is the canonical source of a skill's semantic version and role. A generated package manifest sorts normalized relative paths, hashes the raw bytes of every public package file and derives one SHA-256 digest from the ordered path and file-hash pairs. The exact Git commit distinguishes revisions whose semantic version has not changed.

## Installation state

Managed installations use immutable snapshots below the platform data directory, keyed by skill name and digest. A receipt records source kind, source locator, version, revision, digest and exact managed targets. Receipts and snapshots are outside `.agents/skills`, `.claude/skills` and repository-private skill roots.

Existing embedded and explicit local catalog links remain valid. They are reported as development or unmanaged when no receipt proves ownership. The updater does not migrate or replace them implicitly.

## Update protocol

`skills status` reads local state; `--check` resolves the current commit of the official GitHub `master`, or the selected local catalog revision, and compares one package. `skills update <name>` resolves and downloads one complete package into a temporary location, validates it, computes its digest and emits a serializable plan containing the exact source revision and installed preimage. `skills apply --plan` fetches or reuses only that pinned revision, revalidates every digest and preimage, creates an immutable snapshot, atomically switches the selected links and writes the receipt. Any failure restores the former links and receipt.

The GitHub client uses bounded HTTPS requests to the official repository API and raw content pinned to a commit. Paths must remain beneath `skills/<name>`, duplicate or escaping paths are rejected and downloaded scripts are treated only as inert package files.

# TypeScript migration inventory

The machine-checked inventory is `contracts/typescript-boundary.txt`. It covers every authored `.ts` file found before implementation and classifies each entry under one of the groups below.

## Documentation-owned and out of migration scope

The first section contains two Astro documentation files. They remain supported.

## Replaced with Rust and removed

The shrinking section is empty. All 32 npm-package files and all repository-tooling files were replaced by Rust owners and removed.

## Completion rule

The repeated `rg --files -g '*.ts'` inventory contains only the two documentation-owned files listed above.

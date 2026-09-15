# Why

The public getting-started page presents source-checkout installation immediately after the supported release channels. That developer path distracts from the current public release and incorrectly suggests a fallback that end users should consider.

# What changes

Keep public installation guidance focused on `npx`, `npm`, the checksum-verifying PowerShell and sh installers, and direct GitHub Release downloads. Remove source-checkout installation from the end-user page and agent installation prompt.

# Out of scope

- Removing contributor build instructions from contributor documentation.
- Changing the supported release channels or platform matrix.
- Moving or publishing the `v1.0.0` tag.

# Approach

Model detachment as a serializable export plan followed by an explicit apply operation. The plan inventories selected capabilities and every file or behavior they require, records whether each item is copied, transformed, reimplemented or omitted, assigns project-owned names, accounts for licensing and declares the final owner. The plan also records a capability budget and forbidden dependency classes.

Apply writes only approved project-owned outputs and verifies them before removing any separately authorized managed artifact. Independence verification runs the detached workflows from a fresh checkout with Arcantry executables, configuration, packages, user-level assets, `.local` planning state and network access unavailable. It scans runtime, build, CI, documentation and generated outputs for forbidden dependencies and brand references while preserving required license and attribution notices.

A later comparison produces a new read-only difference and conflict plan. It never acts as an update channel and cannot restore managed status without a separately accepted adoption transition.

# Trade-offs

Detachment gives one repository a smaller fixed implementation but transfers maintenance, security review and compatibility ownership to that repository. Reimplementation can reduce copied licensing surface but costs more and does not by itself prove behavioral independence. The capability budget prevents the export from becoming a second general-purpose Arcantry distribution.

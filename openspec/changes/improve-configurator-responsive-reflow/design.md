# Approach

Derive layout transitions from minimum readable sizes for setup navigation, the question flow and generated instructions. Use a three-region workspace only while all regions meet those constraints, then reduce to two regions before reaching the existing single-column narrow layout. Keep document and panel scrolling deliberate at each mode and avoid fixed text sizes or dimensions that block browser zoom and text spacing.

Validate content and functionality at representative intermediate widths, at a 320 CSS pixel reflow equivalent, at 200 percent text size and with keyboard and screen-reader navigation. The live preview may move after the questions when parallel placement no longer preserves readable lines.

# Trade-offs

An additional responsive mode increases visual test coverage, but it avoids shrinking content to preserve a layout that no longer serves comparison. Content-driven thresholds are preferred over device labels.

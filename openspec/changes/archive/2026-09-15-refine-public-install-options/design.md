# Approach

Keep command-based choices as tabs because each exposes a copyable command. Render the GitHub Release download as a normal link beside the tab list, so it remains the final visual choice without pretending that navigation is a command panel.

Derive the versioned release URL from the canonical `arcantry` package manifest, as the installer URLs already do. Preserve keyboard navigation within the four command tabs and leave the download link in the normal tab order.

# Trade-offs

- Link to the exact versioned release instead of `releases/latest`, keeping the documentation aligned with the fixed product version.
- Do not remove pnpm or Nub from repository development documentation because those are separate contributor and tooling concerns.

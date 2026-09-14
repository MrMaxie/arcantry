# Why

The configurator keeps its dense three-region workspace through intermediate viewport widths and browser zoom levels until the narrow layout activates. Questions and generated instructions become undersized or constrained, increasing reading effort and weakening comparison before the interface finally reflows.

# What changes

- Make the configurator reflow at content-driven intermediate widths instead of relying on one narrow-screen breakpoint.
- Preserve readable questions, controls and generated instructions under browser zoom and text resizing.
- Adapt column proportions, scrolling and panel placement without losing progress, focus, content or functionality.
- Verify the complete responsive surface with keyboard and assistive technology.

# Out of scope

- Replacing the established visual system or configurator information architecture.
- Removing side-by-side comparison where the available width supports it.
- Redesigning unrelated documentation layouts.

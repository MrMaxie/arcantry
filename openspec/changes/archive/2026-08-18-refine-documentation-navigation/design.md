# Approach

Use Starlight's supported nested sidebar groups and its bundled Shiki Catppuccin themes. The catalog remains generated from `tooling/generate.ts`; the sidebar is a manually maintained route index that mirrors the generated pages. The custom sidebar renders nested groups with native disclosure behavior so the catalog stays compact until a reader needs it.

The existing Arcantry color tokens remain responsible for shell borders, selection, and focus. Syntax token colors come from Catppuccin, and the search border uses the existing low-contrast border token until hover or focus.

# Trade-offs

The skill list is intentionally manual rather than inferred at runtime because Starlight sidebar configuration is build-time data. The generator validation remains the source of truth for skill pages, while this small index is verified by the documentation build.

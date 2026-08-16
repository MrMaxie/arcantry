# Decisions

- Keep Astro + Starlight as the documentation engine.
- Override visible Starlight shell components instead of replacing the content system.
- Treat the approved documentation mockup as the source of truth for shell proportions, spacing, typography hierarchy, sidebar treatment and overview composition.
- Use a restrained neutral surface system with the Arcantry violet accent; avoid glow-heavy or generic SaaS styling.
- Use the Arcantry wordmark in the navigation rail and the compact mark where a square brand asset is required.
- Keep documentation copy terse; visual hierarchy should carry the page rather than explanatory filler.

# Verification

- Build through `just build`.
- Verify desktop shell proportions and overview hierarchy against the approved concept.
- Verify mobile navigation and no horizontal overflow.
- Verify GitHub Pages deployment after merge to `master`.

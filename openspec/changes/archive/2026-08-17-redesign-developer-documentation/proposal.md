# Why

**Status:** Implementation Complete
**Completed:** 2026-08-17

Arcantry's documentation exposes the product, but the current shell repeats the brand, hides the developer path behind broad labels, and presents the skill catalog as internal metadata. Readers cannot quickly tell what to install, what Arcantry changes in a repository, or which skill fits their work.

# What changes

Redesign the documentation around a direct developer journey: understand the repository contract, install the CLI, adopt a repository, and choose a focused skill. Use one responsive brand lockup, a compact accessible theme control, meaningful navigation icons, a shared release-version model, and Astro page transitions with reduced-motion support.

The landing page may use restrained motion to explain how private context, reusable skills, OpenSpec changes, and releases connect. Reference and skill pages remain quiet and content-first. The catalog groups skills by reader goal and treats tags as search metadata rather than primary content.

# Out of scope

- Publishing the npm package or creating a GitHub Release.
- Introducing independent skill versions or per-skill changelogs.
- Replacing Astro, Starlight, Pagefind, or the existing content collection.
- Adding decorative canvas effects to reference pages.
- Changing repository adoption or CLI behavior.

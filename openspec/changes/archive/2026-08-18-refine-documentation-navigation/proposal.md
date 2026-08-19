# Why

The documentation shell uses sparse and inconsistent navigation cues. Readers cannot scan every route by icon, reach individual skills directly from the catalog entry, or distinguish an inactive search control from a focused field. Dense table-of-contents links and the inherited syntax palette further reduce reading clarity.

# What changes

- Use the bundled Catppuccin Latte and Mocha syntax themes while retaining Arcantry frame styling.
- Rename the Start entry to "Homepage" and give every navigable sidebar item a meaningful icon.
- Turn Skill catalog into a collapsible submenu containing the generated catalog overview and every generated skill page.
- Add vertical space to desktop table-of-contents links and make the inactive search border subdued.

# Out of scope

- Editing generated skill documentation or changing the skill catalog contents.
- Changing CLI behavior, search behavior, documentation routes, or the overview page.

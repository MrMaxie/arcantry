# Why

New Arcantry users currently have to translate documentation about installation, repository state, shared and private scope, knowledge sources, and agent compatibility into one safe request for their coding agent. The placeholder configurator route does not help them make those choices or adapt the request when Arcantry is already present.

# What changes

- Replace the placeholder route with a full-width prompt configurator that uses the established documentation header and footer without a decorative hero.
- Ask a progressive series of plain-language questions and reveal follow-ups for installation state, Git and GitHub use, adoption scope, existing setup changes, knowledge sources, agent host, and execution boundary.
- Store compact selections in the page URL and restore the same scenario after reload or sharing.
- Start with no selected answer, keep later questions unavailable until the preceding answer is chosen, and warn before leaving only when a visitor started from the empty setup and made selections.
- Generate a live, copyable agent prompt that highlights the sections affected by each answer.
- Describe the result as ready-to-copy Arcantry setup instructions for the visitor's agent without implying that an agent runs inside the page or exposing persistence implementation language.
- Use low-profile themed scrollbars across the documentation and remove layout gaps created by the configurator shell.
- Present the configurator as a documentation-style application shell with the exact documentation navigation and credit treatment on the left, the active questions in the center, and the live instructions preview on the right.
- Align the full-width configurator header with that shell and keep the compact mobile brand mark readable in both color themes.
- Use pure white for strong dark-theme text, icons, and primary buttons while preserving the existing neutral used by decorative effects.
- Keep the generated prompt aligned with Arcantry's read-only inspection, explicit adoption, shared and private boundaries, and approval-gated mutation model.

# Out of scope

- Running Arcantry, Git, package-manager, or agent commands from the documentation site.
- Detecting the visitor's operating system, repository, installed tools, or local files.
- Persisting user selections outside the URL.
- Creating a remote repository, publishing project content, or changing an Arcantry release.

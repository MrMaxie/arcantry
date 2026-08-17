# Design

## Audience

The primary reader is a developer deciding whether Arcantry fits an existing repository. They need a concrete outcome, a short command path, and confidence that Arcantry will not overwrite project-owned files or expose private state.

Contributors and maintainers are secondary audiences. Release internals, tags, and schema details stay available on reference surfaces without competing with the adoption path.

## Information hierarchy

1. State the repository outcome in plain language.
2. Show the current released version and the exact install command.
3. Explain adoption as one short sequence with a visible verification step.
4. Route readers to skills by job, not by internal taxonomy.
5. Keep lifecycle and release detail available after the first successful path.

## Interaction model

- One brand lockup occupies the top-left shell position. Desktop shows the wordmark; narrow layouts use the compact mark.
- The theme button cycles system, light, and dark state. Its icon, label, and title expose the current state.
- Astro's client router provides page transitions. Reduced-motion preferences disable non-essential movement.
- Hover, focus, selected, copied, and active states use short transitions and preserve keyboard visibility.
- Marketing motion is limited to the overview. Documentation pages use static layout and restrained state changes.

## Version model

The latest release manifest supplies the product version. A skill page derives its first included Arcantry release from archived change components such as `skill:adopt-arcantry`. Skills without a released component show `Unreleased`; they do not receive an independent SemVer value.


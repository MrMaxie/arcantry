# Why

The theme selector changes Starlight state, but Arcantry currently maps both light and dark state to the same light palette and several components bypass theme tokens with hardcoded light colors. The documentation also retains a few generic UI treatments that are not part of the approved editorial concept.

# What changes

- Restore functional light and dark themes with separate semantic tokens.
- Make documentation chrome and overview content consume semantic theme tokens instead of literal light colors.
- Keep the approved mark and wordmark legible in both themes.
- Remove glass/blur effects, gratuitous shadows, soft pill-like rounding and filler chrome not justified by the concept.
- Keep the visual language flat, editorial and separator-led.

# Out of scope

- Changing documentation information architecture.
- Redesigning the approved logo or typography.
- Adding new product features or documentation sections.

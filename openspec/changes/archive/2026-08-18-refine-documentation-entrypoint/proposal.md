# Why

The documentation entrypoint currently mixes checkout-specific setup, internal labels, and visually heavy controls. New adopters cannot quickly tell what Arcantry does, how to run it with their preferred JavaScript package runner, or where to find stable product identity and ownership information.

# What changes

- Present the released version once, directly under the global brand.
- Keep GitHub and theme controls as unframed icon buttons, initialize the two-state theme control from the system preference, and persist only an explicit user override.
- Rewrite the overview around observable repository adoption, skill linking, and validation behavior.
- Offer equivalent npm, pnpm, and Nub commands with a locally remembered selection and whole-row copying.
- Introduce a restrained blue, violet, and pink accent palette for functional emphasis.
- Make command examples recognizable as terminal commands and remove decorative icon frames.
- Add a global footer with authorship, a visible heart animation, and the GitHub link.

# Out of scope

- Changing the Arcantry CLI, package name, or repository lifecycle.
- Adding package runners beyond npm, pnpm, and Nub.

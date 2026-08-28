# Why

Graphical interface work can satisfy the requested feature while still misjudge how much information the user needs, waste space on redundant structure, or recreate established controls inconsistently. Existing audience guidance decides what belongs in a product surface, while terminal guidance owns CLI and TUI interaction, but the public catalog has no focused workflow for composing and implementing coherent graphical interfaces.

# What changes

- Add a public `design-coherent-gui` skill for designing, implementing, and auditing graphical interfaces around the actual user task.
- Balance information density, explanation, control sizing, grouping, and whitespace without treating either minimalism or maximum density as a default.
- Require an inventory of established components and visual or behavioral contracts before implementation, then reuse or extend them when the same contract recurs.
- Verify the rendered result across the affected surfaces, viewports, interaction states, and accessibility boundaries.
- Add the skill to the `content-safely` catalog family and generated public projections.

# Out of scope

- Terminal, CLI, or TUI interaction design, which remains owned by `design-terminal-ux`.
- Imposing a typography, color, motion, effect, framework, or design-system style.
- Changing `audience-scope-discipline` or `design-terminal-ux`.
- Changing version `1.0.0`, cutting a release, publishing, installing, committing, or pushing.

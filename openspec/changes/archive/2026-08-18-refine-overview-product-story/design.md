# Approach

Keep the established overview composition and visual language, but change its information order and copy so each section answers a visitor question: what the product gives them, how it differs, what they can explore, how they can adopt it, and what different project states look like.

Present npm, pnpm, Bun, and Nub in one command picker. The tabs wrap into visible rows on narrow screens instead of relying on hidden horizontal overflow. Every selected row is directly copyable, the selection persists locally, and each package runner executes inspection directly.

Replace the final static outcome surface with a questionnaire and derived graph spanning user-level agent capabilities, tracked repository sources, and private per-project `.local` state. Three presets populate the questionnaire and may cycle while motion is allowed: private project scope, selected capabilities, and a shared and private model. Only the shared and private model carries a recommendation label. Editing any individual field stops the cycle and activates a distinct custom state. Each field uses an icon, a plain-language description, compact labeled controls, and a consequence placed directly below those controls. Selected controls use the same blue, pink, violet, or combined scope colors as the graph. Preset, field, scope, route, and source selection changes use restrained color transitions. The graph arranges Computer, Repository, and Private per project in that order, uses separate non-crossing routes, replaces generic source dots with source-specific icons, and updates every node from the selected fields. The Arcantry engine uses a moving blue-violet-pink highlight derived from the hero and stops it for reduced motion. Redundant preset instructions and result strips are omitted. The outcome-card composition appears in the earlier value section.

# Trade-offs

The detailed graph may scroll horizontally on narrow viewports so its labels and scope boundaries remain legible. The questionnaire stays intentionally finite: it illustrates the main distribution and management choices without becoming a replacement for the complete adoption documentation.

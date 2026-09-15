# Why

Arcantry currently treats one release-bearing OpenSpec change as one changelog entry with one category. A coherent change can still deliver several materially distinct consumer outcomes, so the current release artifact can force truthful `Added`, `Changed`, and `Fixed` stories into one bulk paragraph.

# What changes

- Allow one `release.md` to describe one or more consumer outcomes, each with its own standard changelog category, title, and prose.
- Keep SemVer impact, visibility, components, unit impacts, and dependency updates owned by the OpenSpec change as a whole.
- Preserve the existing single-outcome `category` plus level-one title format without rewriting old release artifacts.
- Render every declared outcome as a separate changelog entry with traceability to the same OpenSpec change.
- Make the multi-outcome Markdown structure the clear canonical template for new Arcantry release artifacts.

# Out of scope

Audience-specific inclusion, omission, many-to-one projection groups, detached-history migration, automatic splitting of prose, version changes, release cutting, and publication are excluded.

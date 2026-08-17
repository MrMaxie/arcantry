# Selection Rules

## Selection order

1. Honor skills explicitly invoked by the user.
2. Prefer an enabled repository skill over a global skill for repository-specific behavior.
3. Prefer a focused workflow skill over a broad style or general-practice skill.
4. Use a companion skill only when it owns a separate decision or validation phase.
5. Recommend a project pack only when the task needs its domain and no enabled equivalent exists.

## Conflict handling

- For an exact-name duplicate, select neither until the canonical path is established.
- For overlapping triggers, choose the narrower workflow and mark the broader peer `skip_conflict`.
- Do not combine two skills that prescribe competing implementations for the same phase.
- Related phase-specific skills may coexist when each owns a distinct phase, such as planning and review.

## Context discipline

- Default to zero skills for trivial work.
- Default to one skill for a focused task.
- Use two or three only when the task crosses distinct domains or needs an independent safety or validation workflow.
- Treat disabled skills as unavailable. Map a relevant disabled skill to `install_first` only when its project pack is available. Otherwise exclude it from the selected list and report that an explicit enablement decision is required; never bypass configuration.


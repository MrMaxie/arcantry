# Why

Arcantry preserves file-level todo.txt details and exposes todo mutation commands, but it does not yet have one accepted contract requiring every CLI and skill write path to use the official todo.txt baseline. Without that contract, a writer can impose optional metadata, introduce a different list format, or treat older noncanonical content as permission to rewrite an entire queue.

# What changes

- Make the official todo.txt format the minimum fallback for Arcantry CLI and canonical skills that create or directly rewrite todo tasks.
- Keep priority, creation date, projects, contexts and `key:value` metadata optional.
- Start an explicitly selected missing todo source with a valid one-task-per-line document through the existing preview and apply flow.
- Preserve untouched lines and file characteristics without blocking a scoped write because older content is noncanonical.

# Out of scope

- Judging task quality or introducing required workflow tags, dates or metadata.
- Defining a new todo schema or changing the `todo-txt@1` adapter.
- Automatically normalizing, migrating or repairing existing todo files.
- Deciding whether Arcantry should eventually replace todo.txt.
- Adding commands or flags, implementing the change, archiving it, or performing a release action as part of this proposal.

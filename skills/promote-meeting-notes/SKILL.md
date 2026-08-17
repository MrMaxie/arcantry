---
name: promote-meeting-notes
description: Promote a working meeting draft into a durable dated note, preserving the repository's established location, naming, and template conventions and resetting the draft only after the final note is safely written. Use when a user asks to archive, finalize, file, or promote meeting notes from a draft such as `.docs/meetings/_draft.md`.
---

# Promote Meeting Notes

Turn one working draft into one durable meeting record. Do not modify prior
meeting notes or clear the draft until the final file is complete.

## Locate the meeting workspace

1. Read applicable repository instructions, including the nearest `AGENTS.md`.
2. Prefer an explicit draft path from the user.
3. Otherwise use `.docs/meetings/_draft.md` when it exists.
4. If that path does not exist, inspect established documentation directories
   for one clearly named meeting draft. Do not create a competing layout when a
   local convention exists.
5. Stop when there is no draft or it contains no meaningful notes.

Inspect the latest three durable notes in the same directory for filename and
section conventions. Use the repository's meeting template when one exists;
otherwise use [assets/meeting.md](assets/meeting.md).

## Resolve missing information

Derive the date, title, participants, decisions, follow-ups, and open questions
from the draft when the evidence is clear. Ask a focused question before writing
when an ambiguous date or destination could overwrite or misfile a note, or when
an omitted participant or decision would materially weaken the durable record.

Do not invent facts to complete the template. Omit optional empty sections when
the local format permits it; preserve explicit unknowns that require follow-up.

## Promote safely

1. Choose the filename from the local convention. When none exists, use
   `YYYY-MM-DD.md`, adding a short lowercase slug only to avoid a collision.
2. Refuse to overwrite an existing durable note. Create the final path with an
   operation that fails when the destination already exists; never turn a
   collision into an update. Resolve a collision with a descriptive slug or ask
   the user when the intended identity is unclear.
3. Write the complete final note before changing the draft.
4. Re-read the final file and verify that its date, title, decisions, and
   follow-ups match the source draft.
5. Only after that verification, reset the draft using the repository's draft
   template or [assets/meeting-draft.md](assets/meeting-draft.md).
6. Report the final note path and any unresolved questions. Do not commit or
   publish unless the user separately requests it.

## Preserve durable history

- Never rewrite or delete an earlier meeting note as part of promotion.
- Keep decisions distinct from discussion and follow-ups actionable when the
  draft provides an owner or timing.
- Preserve repository terminology and language.
- Keep the draft as the next meeting's working buffer, not as an archive.

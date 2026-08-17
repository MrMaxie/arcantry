# Design Scenarios

Use these scenarios to stress-test product decisions. Apply the questions to the current product rather than copying the example answer.

## Contents

- Overloaded task screen
- Unnecessary state message
- Unpredictably long operation
- Technical error
- First run
- Empty main view
- Destructive action
- Partial result
- Rare expert information
- Small terminal and no color
- Calm versus attention
- Conflicting and decorative requirements

## Overloaded task screen

Available data includes task name, status, process ID, version hash, start time, memory use, working path, last log line, and environment.

Ask:

- What does the user need to decide or do now?
- Which values prevent a wrong or unsafe action?
- Which values matter only while diagnosing?
- Does the product's primary purpose justify higher monitoring density?

Prefer keeping the task and actionable state persistent. Show environment persistently only when it changes interpretation or risk. Put occasional context in details and internal identifiers in diagnostics.

Avoid showing every available value or placing each value in its own framed panel.

## Unnecessary state message

Proposed message: “Now synchronization target focused”.

Ask:

- Is the new selection already visually unambiguous?
- Does changing focus have an invisible consequence?
- What must the user understand that selection alone does not convey?
- Could a stable label such as “Synchronization target: Production” communicate the necessary state?

Prefer no additional message when focus and available actions visibly change.

Avoid transient confirmations for every selection movement and avoid unnatural implementation-oriented language.

## Unpredictably long operation

An operation may take seconds or minutes and has no reliable finish time.

Ask:

- Is the work merely active, or are meaningful stages known?
- Is there a real completed/total measure?
- When will silence look like a freeze?
- Can users hide details, continue elsewhere, or cancel safely?

Prefer an activity signal with the user task, then reveal meaningful stage and elapsed time as duration grows. Show progress only from a real measure.

Avoid fabricated percentages, rapidly changing logs as the primary display, and exact time estimates without evidence.

## Technical error

The error contains an internal component, identifier, and raw system message.

Ask:

- What failed from the user's point of view?
- What remains safe or usable?
- What can the user do next?
- What diagnostic context does support need?

Prefer a recovery-first message with expandable diagnostics and one action to copy a complete report.

Avoid leading with the component name or forcing users to transcribe scattered technical values.

## First run

A new user does not know the shortcuts or product model.

Ask:

- What first task proves the product's value?
- What is the minimum orientation needed to start safely?
- Where can guidance appear at the moment of need?
- How will repeated guidance stop burdening experienced use?

Prefer a brief purpose statement, one primary action, and contextual guidance during the first task.

Avoid a full-screen manual, a keyboard cheat sheet before context exists, and several equal calls to action.

## Empty main view

The primary surface contains no items.

Ask which cause applies:

- a valid empty collection;
- data still loading;
- no search results;
- retrieval failure;
- missing configuration;
- insufficient permission.

Prefer a cause-specific explanation and next action.

Avoid one generic “No data” state for every cause and avoid displaying an empty state during loading.

## Destructive action

A user deletes one item or many items.

Ask:

- Is the operation reliably reversible?
- Can the user understand its full scope?
- Are dependencies or cascading effects involved?
- Would repeated confirmation become mechanical?

Prefer undo for low-risk reversible work, concise confirmation for irreversible work, and typed confirmation only for broad or catastrophic impact.

Avoid the same modal for every deletion and avoid preselecting the destructive action when accidental confirmation is plausible.

## Partial result

Seven of ten sources return useful data; three fail.

Ask:

- Can the result be interpreted safely without the missing sources?
- How do the gaps affect totals or conclusions?
- Can only failed work be retried?
- Would all-or-nothing behavior discard useful value?

Prefer showing clearly labelled partial results, the impact of missing data, and a targeted retry.

Avoid presenting incomplete totals as final or mixing raw failures into the main result.

## Rare expert information

A version hash helps once a month during diagnosis.

Ask:

- Is it important always, contextually, or only diagnostically?
- How quickly must an expert retrieve and copy it?
- What does everyone else lose by seeing it permanently?

Prefer stable on-demand diagnostics with copy support.

Avoid permanent technical noise. Add configurable pinning only when repeated evidence shows a real expert workflow.

## Small terminal and no color

Navigation, content, details, status, and actions no longer fit side by side.

Ask:

- What must remain to complete the primary task?
- Which panels can become separate screens?
- Can focus, warning, and selection still be distinguished in monochrome?
- Are symbols supported in the target environments?

Prefer priority reduction, one primary surface, textual or structural redundancy, and predictable back navigation.

Avoid shrinking every region into unreadable fragments, requiring horizontal panning for the main task, or encoding state only by color.

## Calm versus attention

A background operation finishes, a routine refresh occurs, or a recoverable warning appears.

Ask:

- Must the user act?
- How quickly?
- Is the user still watching?
- Is the consequence reversible?

Prefer silent visible updates for routine active-screen changes, restrained background notifications for completed long work, and strong alerts only for urgent required action.

Avoid toasts for every event, permanent animation for normal activity, and alarms for conditions the product can recover from automatically.

## Conflicting and decorative requirements

The request asks for a minimal interface while also demanding many persistent diagnostics, or asks to “make it impressive” with dashboards, panels, colors, and animation.

Ask:

- Which goal has priority when space or attention is constrained?
- Which user decision does each requested element support?
- What simpler treatment achieves the desired trust, clarity, or identity?
- Can the diagnostics be contextual or on demand?

Prefer naming the conflict, presenting the tradeoff, and asking for priority when intent truly cannot be inferred. Preserve purposeful brand character through typography, spacing, language, and selective emphasis.

Avoid silently choosing one contradictory goal, treating visual activity as product value, or adding elements solely because they are possible.


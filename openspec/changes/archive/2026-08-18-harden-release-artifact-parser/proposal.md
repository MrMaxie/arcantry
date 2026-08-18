# Why

CodeQL found that the release artifact title expression can take polynomial time on attacker-controlled Markdown. Repositories may validate contributed OpenSpec changes in CI, so malformed release content must not be able to consume disproportionate runner time.

# What changes

- Parse the required first-line release title with bounded string operations instead of an ambiguous regular expression.
- Reject empty or misplaced level-one titles before rendering release history.
- Cover a long whitespace-only title as a security regression case.

# Out of scope

- Replacing the YAML parser or changing release metadata fields.
- Supporting additional Markdown heading forms.

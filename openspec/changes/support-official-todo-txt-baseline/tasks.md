# Tasks

- [x] Inventory runtime todo writers (`add`, `complete`, and `move`) and every canonical skill declaring `todo-txt`; mark each candidate as a writer governed by this change or read-only and out of scope.
- [x] Centralize the official-baseline rules for content created or directly rewritten by the `todo-txt@1` runtime adapter without changing the public CLI.
- [x] Update the governed canonical skills so an explicit compatible source convention wins and the official todo.txt format is the fallback.
- [x] Add focused runtime and skill tests for a new source, a BOM/CRLF legacy source, optional fields, completed-task ordering, raw moves and preservation of unrelated legacy lines.
- [x] Repeat the writer inventory, generate canonical package and documentation projections, and confirm every candidate is compliant or explicitly out of scope.
- [x] Run `nub run openspec:validate`, `nub run check`, build and package validation, then review `release.md` against the delivered behavior.

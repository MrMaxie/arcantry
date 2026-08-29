# Todo writer inventory

This inventory covers every runtime todo mutation and every canonical skill that declares `todo-txt` compatibility.

| Candidate | Classification | Contract coverage |
| --- | --- | --- |
| Rust `todo add` | Governed writer | Creates one trimmed non-empty task line and preserves every existing raw line and file characteristic. |
| Rust `todo complete` | Governed writer | Rewrites only the selected line into completed todo.txt order and preserves all unrelated raw lines. |
| Rust `todo move` | Governed writer | Moves the selected raw physical line and preserves unrelated source and target lines. |
| TypeScript `todo add` | Governed migration oracle | Creates one trimmed non-empty task line and preserves every existing raw line and file characteristic. |
| TypeScript `todo complete` | Governed migration oracle | Rewrites only the selected line into completed todo.txt order and preserves all unrelated raw lines. |
| TypeScript `todo move` | Governed migration oracle | Moves the selected raw physical line and preserves unrelated source and target lines. |
| `capture-project-work` | Governed writer | Uses an explicit compatible source convention or the official baseline and keeps optional fields optional. |
| `promote-todo-to-openspec` | Governed writer for partial retention | Preserves the selected source convention or uses the official baseline for the retained task without inventing optional fields. |
| `adopt-arcantry` | Read-only or delegating, out of writer scope | Inspects sources and delegates authorized mutations to the governed CLI. |
| `reconcile-project-sources` | Read-only planner, out of writer scope | Inspects drift and prepares a CLI transition without authoring todo task content. |

The final inventory matches the initial candidate set. No additional canonical skill declares `todo-txt` compatibility.

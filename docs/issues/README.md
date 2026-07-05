# Issues

A running log of known bugs, inconsistencies, and hardening work found in the
codebase. Each file is one issue. These are **not** blockers for the v0.2 editor
core — they're the rough edges surfaced while building it.

| #                                                 | Title                                                              | Severity | Area           |
| ------------------------------------------------- | ------------------------------------------------------------------ | -------- | -------------- |
| [001](001-no-unsaved-changes-guard.md)            | No unsaved-changes guard when switching files / projects           | High     | Editor / data  |
| [002](002-file-io-errors-are-silent.md)           | File I/O failures are swallowed — no error surfaced to the user    | High     | Frontend / IPC |
| [003](003-eps-files-appear-but-cannot-open.md)    | `.eps` files show in the explorer but can't be opened or previewed | Medium   | Explorer       |
| [004](004-tree-walk-no-symlink-or-depth-guard.md) | Project tree walk has no symlink-cycle or depth guard              | Medium   | Backend        |
| [005](005-recents-not-validated.md)               | Recent projects aren't validated — stale entries throw             | Medium   | Homepage       |
| [006](006-dead-greet-scaffolding.md)              | Dead `greet` scaffolding command left from the Tauri template      | Low      | Cleanup        |
| [007](007-search-highlight-offsets-non-ascii.md)  | Search match offsets can be wrong for non-ASCII text               | Low      | Backend        |

## Severity

- **High** — data loss or a silent failure a user will hit in normal use.
- **Medium** — real bug or hardening gap, but narrow or non-destructive.
- **Low** — cleanup / correctness nit with little user impact.

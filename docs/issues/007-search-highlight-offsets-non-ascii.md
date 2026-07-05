# 007 — Search match offsets can be wrong for non-ASCII text

- **Severity:** Low
- **Area:** Backend (project search)
- **Status:** Open

## Summary

`search_project` finds a match by lowercasing the line and calling `str::find`,
then reports the byte offset as `match_start` and `query.len()` as `match_len`.
Rust's `to_lowercase()` does full Unicode case folding, which can change a
string's byte length (and a few characters map to multiple). So for lines
containing non-ASCII text, `match_start` / `match_len` may not line up with the
original `line_text`.

This is currently **latent**: the project-search UI re-locates the match itself
with a case-insensitive `indexOf` on `lineText` for highlighting and only uses
`line` (not the offsets) to jump. But the offsets are part of the command's
returned payload and would mislead any future consumer that trusts them.

## Where

- `src-tauri/src/project.rs` — `search_project()` (the `find` on the lowercased
  line and the `match_start` / `match_len` fields of `SearchMatch`).

## Suggested fix

Either compute the offset against the original (non-lowercased) line via a
case-insensitive scan that preserves byte positions, or drop `match_start` /
`match_len` from the payload and let the frontend own highlight positioning
(which it already does).

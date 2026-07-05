# 005 — Recent projects aren't validated; stale entries throw

- **Severity:** Medium
- **Area:** Homepage / recents
- **Status:** Open

## Summary

`recents` persists project paths to `localStorage` but never checks that they
still exist. If a recent project is moved, renamed, or deleted, opening it calls
`read_project_tree`, which returns `Err("Not a directory…")`. Combined with
[issue 002](002-file-io-errors-are-silent.md), that error is unhandled and the
click appears to do nothing.

## Expected

Dead recents are either hidden, visually marked as unavailable, or removed with a
clear message when a user tries to open one.

## Where

- `src/lib/recents.svelte.ts` — `add()` / `items` store raw paths, no validation.
- `src/lib/project.svelte.ts` — `openProject()` throws on a missing dir.
- Homepage recent-projects UI (consumes `recents.items`).

## Suggested fix

On open failure, catch the error, call `recents.remove(path)`, and show a short
"Project no longer exists" message. Optionally add a Rust `path_exists` command
(or reuse the failed `read_project_tree`) to grey out dead entries on the
homepage proactively.

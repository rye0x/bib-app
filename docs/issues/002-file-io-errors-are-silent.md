# 002 — File I/O failures are swallowed, no error surfaced

- **Severity:** High
- **Area:** Frontend / IPC
- **Status:** Open

## Summary

The Rust file commands return `Result<_, String>`, so any failure arrives on the
JS side as a rejected promise. None of the callers in `project.svelte.ts` catch
it, and the app has no toast/notification surface, so failures are invisible.

The most damaging case is **Save**: if `write_text_file` fails (permissions,
read-only volume, disk full, file deleted underneath), `project.save()` rejects
_before_ `savedContent` is updated. The user sees the unsaved dot stay on and
naturally assumes it just hasn't saved yet — there is no indication the write
actually failed.

## Steps to reproduce

1. Open a file, make it read-only on disk (`chmod 400`), edit it in Bib.
2. Press `⌘S`.
3. Nothing visible happens; the write throws an unhandled rejection and the
   change is never persisted.

## Expected

Surface a clear error ("Couldn't save main.tex: permission denied") and keep the
buffer dirty. Same for failed opens and failed project reads.

## Where

- `src/lib/project.svelte.ts` — `save()`, `openFile()`, `openProject()`,
  `searchProject()` all `await invoke(...)` with no `try/catch`.
- `src-tauri/src/project.rs` — commands correctly return `Err(String)`; the gap
  is purely on the consumer side.

## Suggested fix

Introduce a lightweight notifications store (`*.svelte.ts`) + a toaster
component, and wrap the IPC calls so errors become user-visible messages. This
pairs naturally with the v0.3 "compile log surfacing" work.

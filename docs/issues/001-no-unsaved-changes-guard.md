# 001 — No unsaved-changes guard when switching files / projects

- **Severity:** High
- **Area:** Editor / data integrity
- **Status:** Open

## Summary

Unsaved edits are discarded silently. The dirty state is tracked
(`project.isDirty`) and shown as a dot, but nothing acts on it: opening another
file, opening another project, or closing the project all overwrite `content`
without prompting or auto-saving.

## Steps to reproduce

1. Open a project and a `.tex` file.
2. Type some changes (the unsaved dot appears in the tab strip and tree).
3. Click a different file in the explorer.
4. The edits are gone — reopening the first file shows the last-saved content.

The same happens via **Open Project** (`⌘O`) and would happen on window close.

## Expected

Before discarding a dirty buffer, either prompt (Save / Discard / Cancel) or
auto-save. At minimum the app should not lose work without acknowledgement.

## Where

- `src/lib/project.svelte.ts` — `openFile()`, `openFileAt()`, `openProject()`,
  and `close()` reassign `content` / `savedContent` with no dirty check.
- `src/lib/components/editor/tree-node.svelte` — `onclick` calls
  `project.openFile()` directly.

## Suggested fix

Add a `confirmDiscard()` gate (a small confirm dialog, e.g. shadcn
`alert-dialog`) that `openFile`/`openProject`/`close` await when
`project.isDirty`. Consider also intercepting the Tauri window close event
(`onCloseRequested`) so quitting with unsaved work prompts too.

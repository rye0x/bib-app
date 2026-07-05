# 003 — `.eps` files appear in the explorer but can't be opened or previewed

- **Severity:** Medium
- **Area:** Explorer / source-file filter drift
- **Status:** Open

## Summary

The "what counts as a source file" rule is duplicated in two places that have
drifted apart:

- **Rust** — `SOURCE_EXTS` in `src-tauri/src/project.rs` includes `"eps"`, so
  `.eps` files are surfaced in the explorer tree.
- **TypeScript** — `fileKind()` in `src/lib/project.svelte.ts` has no `eps`
  case, so it returns `"other"`.

Result: an `.eps` file shows up in the tree (with an image icon, since
`tree-node.svelte`'s icon regex _does_ match `eps`), but `openable` is false, so
clicking it does nothing and the row renders muted/disabled. It looks like a file
you should be able to open, but you can't.

## Expected

One source of truth for the file-type policy. Either `.eps` is openable (preview
it, or at least handle it consistently) or it isn't shown at all.

## Where

- `src-tauri/src/project.rs` — `const SOURCE_EXTS` (includes `eps`).
- `src/lib/project.svelte.ts` — `fileKind()` (no `eps`).
- `src/lib/components/editor/tree-node.svelte` — `FileIcon` regex matches `eps`,
  `openable` derives from `fileKind`.

## Suggested fix

Collapse the policy to one definition. Simplest: have `fileKind` treat `eps` as
`"image"` only if the webview can actually render it (it usually can't natively),
otherwise drop `eps` from `SOURCE_EXTS`. Longer term, expose the allowlist from
Rust to the frontend so the two can't diverge.

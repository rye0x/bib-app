# 004 — Project tree walk has no symlink-cycle or depth guard

- **Severity:** Medium
- **Area:** Backend (Tauri / Rust)
- **Status:** Open

## Summary

`build_node()` (and the new `collect_text_files()` used by project search) recurse
into any child directory via `path.is_dir()`, which follows symlinks. There is no
visited-set, depth cap, or `symlink_metadata` check, so:

- A symlink cycle (`a/link -> ..`) causes unbounded recursion and eventually a
  stack overflow / crash.
- A pathologically deep or huge tree is walked fully and synchronously on the
  command's runtime thread, blocking it.

`SKIP_DIRS` mitigates the common heavy dirs (`.git`, `node_modules`) but is an
allowlist of names, not a real cycle guard.

## Steps to reproduce

1. In a project folder: `ln -s .. loop`.
2. Open the folder in Bib → `read_project_tree` recurses through the loop.

## Expected

The walk terminates safely on cycles and doesn't hang the app on large trees.

## Where

- `src-tauri/src/project.rs` — `build_node()` and `collect_text_files()`.

## Suggested fix

Use `entry.file_type()` / `symlink_metadata` to skip symlinked directories (or
track canonicalized visited paths), and add a sane max depth. For large trees,
consider a well-tested walker such as `walkdir` (which handles symlink loops) or
`ignore` (which also respects `.gitignore`).

# 006 — Dead `greet` scaffolding command left from the Tauri template

- **Severity:** Low
- **Area:** Cleanup
- **Status:** Open

## Summary

The `greet` command from the default Tauri starter is still present and
registered, but nothing in the app calls it. It ships in the binary's command
surface and carries a unit test, adding noise to the IPC boundary.

## Where

- `src-tauri/src/lib.rs` — `fn greet(...)`, its entry in
  `tauri::generate_handler![...]`, and the `greet_includes_the_name` test.

## Suggested fix

Remove the `greet` function, drop it from the `generate_handler!` list, and
delete the accompanying test. Keeps the exposed command set limited to what the
app actually uses (`read_project_tree`, `read_text_file`, `write_text_file`,
`search_project`).

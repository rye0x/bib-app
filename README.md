# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Quality & Testing

Git hooks are managed by [lefthook](https://github.com/evilmartians/lefthook) and installed
automatically on `pnpm install`. To install them manually: `pnpm exec lefthook install`.

**On commit** (fast, staged files only): Prettier + `cargo fmt` auto-format and re-stage; ESLint,
`svelte-check`, and Clippy (`-D warnings`) must pass.

**On push**: the Vitest unit suite, `cargo test`, and the Playwright E2E suite must pass.

### Commands

```bash
pnpm lint        # prettier --check + eslint (whole repo)
pnpm format      # prettier --write (whole repo)
pnpm check       # svelte-check + tsc
pnpm test        # vitest (unit) once
pnpm test:unit   # vitest watch
pnpm test:e2e    # playwright e2e

# Rust (run in src-tauri/)
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

### End-to-end tests

E2E runs with **Playwright against the SvelteKit SPA in a browser**, with the Tauri runtime stubbed
(`e2e/tauri-mock.ts`) — `tauri-driver` isn't supported on macOS. First-time setup needs the browser:
`pnpm exec playwright install chromium`.

### Skipping hooks

`LEFTHOOK=0 git commit …` or `git commit --no-verify` (use sparingly).

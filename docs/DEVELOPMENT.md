# Development

Everything you need to build and hack on Bib.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [Node.js](https://nodejs.org) (LTS)
- [pnpm](https://pnpm.io)
- Tauri platform dependencies — see the [Tauri prerequisites guide](https://tauri.app/start/prerequisites/)

## First-time setup

```bash
pnpm install                          # deps + installs lefthook git hooks
pnpm exec playwright install chromium # browser for E2E tests
```

`pnpm install` installs the lefthook git hooks automatically. To (re)install them manually: `pnpm exec lefthook install`.

## Running

```bash
pnpm tauri dev   # run the full desktop app (Rust + frontend)
pnpm dev         # frontend only, in the browser (Vite on :1420)
pnpm tauri build # production desktop build
pnpm build       # frontend production build only
```

## Commands

```bash
pnpm lint        # prettier --check + eslint (whole repo)
pnpm format      # prettier --write (whole repo)
pnpm check       # svelte-kit sync + svelte-check + tsc
pnpm check:watch # svelte-check in watch mode
pnpm test        # vitest (unit) once
pnpm test:unit   # vitest watch
pnpm test:e2e    # playwright e2e

# Rust (run in src-tauri/)
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Git hooks (lefthook)

Hooks are managed by [lefthook](https://github.com/evilmartians/lefthook) and enforce quality automatically.

**On commit** (fast, staged files only): Prettier and `cargo fmt` auto-format and re-stage; then ESLint, `svelte-check`, and Clippy (`-D warnings`) must pass.

**On push**: the Vitest unit suite, `cargo test`, and the Playwright E2E suite must pass.

**Skipping hooks** (use sparingly): `LEFTHOOK=0 git commit …` or `git commit --no-verify`.

## Testing

- **Unit** — [Vitest](https://vitest.dev) with [Testing Library](https://testing-library.com) for Svelte. Existing coverage: theme store, zoom store, `cn` util, and the `action-row` component.
- **End-to-end** — [Playwright](https://playwright.dev) runs against the **SvelteKit SPA in a browser** with the Tauri runtime stubbed (`e2e/tauri-mock.ts`), because `tauri-driver` isn't supported on macOS. Specs live in `e2e/` (`home.spec.ts`, `settings.spec.ts`, `smoke.spec.ts`). First-time setup needs `pnpm exec playwright install chromium`.

## Editor setup

No specific editor is required — use whatever you like. For the best experience, wire up the language servers this stack uses:

- [`svelte-language-server`](https://github.com/sveltejs/language-tools) — Svelte / SvelteKit
- [`rust-analyzer`](https://rust-analyzer.github.io/) — Rust / Tauri backend
- [`typescript-language-server`](https://github.com/typescript-language-server/typescript-language-server) — TypeScript
- [`tailwindcss-language-server`](https://github.com/tailwindlabs/tailwindcss-intellisense) — Tailwind v4 class hints

Also configure Prettier and ESLint in your editor (or rely on `pnpm format` / `pnpm lint`) so formatting matches the project.

## More

- [Architecture](ARCHITECTURE.md) — how the pieces fit together
- [Contributing](CONTRIBUTING.md) — branch, commit, and PR conventions

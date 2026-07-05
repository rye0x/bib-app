# Architecture

How Bib is put together.

## Overview

Bib is a **SvelteKit single-page app rendered inside a native Tauri window**. The frontend is compiled to static assets (no server, no SSR) via `@sveltejs/adapter-static` with `fallback: index.html`. Tauri loads those assets in the OS webview and provides the native shell — window, menus, and a Rust backend — over Tauri's IPC.

```
┌─────────────────────────────────────────────┐
│  Native window (Tauri 2, Rust)               │
│  ┌───────────────────────────────────────┐  │
│  │  OS WebView                            │  │
│  │  ┌─────────────────────────────────┐  │  │
│  │  │  SvelteKit SPA (build/)         │  │  │
│  │  │  Svelte 5 · Tailwind · shadcn   │  │  │
│  │  └─────────────────────────────────┘  │  │
│  └───────────────────────────────────────┘  │
│  Rust core: menus, window API, commands       │
└─────────────────────────────────────────────┘
        ▲                        │
        │  events (menu:zoom)    │  window/opener API
        └────────────────────────┘
```

## Tech stack

| Concern       | Choice                                                               |
| ------------- | -------------------------------------------------------------------- |
| Desktop shell | Tauri 2 (`macos-private-api` feature), tauri-plugin-opener           |
| Backend       | Rust, edition 2021, library crate `bib_app_lib`, serde / serde_json  |
| Framework     | SvelteKit 2 in SPA mode (`adapter-static`, `fallback: index.html`)   |
| Language      | Svelte 5 (runes), TypeScript 6 (strict, `moduleResolution: bundler`) |
| Styling       | Tailwind CSS v4 (CSS-first, via `@tailwindcss/vite`), tw-animate-css |
| Components    | shadcn-svelte v1 (style `luma`, base color `neutral`)                |
| Icons / font  | Remix Icon (`remixicon-svelte`), Inter variable font                 |
| Build         | Vite 8, pnpm                                                         |
| Testing       | Vitest + Testing Library (unit), Playwright (E2E)                    |
| Tooling       | ESLint, Prettier (svelte + tailwind plugins), Clippy, lefthook       |

Product identifier: `com.nathan.bib-app`. Version `0.1.0` across `package.json`, `Cargo.toml`, and `tauri.conf.json`.

## Repository layout

```
bib-app/
├── src/                  # SvelteKit frontend source
│   ├── routes/           # Pages + app shell
│   └── lib/              # Stores, components, utils
├── src-tauri/            # Rust / Tauri backend
│   ├── src/              # lib.rs (menus, commands), main.rs
│   ├── capabilities/     # Permission grants
│   ├── Cargo.toml
│   └── tauri.conf.json
├── e2e/                  # Playwright specs + Tauri mock
├── static/               # Static assets (bib-logo.svg)
├── build/                # Frontend build output (Tauri frontendDist)
├── components.json       # shadcn-svelte config
├── svelte.config.js · vite.config.js · tsconfig.json
└── package.json
```

## Frontend

- **App shell** — `src/routes/+layout.svelte` renders a custom macOS-style titlebar (the window is frameless/transparent): traffic-light close/minimize/maximize buttons wired to the Tauri window API, a draggable region (double-click to toggle maximize), and a settings/back nav button. It initializes theme and zoom on mount and listens for `menu:zoom` events from Rust.
- **Pages** — `+page.svelte` (home: logo, core actions, recent projects placeholder), `settings/+page.svelte` (appearance + editor zoom).
- **State** — rune-based stores in `src/lib`:
  - `theme.svelte.ts` — light/dark, toggles the `.dark` class, persisted to `localStorage` (applied pre-paint via inline script in `app.html`).
  - `zoom.svelte.ts` — UI zoom via CSS `zoom` on the document root (range 0.5–3.0), persisted to `localStorage`.
- **Utilities** — `utils.ts` exports `cn` (clsx + tailwind-merge).
- **Components** — shadcn-svelte primitives under `src/lib/components/ui/` (button, card); feature components like `components/home/action-row.svelte`.

## Backend (Tauri / Rust)

- **Window** — configured in `tauri.conf.json`: 800×600, `decorations: false`, `transparent: true`, `macOSPrivateApi: true`, `csp: null`.
- **Menus** — built in `src-tauri/src/lib.rs`: Bib / Edit / View / Window submenus. The View menu provides Zoom In (`⌘=`), Zoom Out (`⌘-`), Reset Zoom (`⌘0`), Reset All Zoom, and fullscreen.
- **Capabilities** — `src-tauri/capabilities/default.json` grants `core:default`, window `start-dragging` / `minimize` / `toggle-maximize` / `close`, and `opener:default`.
- **Release profile** — `Cargo.toml` enables LTO, `opt-level = 3`, `panic = abort`, and symbol stripping for small, fast binaries.

## Frontend ↔ backend contract

Communication uses Tauri's IPC in two directions:

- **Frontend → native** — the titlebar calls the Tauri window API (drag, minimize, maximize, close).
- **Native → frontend** — native menu actions emit events (e.g. the View menu emits `menu:zoom`), which the layout listens for and applies to the zoom store.

## Dev server

Vite runs on port **1420** (`strictPort`), with HMR on **1421**; `src-tauri/**` is ignored by the watcher. Tauri's `beforeDevCommand` runs `pnpm dev` and points `devUrl` at `http://localhost:1420`; `beforeBuildCommand` runs `pnpm build` and serves from `../build`.

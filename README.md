<div align="center">

<img src="static/bib-logo.svg" alt="Bib logo" width="96" height="96" />

# Bib

**A local-first LaTeX editor built for speed.**

![version](https://img.shields.io/badge/version-0.1.0-blue)
![license](https://img.shields.io/badge/license-MIT-green)
![platforms](https://img.shields.io/badge/platforms-macOS%20%C2%B7%20Linux%20%C2%B7%20Windows-lightgrey)

</div>

---

## What is Bib?

Bib is a native, local-first LaTeX editor for **macOS, Linux, and Windows**. It pairs a minimalist, keyboard-first writing surface with the tools researchers and writers actually need — reusable **templates** and a **universal bibliography** — while keeping every file on your own machine.

It's built with [Tauri](https://tauri.app) and Rust instead of Electron, so it starts fast and stays light on memory.

> **Status:** early development — **v0.1.0**. The app shell, theming, and UI are in place; the LaTeX editor itself is the next milestone. See the [Roadmap](docs/ROADMAP.md).

## Why Bib?

- **Local-first.** Your documents, templates, and bibliography live on your disk. No account, no cloud lock-in, works fully offline.
- **Fast & lightweight.** Native Tauri + Rust core — low RAM, quick startup, no bundled browser engine.
- **Minimalist & keyboard-first.** An interface inspired by **Neovim/Vim** and the **Zed** editor: quiet, focused, out of your way.
- **Cross-platform.** One codebase, native builds for macOS, Linux, and Windows.
- **Built for LaTeX.** Templates and a universal `.bib` store designed around how LaTeX projects really work.

## Features

**Available now (v0.1.0)**

- Custom frameless window with a native macOS-style titlebar
- Light / dark theme
- UI zoom (`⌘+` / `⌘−` / `⌘0`), Zed/VS Code style
- Native menu bar (Bib / Edit / View / Window)
- Homepage and settings screens

**Planned**

- LaTeX editor with syntax highlighting, keyboard-first editing, and LSP-style autocompletion
- Three-pane workspace: project explorer, editor, and compiled PDF preview
- Compile + live PDF preview
- Drag-and-drop component palette (tables, figures, lists, math)
- Template library
- Universal bibliography with citation autocomplete
- _Post-1.0:_ AI-enabled LaTeX editing

See [docs/FEATURES.md](docs/FEATURES.md) for details, and the [Roadmap](docs/ROADMAP.md) for timing.

## Tech stack

| Layer    | Technology                                                         |
| -------- | ------------------------------------------------------------------ |
| Desktop  | Tauri 2 · Rust (edition 2021)                                      |
| Frontend | SvelteKit 2 (SPA) · Svelte 5 (runes) · TypeScript 6                |
| UI       | Tailwind CSS v4 · shadcn-svelte v1 · Remix Icon · Inter            |
| Build    | Vite 8 · pnpm                                                      |
| Quality  | Vitest · Testing Library · Playwright · ESLint · Prettier · Clippy |

Full details in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## Getting started

**Prerequisites**

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [Node.js](https://nodejs.org) (LTS)
- [pnpm](https://pnpm.io)
- Platform dependencies for Tauri — see the [Tauri prerequisites guide](https://tauri.app/start/prerequisites/)

**Run in development**

```bash
pnpm install
pnpm tauri dev
```

**Build a release**

```bash
pnpm tauri build
```

New to the codebase? Start with [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md).

## Documentation

- [Roadmap](docs/ROADMAP.md) — versioned development phases
- [Architecture](docs/ARCHITECTURE.md) — tech stack and how it fits together
- [Development](docs/DEVELOPMENT.md) — setup, commands, tooling, and testing
- [Features](docs/FEATURES.md) — feature-by-feature detail
- [Contributing](docs/CONTRIBUTING.md) — branch, commit, and PR conventions

## License

Bib is released under the MIT License.

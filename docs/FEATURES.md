# Features

A closer look at what Bib does and what's coming. Each item is tagged **Available** (shipping in the current build) or **Planned** (see the [Roadmap](ROADMAP.md) for timing).

---

## Editing

The core writing experience: a minimalist, keyboard-first LaTeX editor inspired by **Neovim/Vim** and the **Zed** editor. Quiet UI, fast keys, no distractions.

- LaTeX editing surface — **Planned** (v0.2)
- Keyboard-first navigation and editing — **Planned** (v0.2)
- LaTeX syntax highlighting — **Planned** (v0.2)
- Open / save `.tex` files and project file tree — **Planned** (v0.2)
- Recent projects on the homepage — **Planned** (v0.2)
- Customizable keybindings and command palette — **Planned** (v0.6)

## Compile & Preview

- LaTeX compilation — **Planned** (v0.3)
- PDF preview pane — **Planned** (v0.3)
- Error and log surfacing — **Planned** (v0.3)
- Live / on-save recompile — **Planned** (v0.3)

## Templates

Reusable starting points so you never begin from a blank file.

- Built-in template library — **Planned** (v0.4)
- "New from template" flow — **Planned** (v0.4)
- Custom, user-defined templates — **Planned** (v0.4)

## Universal Bibliography

One `.bib` store shared across every project — the namesake feature.

- Universal bibliography store — **Planned** (v0.5)
- Citation insertion and autocomplete — **Planned** (v0.5)
- BibTeX / BibLaTeX entry management — **Planned** (v0.5)
- Import from DOI / arXiv — **Planned** (v0.5)

## Local-first & performance

Bib is native (Tauri + Rust), not an Electron app. Your files stay on your machine, it works offline, and it's built to be light on memory with fast startup.

- Native cross-platform desktop app (macOS / Linux / Windows) — **Available** _(foundation)_
- Fully local, offline-capable — **Available** _(foundation)_
- RAM / startup benchmarking and tuning — **Planned** (v0.6)
- Signed installers for all platforms — **Planned** (v1.0)

## Interface

The shell that's already in place.

- Custom frameless window with a native macOS-style titlebar — **Available**
- Light / dark theme with persistence — **Available**
- UI zoom (`⌘+` / `⌘−` / `⌘0`), Zed/VS Code style — **Available**
- Native menu bar (Bib / Edit / View / Window) — **Available**
- Homepage and settings screens — **Available**

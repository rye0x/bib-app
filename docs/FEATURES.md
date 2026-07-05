# Features

A closer look at what Bib does and what's coming. Each item is tagged **Available** (shipping in the current build) or **Planned** (see the [Roadmap](ROADMAP.md) for timing).

---

## Editing

The core writing experience: a minimalist, keyboard-first LaTeX editor inspired by **Neovim/Vim** and the **Zed** editor. Quiet UI, fast keys, no distractions.

- LaTeX editing surface — **Planned** (v0.2)
- Keyboard-first navigation and editing — **Planned** (v0.2)
- LaTeX syntax highlighting — **Planned** (v0.2)
- LaTeX autocompletion — LSP-style completion dropdown for commands, environments, and references — **Planned** (v0.2)
- Open / save `.tex` files — **Planned** (v0.2)
- Recent projects on the homepage — **Planned** (v0.2)
- Customizable keybindings and command palette — **Planned** (v0.6)

## Search & navigation

Move around a LaTeX project as fast as you would in a code editor.

- Find & replace in the current file (`⌘F`) — **Planned** (v0.2)
- Fuzzy file finder — quick-open any file in the project (`⌘P`) — **Planned** (v0.2)
- Project-wide search across all files (`⌘⇧F`) — **Planned** (v0.2)

## Workspace layout

A familiar three-pane layout, like a code editor built for LaTeX.

- **Project explorer** rooted at the project directory — shows only source files (`.tex`, images, `.bib`, and related types); build/auxiliary artifacts created during compilation (`.aux`, `.log`, `.out`, `.toc`, `.synctex.gz`, …) are hidden — **Planned** (v0.2)
- **Editor panel** — the main writing surface — **Planned** (v0.2)
- **Compiled PDF preview**, side by side with the editor — **Planned** (v0.3)

## Components

Insert LaTeX building blocks without writing the boilerplate.

- Drag-and-drop component palette — tables, figures, lists, math environments, and more — **Planned** (v0.4)

## Compile & Preview

- LaTeX compilation — **Planned** (v0.3)
- PDF preview pane — **Planned** (v0.3)
- Compile log surfacing — **Planned** (v0.3)
- Error diagnostics & debugging — explain what each error actually means in plain language and jump to the offending line, instead of dumping a raw log — **Planned** (v0.3)
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

## AI-enabled editing

After the stable 1.0, Bib gains an AI-assisted LaTeX layer — kept local-first and opt-in.

- AI-assisted authoring and rewriting — **Planned** (post-1.0)
- Context-aware LaTeX completion and error fixes — **Planned** (post-1.0)
- Assistance for citations, tables, and figures — **Planned** (post-1.0)

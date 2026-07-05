# Roadmap

> **This is a proposed roadmap.** Phases, features, and version numbers are a starting point and will change as Bib develops. Nothing here is a commitment or a delivery date.

**Guiding principles:** local-first · fast · minimalist · keyboard-first.

Each phase groups related work under a version. Bib follows [semantic versioning](https://semver.org); pre-1.0 releases are considered unstable.

---

## v0.1 — Foundation _(current)_

The shell everything else is built on.

- [x] Tauri 2 + SvelteKit 2 (SPA) + Svelte 5 project scaffold
- [x] Custom frameless window with a native macOS-style titlebar
- [x] Light / dark theme with persistence
- [x] UI zoom (`⌘+` / `⌘−` / `⌘0`) driven by the native View menu
- [x] Native menu bar (Bib / Edit / View / Window)
- [x] Homepage and settings screens
- [x] Dev tooling: lefthook git hooks, Vitest, Playwright, ESLint, Prettier, Clippy

## v0.2 — Editor Core

Turn Bib into an actual editor, laid out like a code editor: project explorer + editor panel.

- [x] LaTeX editing surface
- [x] Minimalist, keyboard-first editing UX (Neovim/Zed-inspired) — keyboard-driven quick-open, project search, and find/replace overlays
- [x] Project explorer rooted at the project directory — shows only source files (`.tex`, images, `.bib`, …); build/auxiliary artifacts are hidden
- [x] Open and save `.tex` files
- [x] LaTeX syntax highlighting
- [x] LaTeX autocompletion — LSP-style completion dropdown (commands, environments, refs, cites)
- [x] Find & replace in the current file (`⌘F`)
- [x] Fuzzy file finder — quick-open across the project (`⌘P`)
- [x] Project-wide search across all files (`⌘⇧F`)
- [x] Recent projects wired to real state (homepage)

## v0.3 — Compile & Preview

See what you're writing. Completes the three-pane workspace: explorer · editor · PDF.

- [ ] LaTeX compilation
- [x] PDF & image preview in the editor panel — open any `.pdf` or image from the explorer (rendered via the Tauri asset protocol)
- [ ] Compiled PDF preview pane, side by side with the editor
- [ ] Compile log surfacing
- [ ] Error diagnostics & debugging — explain what each error actually means and jump to the offending line
- [ ] Live / on-save recompile

## v0.4 — Templates & Components

Start faster and build faster.

- [ ] Built-in template library
- [ ] "New from template" flow
- [ ] Custom, user-defined templates
- [ ] Drag-and-drop component palette — insert tables, figures, lists, math environments, and other LaTeX building blocks without writing the boilerplate

## v0.5 — Universal Bibliography

The "Bib" in Bib.

- [ ] Universal `.bib` store shared across projects
- [ ] Citation insertion and autocomplete
- [ ] BibTeX / BibLaTeX entry management
- [ ] Import from DOI / arXiv

## v0.6 — Performance & Polish

Make it fast, make it yours.

- [ ] RAM and startup benchmarking
- [ ] Customizable keybindings
- [ ] Command palette
- [ ] Cross-platform packaging and code signing

## v1.0 — Stable

- [ ] All core pillars (editor, compile, templates, components, bibliography) stable
- [ ] Documentation complete
- [ ] Signed installers for macOS, Linux, and Windows

## Beyond 1.0 — AI-Enabled Editing

Once the stable release lands, Bib grows an AI-assisted LaTeX layer — kept local-first and opt-in.

- [ ] AI-assisted authoring and rewriting
- [ ] Context-aware LaTeX completion and error fixes
- [ ] Assistance for citations, tables, and figures

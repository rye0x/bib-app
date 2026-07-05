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

Turn Bib into an actual editor.

- [ ] LaTeX editing surface
- [ ] Minimalist, keyboard-first editing UX (Neovim/Zed-inspired)
- [ ] Open and save `.tex` files
- [ ] LaTeX syntax highlighting
- [ ] Project file tree
- [ ] Recent projects wired to real state (homepage)

## v0.3 — Compile & Preview

See what you're writing.

- [ ] LaTeX compilation
- [ ] PDF preview pane
- [ ] Error and log surfacing
- [ ] Live / on-save recompile

## v0.4 — Templates

Start faster.

- [ ] Built-in template library
- [ ] "New from template" flow
- [ ] Custom, user-defined templates

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

- [ ] All core pillars (editor, compile, templates, bibliography) stable
- [ ] Documentation complete
- [ ] Signed installers for macOS, Linux, and Windows

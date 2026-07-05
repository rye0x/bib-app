# Roadmap

> **This is a proposed roadmap.** Phases, features, and version numbers are a starting point and will change as Bib develops. Nothing here is a commitment or a delivery date.

**Guiding principles:** local-first · fast · minimalist · keyboard-first.

Each phase groups related work under a version. Bib follows [semantic versioning](https://semver.org); pre-1.0 releases are considered unstable.

---

## v0.1 — Foundation

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

- [x] LaTeX compilation — `latexmk` (falling back to `pdflatex`), run in-process on the auto-detected main document
- [x] PDF & image preview in the editor panel — open any `.pdf` or image from the explorer (rendered via the Tauri asset protocol)
- [x] Compiled PDF preview pane, side by side with the editor — the third pane of the workspace, kept fresh across rebuilds
- [x] Compile log surfacing — the full build log in a collapsible drawer
- [x] Error diagnostics & debugging — parsed errors/warnings with plain-language hints for common failures; click one to jump to the offending line
- [x] Live / on-save recompile — an "Auto" toggle that rebuilds on every save (⌘B builds on demand)

## v0.4 — Templates & Components _(current)_

Start faster and build faster.

- [ ] Built-in template library
- [ ] Templates gallery page — browse curated LaTeX templates (article, report, beamer, CV, …); picking one scaffolds a new project pre-filled with that template. Wires up the ⌘T "Choose Template" homepage action.
- [ ] "New from template" flow
- [ ] Custom, user-defined templates
- [ ] Drag-and-drop component palette — insert tables, figures, lists, math environments, and other LaTeX building blocks without writing the boilerplate

## v0.5 — Bibliography

The "Bib" in Bib. Two libraries, one workflow: a **universal** library that follows you across every project, and a **project** library scoped to the document you're writing. Both are first-class; the editor reads from whichever the user points it at.

**Two-tier library model**

- [ ] Universal `.bib` store — a global library shared across all projects, living outside any single project directory
- [ ] Project `.bib` store — a per-project library scoped to the current project, tracked alongside its source files
- [ ] Library switcher — pick which library (universal, project, or both merged) the editor cites from and inserts into
- [ ] Promote / copy entries between the two — pull a project entry into the universal library, or drop a universal entry into the current project

**Entry management** (works against whichever library is active)

- [ ] BibTeX / BibLaTeX entry management — add, edit, and delete entries
- [ ] Import from DOI / arXiv, routed to the chosen library
- [ ] Duplicate detection across universal + project on import

**Citing while you write**

- [ ] Citation insertion and autocomplete drawing from the active library set
- [ ] `\cite` autocomplete surfaces entries from both universal and project libraries, labeled by source

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

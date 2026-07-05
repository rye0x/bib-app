# Contributing

Thanks for helping build Bib. This guide covers how we branch, commit, and ship changes. For setup and commands, see [DEVELOPMENT.md](DEVELOPMENT.md).

## Before you start

- Read [ARCHITECTURE.md](ARCHITECTURE.md) to understand the layout.
- Check the [Roadmap](ROADMAP.md) so your work lines up with the current phase.

## Branches

Use Git-Flow-style, purpose-prefixed branch names:

- `feature/<short-name>` — new functionality
- `fix/<short-name>` — bug fixes
- `chore/<short-name>` — tooling, deps, docs, maintenance

Branch off `main`.

## Commits

Follow [Conventional Commits](https://www.conventionalcommits.org):

```
<type>(<scope>): <summary>
```

Common types: `feat`, `fix`, `chore`, `docs`, `test`, `refactor`, `perf`, `build`. Keep the summary imperative and concise.

Examples:

```
feat(editor): add LaTeX syntax highlighting
fix(zoom): clamp zoom to configured range
docs(readme): rewrite for general audience
```

## Pull requests

1. Branch from `main`, make your change, and push.
2. Ensure all git hooks pass — commit hooks run Prettier/`cargo fmt`, ESLint, `svelte-check`, and Clippy; push hooks run Vitest, `cargo test`, and Playwright (see [DEVELOPMENT.md](DEVELOPMENT.md#git-hooks-lefthook)).
3. Open a PR against `main` with a clear description of what and why.
4. Address review feedback; keep the branch up to date.

## Code style

- **Pragmatic and minimalist.** Write concise, direct code. No over-engineering, no unnecessary comments — let the code and its tests speak.
- **Svelte 5 runes.** Use runes (`$state`, `$derived`, `$effect`) and the project's existing store patterns (`src/lib/*.svelte.ts`).
- **Match the surroundings.** Follow the naming, structure, and idioms already in the file you're editing.
- **Let tooling decide formatting.** Prettier and ESLint are the source of truth — run `pnpm format` and `pnpm lint`.

## Tests

Add or update tests for behavior you change. Unit tests live next to the code (`*.test.ts`); E2E specs live in `e2e/`.

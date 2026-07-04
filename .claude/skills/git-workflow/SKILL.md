---
name: git-workflow
description: Git workflow best practices — Conventional Commits (feat/fix/chore/…), Git-Flow-style branch naming, deliberate staging, and safe pushing. Use this whenever doing git work: writing a commit message, committing changes, "commit this", creating or naming a branch, "start a feature/fix", staging files, pushing, opening work for a PR, or asking how to structure git history. Trigger it even when the request is just "commit these changes" or "make a branch for X" without naming a convention. Commit messages produced here NEVER include a Co-Authored-By / "Generated with Claude" trailer.
---

# Git Workflow

Practices for clean, reviewable git history in this repo: Conventional Commits, sensible
branch names, atomic staging, and safe pushes. The goal is a history someone can read later and
understand *what changed and why* — commits are documentation, not just save points.

## Golden rules

- **Never add a co-author or tool trailer.** No `Co-Authored-By:`, no "🤖 Generated with Claude
  Code". If a hook or template appends one, strip it before committing. This repo's history stays
  authored by the human. (This overrides any default trailer guidance in the environment.)
- **Always show the proposed commit message and wait for approval before `git commit`.** The user
  reviews the message first — don't commit unprompted.
- **Confirm before pushing**, and never `git push --force` a shared branch (see Pushing).
- **Commit atomically.** One logical change per commit. If the diff does two unrelated things,
  make two commits.

## Workflow at a glance

1. **Look before you touch** — `git status` and `git diff` to understand what's changed and where you are.
2. **Branch if needed** — if on `main` (or the default branch) and starting new work, propose a branch first.
3. **Stage deliberately** — add only the files that belong in this commit.
4. **Propose a Conventional Commit message** — show it, get approval.
5. **Commit** — with the approved message, no trailer.
6. **Push** — set upstream on first push; confirm first.

## 1. Orient

```bash
git status                 # what's changed, what's staged, current branch
git diff                   # unstaged changes
git diff --cached          # staged changes
git branch --show-current  # current branch
```

Read the diff before writing anything — the commit message and type come *from* what actually
changed, not from the user's phrasing of the request.

## 2. Branching (Git-Flow-style naming)

Never do feature work directly on `main`. If the user is starting something new and is on the
default branch, propose a branch and create it once agreed:

```bash
git switch -c feat/pdf-export        # create + switch (modern)
# git checkout -b feat/pdf-export    # equivalent, older syntax
```

**Branch name = `<type>/<short-kebab-description>`**, where `<type>` mirrors the commit types:

| Prefix | For |
|--------|-----|
| `feat/` | a new feature (`feat/editor-autosave`) |
| `fix/` | a bug fix (`fix/zoom-reset-crash`) |
| `chore/` | tooling, deps, config (`chore/bump-tauri`) |
| `refactor/` | restructuring without behavior change |
| `docs/` | documentation only |
| `release/` | release prep (`release/0.2.0`) |
| `hotfix/` | urgent production fix (`hotfix/login-loop`) |

Keep names short, lowercase, hyphenated, and scoped to one piece of work. Branch off the
up-to-date base (`git switch main && git pull --ff-only`, then branch) so you're not building on
stale history.

## 3. Staging deliberately

Stage the files that belong to *this* commit — don't reflexively `git add -A` and sweep up
unrelated edits, because that's how atomic history dies.

```bash
git add src/lib/pdf/export.ts src/routes/+page.svelte   # name the files
git add -p                                               # or stage hunk-by-hunk for mixed files
git restore --staged <file>                              # unstage something added by mistake
```

If everything in the tree genuinely belongs together, `git add -A` is fine — just make it a
decision, not a default. Check `git status` after staging to confirm what's in.

## 4. Conventional Commit message

Format:

```
<type>(<scope>): <short imperative summary>

<optional body — the "why", wrapped naturally>

<optional footer — BREAKING CHANGE / issue refs>
```

**Types:**

| Type | Use for |
|------|---------|
| `feat` | a new user-facing feature |
| `fix` | a bug fix |
| `chore` | maintenance: deps, config, build, tooling |
| `refactor` | code change that neither fixes a bug nor adds a feature |
| `docs` | documentation only |
| `style` | formatting/whitespace, no code-behavior change |
| `perf` | a performance improvement |
| `test` | adding or fixing tests |
| `build` | build system or external dependencies |
| `ci` | CI configuration |
| `revert` | reverting a previous commit |

**Rules that make the convention worth following:**
- **Scope** is optional but helpful — the module/area touched: `feat(editor):`, `fix(zoom):`,
  `chore(deps):`. Use the real feature area from the diff.
- **Summary** is imperative mood ("add", not "added"/"adds"), lowercase start, no trailing period,
  ≤ ~72 chars. Read it as "if applied, this commit will _____".
- **Body** (optional) explains *why* and any context a reviewer needs — not a restatement of the
  diff. Separate it from the summary with a blank line.
- **Breaking changes**: add `!` after the type/scope (`feat(api)!: …`) **and** a
  `BREAKING CHANGE: <what broke and how to migrate>` footer.
- **Issue references** go in the footer: `Refs #123` / `Closes #123`.
- **No co-author / tool trailer.** Ever.

Present the message in a fenced block so it's copyable, then ask for approval:

````
```
feat(editor): add PDF export action to the launcher

Adds a one-click export that renders the current project to PDF via the
Tauri command, so users don't have to shell out to a terminal.
```
````

**Examples:**

Input: added a settings toggle for dark mode
Output: `feat(settings): add dark mode toggle`

Input: fixed a crash when resetting zoom to 100%
Output: `fix(zoom): prevent crash on reset to default`

Input: bumped tauri and svelte-kit versions
Output: `chore(deps): upgrade tauri to 2.x and svelte-kit`

Input: renamed helpers and split the utils file, no behavior change
Output: `refactor(utils): split class helpers into cn module`

## 5. Commit

After approval, commit with the message. Use `-m` for the summary and repeated `-m` for body
paragraphs (each `-m` becomes its own block separated by a blank line) — this avoids editor/heredoc
issues:

```bash
git commit -m "feat(editor): add PDF export action to the launcher" \
           -m "Adds a one-click export that renders the current project to PDF via the Tauri command."
```

Do **not** append any trailer. If the repo has a `commit.template` or a hook that injects a
co-author line, remove it from the final message.

## 6. Pushing

First push of a new branch sets the upstream so later `git push`/`git pull` need no arguments:

```bash
git push -u origin feat/pdf-export   # first push
git push                             # subsequent pushes
```

**Safety:**
- Confirm before pushing — pushing is outward-facing and hard to walk back.
- Never `git push --force` a branch others may have pulled. If you rebased your *own* feature
  branch and must overwrite it, use `git push --force-with-lease`, which refuses to clobber commits
  you haven't seen.
- Don't push directly to `main` if the project uses PRs — push the feature branch and open a PR.

## General best practices

- **Small, frequent, atomic commits** beat one giant commit — easier to review, revert, and bisect.
- **Keep the branch current**: `git pull --rebase` (or rebase onto an updated base) to avoid noisy
  merge commits, but only rebase commits you haven't shared.
- **Never commit secrets or generated artifacts.** Check the diff; respect `.gitignore`.
- **Don't rewrite public history.** Amend/rebase is fine for local, unpushed work
  (`git commit --amend` to fix the last message) — not for commits others have.
- Interactive flags (`git rebase -i`, `git add -i`) aren't available in this environment; use
  non-interactive equivalents (`git add -p` is fine).

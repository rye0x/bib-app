---
name: senior-frontend
description: Senior frontend workflow for this repo's stack — Svelte 5 (runes), SvelteKit 2 in SPA mode, Tauri 2 desktop, shadcn-svelte v1 components, Tailwind v4 (CSS-first), remixicon-svelte icons, and pnpm. Use this whenever building or editing UI here: creating/editing .svelte components, adding shadcn-svelte components, wiring reactive state, styling with Tailwind tokens, working with routes/layouts, or reviewing frontend code. Trigger it even when the request just says "add a button/dialog/page", "make this reactive", "fix the styling", or "build the X view" without naming the framework — this project is Svelte, not React.
---

# Senior Frontend (Svelte 5 · SvelteKit · Tauri · shadcn-svelte · Tailwind v4)

This is a **Tauri 2 desktop app** with a **SvelteKit 2** frontend running in **SPA mode**
(`adapter-static`, `ssr = false`). Everything is client-side — there is no Node server at
runtime. UI is built with **Svelte 5 runes**, **shadcn-svelte v1** components, **Tailwind v4**
(CSS-first config), **remixicon-svelte** icons, and **pnpm** as the package manager.

Match the existing code. Before writing anything, read a neighbouring file in the same folder
and mirror its imports, prop style, and formatting. The patterns below are the house style,
distilled from the real source.

## The non-negotiables (why they matter)

- **Runes, not the old reactivity.** Use `$state`, `$derived`, `$props`, `$bindable`, `$effect`.
  Never `export let`, `$:`, or stores (`writable`) in new code — this project is on Svelte 5 and
  mixing paradigms makes reactivity hard to reason about. See `references/svelte5.md`.
- **`.svelte.ts` for shared reactive state.** Cross-component state lives in a `*.svelte.ts`
  module that owns a `$state` and exports an object of getters/setters (see
  `src/lib/theme.svelte.ts`, `src/lib/zoom.svelte.ts`). This is the project's answer to stores.
- **Import local `.ts` with a `.js` extension.** e.g. `import { cn } from "$lib/utils.js"`. This
  is how shadcn-svelte and the existing code resolve modules — keep it consistent.
- **Don't reinvent shadcn primitives.** Reuse `$lib/components/ui/*`. Add new ones with the CLI
  (`pnpm dlx shadcn-svelte@latest add <name>`), never hand-roll them. See `references/shadcn-svelte.md`.
- **Style with theme tokens, never raw colors.** Use `bg-primary`, `text-muted-foreground`,
  `border-border`, etc. — never `bg-[#…]` or `text-gray-500`. Tokens flip for dark mode
  automatically. See `references/tailwind-v4.md`.
- **SPA constraints are real.** No SSR, no server `load`, no form actions, no `+server.ts`
  endpoints. Data comes from Tauri commands/events. See `references/sveltekit-tauri.md`.
- **Icons:** `import RiFileAddLine from "remixicon-svelte/icons/file-add-line"` — per-icon default
  imports, rendered as components (`<RiFileAddLine />`).
- **pnpm only.** `pnpm install`, `pnpm dlx …`. No `npm`/`yarn` — the lockfile is `pnpm-lock.yaml`.

## Reference files — read the one that fits the task

| Task | Read |
|------|------|
| Component logic, props, reactivity, effects, events, snippets | `references/svelte5.md` |
| Using / adding shadcn-svelte components, `cn` + `tailwind-variants` | `references/shadcn-svelte.md` |
| Routing, layouts, SPA mode, Tauri commands/events | `references/sveltekit-tauri.md` |
| Colors, dark mode, theme tokens, CSS-first Tailwind | `references/tailwind-v4.md` |

## Component skeleton (the shape almost every file takes)

```svelte
<script lang="ts">
  import { cn } from "$lib/utils.js";

  type Props = {
    label: string;
    count?: number;
    class?: string;
    onselect?: (id: string) => void;
  };

  let { label, count = 0, class: className, onselect }: Props = $props();

  let open = $state(false);
  const summary = $derived(`${label} (${count})`);
</script>

<button
  type="button"
  onclick={() => onselect?.(label)}
  class={cn("rounded-lg px-3 py-2 text-sm hover:bg-accent", className)}
>
  {summary}
</button>
```

Key points visible here and enforced throughout the repo:
- Props are one `type Props = {…}` destructured out of `$props()`; optional props get defaults.
- `class` is renamed to `className` on destructure and merged via `cn(...)` so callers can extend styles.
- Event callbacks are **props** (`onselect`, `onclick`) called optionally (`?.()`), not
  `createEventDispatcher`.
- Children use snippets: `let { children } = $props()` then `{@render children?.()}` — not `<slot>`.

## Workflow

1. **Locate the pattern first.** Find the closest existing component/route and read it. This repo
   is small and consistent; copying its shape is faster and safer than inventing.
2. **Reuse before building.** Check `$lib/components/ui` (shadcn) and `$lib/components/*` (app
   components) before writing new markup. Need a shadcn primitive that isn't installed? Add it with
   the CLI.
3. **Type everything.** `lang="ts"` always. Type props, callbacks, and reactive-state modules.
4. **Verify.** Run `pnpm check` (svelte-check + tsc) after changes. For runtime, `pnpm tauri dev`
   runs the desktop app; `pnpm dev` runs the frontend alone in a browser.

## Commands

```bash
pnpm install                                  # deps
pnpm dev                                       # frontend only (browser, Vite)
pnpm tauri dev                                 # full desktop app
pnpm check                                     # svelte-check + typecheck — run before finishing
pnpm build                                     # production build
pnpm dlx shadcn-svelte@latest add <component>  # add a shadcn-svelte component
```

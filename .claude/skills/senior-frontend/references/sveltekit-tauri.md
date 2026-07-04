# SvelteKit 2 (SPA mode) + Tauri 2

This app is a Tauri desktop shell around a SvelteKit frontend. Because Tauri has no Node server,
SvelteKit runs as a **single-page app**: `@sveltejs/adapter-static` with `fallback: "index.html"`
and `export const ssr = false` in the root `+layout.ts`. Everything runs in the browser/webview.

## What SPA mode means for you

- **No SSR / no prerendering of dynamic data.** Components render client-side only. Guard any code
  that touches `window`, `document`, `localStorage` — it's fine at runtime, but don't assume a
  server pass. (`onMount` only fires in the browser, so it's the safe place for DOM/storage reads —
  that's where `initTheme()`/`initZoom()` are called.)
- **No server `load`, no form actions, no `+server.ts` endpoints, no `+page.server.ts`.** Those
  need a server this app doesn't have. Fetch data from **Tauri commands** instead (see below).
- **Client `load` in `+page.ts`/`+layout.ts` is allowed** for client-only data prep, but most data
  flows through Tauri.

## Routing

File-based routing under `src/routes/`:
- `+page.svelte` — a page. `+layout.svelte` — wraps child pages, renders `{@render children()}`.
- Nested dir = nested route: `src/routes/settings/+page.svelte` → `/settings`.
- `+layout.ts` / `+page.ts` — universal `load` (runs client-side here).

Navigation and route state use the runes-friendly modules:

```svelte
<script lang="ts">
  import { page } from "$app/state";        // reactive: page.url, page.params, …
  import { goto } from "$app/navigation";

  const onSettings = $derived(page.url.pathname.startsWith("/settings"));
</script>

<button onclick={() => goto("/settings")}>Settings</button>
```

Use `$app/state` (`page`), **not** the deprecated `$app/stores`. `page.url.pathname` etc. are
reactive and compose directly with `$derived`.

## Layout pattern

The root `+layout.svelte` imports `../app.css`, takes `let { children } = $props()`, initializes
shared state in `onMount` (`initTheme`, `initZoom`), and renders `{@render children()}`. Add
app-wide chrome (title bar, nav) here.

## Tauri integration

Talk to the Rust side through `@tauri-apps/api`. Two main channels:

**Commands** — call Rust functions (this is your "backend"/data layer):

```ts
import { invoke } from "@tauri-apps/api/core";
const items = await invoke<Item[]>("list_items", { folder });
```

**Events** — subscribe to messages pushed from Rust; always store and call the returned unlisten in
cleanup:

```ts
import { listen } from "@tauri-apps/api/event";
import { onMount } from "svelte";

onMount(() => {
  const unlisten = listen<string>("menu:zoom", (e) => { /* handle e.payload */ });
  return () => { unlisten.then((off) => off()); };
});
```

The layout already uses this pattern to receive `menu:zoom` from the native View menu and drive the
`zoom` state module. Native window control is via `getCurrentWindow()` from
`@tauri-apps/api/window`.

The Rust side lives in `src-tauri/`. When a feature needs new data or OS access, it's usually a new
Tauri command there plus an `invoke` call here — check `src-tauri/src` for existing commands before
assuming something must be done in JS.

## Verifying

- `pnpm check` — svelte-check + tsc. Run before finishing any change.
- `pnpm dev` — frontend only in a browser (fast for pure-UI work; Tauri APIs are unavailable).
- `pnpm tauri dev` — the real desktop app, with Tauri commands/events live.

<script lang="ts">
  import "../app.css";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import RiSettings3Line from "remixicon-svelte/icons/settings-3-line";
  import RiArrowLeftLine from "remixicon-svelte/icons/arrow-left-line";
  import { initTheme } from "$lib/theme.svelte";
  import { initZoom, zoom } from "$lib/zoom.svelte";

  let { children } = $props();

  const appWindow = getCurrentWindow();

  const onSettings = $derived(page.url.pathname.startsWith("/settings"));

  onMount(() => {
    initTheme();
    initZoom();

    // Zoom is driven by the native View menu (⌘=, ⌘-, ⌘0). The Rust side
    // owns the accelerators and forwards the action here.
    const unlisten = listen<string>("menu:zoom", (e) => {
      switch (e.payload) {
        case "in":
          zoom.in();
          break;
        case "out":
          zoom.out();
          break;
        case "reset":
        case "reset-all":
          zoom.reset();
          break;
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  });

  // Double-clicking the empty part of the titlebar zooms/restores the window,
  // matching native macOS behaviour (the buttons are children, so a click on
  // them has a different target and won't trigger this).
  function onTitlebarDblClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      appWindow.toggleMaximize();
    }
  }
</script>

<div class="app-root">
<!-- Custom titlebar: draggable strip + window controls -->
<div
  data-tauri-drag-region
  class="titlebar"
  role="toolbar"
  tabindex="-1"
  ondblclick={onTitlebarDblClick}
>
  <div class="controls">
    <button class="dot close" aria-label="Close" onclick={() => appWindow.close()}>
      <svg class="glyph" viewBox="0 0 12 12">
        <path d="M3.6 3.6 L8.4 8.4 M8.4 3.6 L3.6 8.4" />
      </svg>
    </button>
    <button class="dot min" aria-label="Minimize" onclick={() => appWindow.minimize()}>
      <svg class="glyph" viewBox="0 0 12 12">
        <path d="M3 6 L9 6" />
      </svg>
    </button>
    <button class="dot max" aria-label="Maximize" onclick={() => appWindow.toggleMaximize()}>
      <svg class="glyph" viewBox="0 0 12 12">
        <path d="M3.4 3.4 L3.4 6.6 L6.6 3.4 Z" fill="currentColor" stroke="none" />
        <path d="M8.6 8.6 L8.6 5.4 L5.4 8.6 Z" fill="currentColor" stroke="none" />
      </svg>
    </button>
  </div>

  {#if onSettings}
    <button
      class="nav-btn"
      aria-label="Back"
      title="Back"
      onclick={() => goto("/")}
    >
      <RiArrowLeftLine width={16} height={16} />
    </button>
  {:else}
    <button
      class="nav-btn"
      aria-label="Settings"
      title="Settings"
      onclick={() => goto("/settings")}
    >
      <RiSettings3Line width={16} height={16} />
    </button>
  {/if}
</div>

{@render children()}
</div>

<style>
  /* Rounded window surface. The Tauri window is transparent, so this radius
     is what gives us native-style macOS rounded corners. */
  .app-root {
    min-height: 100vh;
    border-radius: 12px;
    overflow: hidden;
    background: var(--background);
  }

  .titlebar {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: 32px;
    z-index: 9999;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
  }

  .controls {
    display: flex;
    gap: 8px;
  }

  /* clicking a button targets the button (no drag attr), so the drag
     region only fires on the empty strip */
  .dot {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: none;
    padding: 0;
    cursor: default;
  }
  .close {
    background: #ff5f57;
  }
  .min {
    background: #febc2e;
  }
  .max {
    background: #28c840;
  }

  /* glyphs are hidden until the traffic-light group is hovered (native macOS) */
  .glyph {
    width: 12px;
    height: 12px;
    stroke: rgba(0, 0, 0, 0.55);
    color: rgba(0, 0, 0, 0.55);
    stroke-width: 1.4;
    stroke-linecap: round;
    fill: none;
    opacity: 0;
    transition: opacity 0.12s ease;
  }
  .controls:hover .glyph {
    opacity: 1;
  }

  .nav-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    border: none;
    border-radius: 6px;
    background: transparent;
    color: var(--muted-foreground);
    cursor: default;
    transition:
      background 0.15s ease,
      color 0.15s ease;
  }
  .nav-btn:hover {
    background: var(--accent);
    color: var(--accent-foreground);
  }
</style>

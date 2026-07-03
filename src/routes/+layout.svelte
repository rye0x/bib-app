<script lang="ts">
  import "../app.css";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { onMount } from "svelte";

  let { children } = $props();

  const appWindow = getCurrentWindow();

  let isDark = $state(false);

  onMount(() => {
    isDark = document.documentElement.classList.contains("dark");
  });

  function toggleTheme() {
    isDark = !isDark;
    document.documentElement.classList.toggle("dark", isDark);
    try {
      localStorage.setItem("theme", isDark ? "dark" : "light");
    } catch (e) {}
  }

  // Double-clicking the empty part of the titlebar zooms/restores the window,
  // matching native macOS behaviour (the buttons are children, so a click on
  // them has a different target and won't trigger this).
  function onTitlebarDblClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      appWindow.toggleMaximize();
    }
  }
</script>

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

  <button
    class="theme-toggle"
    aria-label="Toggle theme"
    title="Toggle light / dark"
    onclick={toggleTheme}
  >
    {#if isDark}
      <!-- sun -->
      <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="4" />
        <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" />
      </svg>
    {:else}
      <!-- moon -->
      <svg viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
      </svg>
    {/if}
  </button>
</div>

{@render children()}

<style>
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

  .theme-toggle {
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
  .theme-toggle:hover {
    background: var(--accent);
    color: var(--accent-foreground);
  }
</style>

<script lang="ts">
  import { project, type SearchMatch } from "$lib/project.svelte.js";
  import RiSearchLine from "remixicon-svelte/icons/search-line";

  type Props = { open: boolean };
  let { open = $bindable() }: Props = $props();

  let query = $state("");
  let input = $state<HTMLInputElement>();
  let matches = $state<SearchMatch[]>([]);
  let searching = $state(false);
  let selected = $state(0);
  let listEl = $state<HTMLDivElement>();

  // Debounce the search so we don't hit the backend on every keystroke.
  let timer: ReturnType<typeof setTimeout> | undefined;

  $effect(() => {
    if (open) {
      query = "";
      matches = [];
      selected = 0;
      input?.focus();
    }
  });

  // Re-run the (debounced) search whenever the query changes while open.
  $effect(() => {
    const q = query;
    if (!open) return;
    clearTimeout(timer);
    if (q.trim() === "") {
      matches = [];
      searching = false;
      return;
    }
    searching = true;
    timer = setTimeout(async () => {
      try {
        matches = await project.searchProject(q);
      } catch {
        matches = [];
      } finally {
        searching = false;
        selected = 0;
      }
    }, 180);
  });

  $effect(() => {
    void selected;
    listEl
      ?.querySelector<HTMLElement>('[data-selected="true"]')
      ?.scrollIntoView({ block: "nearest" });
  });

  function choose(hit: SearchMatch) {
    project.openFileAt(hit.path, hit.line);
    open = false;
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      selected = Math.min(selected + 1, matches.length - 1);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selected = Math.max(selected - 1, 0);
    } else if (e.key === "Enter") {
      e.preventDefault();
      const hit = matches[selected];
      if (hit) choose(hit);
    } else if (e.key === "Escape") {
      e.preventDefault();
      open = false;
    }
  }

  // Split a preview line into before / match / after around the hit. We
  // re-locate the match in the frontend so highlighting is robust regardless of
  // byte vs. UTF-16 offset differences.
  function parts(hit: SearchMatch, q: string) {
    const idx = hit.lineText.toLowerCase().indexOf(q.trim().toLowerCase());
    if (idx < 0) return { pre: hit.lineText, hit: "", post: "" };
    return {
      pre: hit.lineText.slice(0, idx),
      hit: hit.lineText.slice(idx, idx + q.trim().length),
      post: hit.lineText.slice(idx + q.trim().length),
    };
  }
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex justify-center bg-black/30 pt-[12vh]"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) open = false;
    }}
  >
    <div
      class="flex max-h-[64vh] w-full max-w-[640px] flex-col overflow-hidden rounded-xl border border-border bg-popover text-popover-foreground shadow-2xl"
      role="dialog"
      aria-label="Search in project"
    >
      <div class="flex items-center gap-2 border-b border-border px-4">
        <RiSearchLine width={15} height={15} class="shrink-0 text-muted-foreground" />
        <input
          bind:this={input}
          bind:value={query}
          {onkeydown}
          type="text"
          placeholder="Search across all files…"
          class="w-full bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground"
        />
      </div>

      <div bind:this={listEl} class="min-h-0 flex-1 overflow-y-auto p-1">
        {#if searching}
          <p class="px-3 py-6 text-center text-sm text-muted-foreground">Searching…</p>
        {:else if query.trim() === ""}
          <p class="px-3 py-6 text-center text-sm text-muted-foreground">
            Type to search every source file
          </p>
        {:else if matches.length === 0}
          <p class="px-3 py-6 text-center text-sm text-muted-foreground">No matches</p>
        {:else}
          {#each matches as hit, i (hit.path + ":" + hit.line + ":" + hit.matchStart)}
            {@const p = parts(hit, query)}
            <button
              type="button"
              data-selected={i === selected}
              onclick={() => choose(hit)}
              onmousemove={() => (selected = i)}
              class="flex w-full items-baseline gap-2 rounded-md px-3 py-1.5 text-left text-sm outline-none data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground"
            >
              <span class="shrink-0 font-mono text-xs text-muted-foreground">
                {hit.relPath}:{hit.line}
              </span>
              <span class="min-w-0 flex-1 truncate font-mono text-xs">
                <span class="text-muted-foreground">{p.pre}</span><span
                  class="rounded-sm bg-primary/25 font-semibold text-foreground">{p.hit}</span
                ><span class="text-muted-foreground">{p.post}</span>
              </span>
            </button>
          {/each}
        {/if}
      </div>
    </div>
  </div>
{/if}

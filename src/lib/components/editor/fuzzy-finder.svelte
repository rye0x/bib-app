<script lang="ts">
  import { project, type ProjectFile } from "$lib/project.svelte.js";
  import { fuzzyMatch } from "$lib/fuzzy.js";
  import RiFileTextLine from "remixicon-svelte/icons/file-text-line";

  type Props = { open: boolean };
  let { open = $bindable() }: Props = $props();

  let query = $state("");
  let selected = $state(0);
  let input = $state<HTMLInputElement>();
  let listEl = $state<HTMLDivElement>();

  const MAX = 50;

  type Ranked = { file: ProjectFile; positions: number[] };

  // Rank the project's files against the query. Empty query lists everything
  // in tree order (capped), so ⌘P is useful before you type anything.
  const results = $derived.by<Ranked[]>(() => {
    const files = project.files;
    if (query.trim() === "") {
      return files.slice(0, MAX).map((file) => ({ file, positions: [] }));
    }
    return files
      .map((file) => {
        const m = fuzzyMatch(query, file.relPath);
        return m ? { file, positions: m.positions, score: m.score } : null;
      })
      .filter((r): r is Ranked & { score: number } => r !== null)
      .sort((a, b) => b.score - a.score)
      .slice(0, MAX);
  });

  // Reset each time the palette opens; keep the selection in range as results
  // shrink while typing.
  $effect(() => {
    if (open) {
      query = "";
      selected = 0;
      input?.focus();
    }
  });
  $effect(() => {
    void results;
    if (selected >= results.length) selected = Math.max(0, results.length - 1);
  });

  // Keep the highlighted row scrolled into view.
  $effect(() => {
    void selected;
    listEl
      ?.querySelector<HTMLElement>('[data-selected="true"]')
      ?.scrollIntoView({ block: "nearest" });
  });

  function choose(file: ProjectFile) {
    project.openFile(file.path);
    open = false;
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      selected = Math.min(selected + 1, results.length - 1);
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      selected = Math.max(selected - 1, 0);
    } else if (e.key === "Enter") {
      e.preventDefault();
      const hit = results[selected];
      if (hit) choose(hit.file);
    } else if (e.key === "Escape") {
      e.preventDefault();
      open = false;
    }
  }

  // Split a path into highlighted / plain segments from matched indices.
  function segments(text: string, positions: number[]) {
    const set = new Set(positions);
    const out: { ch: string; hit: boolean }[] = [];
    for (let i = 0; i < text.length; i++) out.push({ ch: text[i], hit: set.has(i) });
    return out;
  }
</script>

{#if open}
  <!-- Backdrop: click to dismiss. -->
  <div
    class="fixed inset-0 z-50 flex justify-center bg-black/30 pt-[12vh]"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) open = false;
    }}
  >
    <div
      class="flex max-h-[60vh] w-full max-w-[560px] flex-col overflow-hidden rounded-xl border border-border bg-popover text-popover-foreground shadow-2xl"
      role="dialog"
      aria-label="Quick open"
    >
      <input
        bind:this={input}
        bind:value={query}
        {onkeydown}
        type="text"
        placeholder="Search files by name…"
        class="w-full border-b border-border bg-transparent px-4 py-3 text-sm outline-none placeholder:text-muted-foreground"
      />

      <div bind:this={listEl} class="min-h-0 flex-1 overflow-y-auto p-1">
        {#if results.length === 0}
          <p class="px-3 py-6 text-center text-sm text-muted-foreground">No matching files</p>
        {:else}
          {#each results as { file, positions }, i (file.path)}
            <button
              type="button"
              data-selected={i === selected}
              onclick={() => choose(file)}
              onmousemove={() => (selected = i)}
              class="flex w-full items-center gap-2 rounded-md px-3 py-1.5 text-left text-sm outline-none data-[selected=true]:bg-accent data-[selected=true]:text-accent-foreground"
            >
              <RiFileTextLine width={14} height={14} class="shrink-0 text-muted-foreground" />
              <span class="min-w-0 flex-1 truncate">
                {#each segments(file.relPath, positions) as seg, i (i)}<span
                    class={seg.hit ? "font-semibold text-foreground" : "text-muted-foreground"}
                    >{seg.ch}</span
                  >{/each}
              </span>
            </button>
          {/each}
        {/if}
      </div>
    </div>
  </div>
{/if}

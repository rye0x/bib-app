<script lang="ts">
  import type { Component } from "svelte";
  import type { SVGAttributes } from "svelte/elements";
  import RiFileAddLine from "remixicon-svelte/icons/file-add-line";
  import RiFolderOpenLine from "remixicon-svelte/icons/folder-open-line";
  import RiLayoutGridLine from "remixicon-svelte/icons/layout-grid-line";

  import { ActionRow } from "$lib/components/home";

  type Action = {
    icon: Component<SVGAttributes<SVGSVGElement>>;
    label: string;
    shortcut?: string;
    onclick?: () => void;
  };

  const actions: Action[] = [
    { icon: RiFileAddLine, label: "New LaTeX Project", shortcut: "⌘N" },
    { icon: RiFolderOpenLine, label: "Open LaTeX Project", shortcut: "⌘O" },
    { icon: RiLayoutGridLine, label: "Choose Template", shortcut: "⌘T" },
  ];

  // Placeholder recent projects — wire up to real state later.
  const recents: { name: string; path: string }[] = [];
</script>

<main
  class="flex min-h-screen flex-col items-center justify-center gap-10 bg-background px-6 pt-[72px] pb-12 text-foreground"
>
  <!-- Brand -->
  <header class="flex flex-col items-center gap-4">
    <img src="/bib-logo.svg" alt="Bib logo" class="size-16 rounded-2xl" />
    <h1 class="text-4xl font-semibold tracking-tight">Bib</h1>
  </header>

  <div class="flex w-full max-w-[420px] flex-col gap-8">
    <!-- Core actions -->
    <nav class="flex flex-col gap-0.5">
      {#each actions as a (a.label)}
        <ActionRow
          icon={a.icon}
          label={a.label}
          shortcut={a.shortcut}
          onclick={a.onclick}
        />
      {/each}
    </nav>

    <!-- Recent projects -->
    <section class="flex flex-col gap-1">
      <h2
        class="px-3 text-xs font-medium uppercase tracking-wider text-muted-foreground"
      >
        Recent
      </h2>
      {#if recents.length === 0}
        <p class="px-3 py-2 text-sm text-muted-foreground">
          No recent projects yet.
        </p>
      {:else}
        {#each recents as r (r.path)}
          <ActionRow icon={RiFolderOpenLine} label={r.name} hint={r.path} />
        {/each}
      {/if}
    </section>
  </div>
</main>

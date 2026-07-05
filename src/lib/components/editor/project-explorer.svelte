<script lang="ts">
  import { project } from "$lib/project.svelte.js";
  import { ScrollArea } from "$lib/components/ui/scroll-area/index.js";
  import TreeNode from "./tree-node.svelte";
  import RiFolderOpenLine from "remixicon-svelte/icons/folder-open-line";
</script>

<aside
  class="flex h-full flex-col border-r border-sidebar-border bg-sidebar text-sidebar-foreground"
>
  <header class="flex h-9 shrink-0 items-center gap-2 border-b border-sidebar-border px-3">
    <span
      class="min-w-0 flex-1 truncate text-xs font-semibold uppercase tracking-wider text-muted-foreground"
      title={project.root ?? ""}
    >
      {project.tree?.name ?? "Explorer"}
    </span>
    <button
      type="button"
      onclick={() => project.openProjectDialog()}
      aria-label="Open project"
      title="Open project…"
      class="grid size-6 shrink-0 place-items-center rounded-md text-muted-foreground outline-none transition-colors hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
    >
      <RiFolderOpenLine width={15} height={15} />
    </button>
  </header>

  <ScrollArea class="flex-1">
    <div class="p-1">
      {#if project.tree?.children}
        {#each project.tree.children as node (node.path)}
          <TreeNode {node} />
        {/each}
      {/if}
    </div>
  </ScrollArea>
</aside>

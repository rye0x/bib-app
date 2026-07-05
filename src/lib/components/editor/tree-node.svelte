<script lang="ts">
  import { untrack } from "svelte";
  import Self from "./tree-node.svelte";
  import { project, fileKind, type TreeNode } from "$lib/project.svelte.js";
  import { cn } from "$lib/utils.js";
  import RiArrowRightSLine from "remixicon-svelte/icons/arrow-right-s-line";
  import RiFileTextLine from "remixicon-svelte/icons/file-text-line";
  import RiImageLine from "remixicon-svelte/icons/image-line";
  import RiBook2Line from "remixicon-svelte/icons/book-2-line";
  import RiFileLine from "remixicon-svelte/icons/file-line";

  type Props = { node: TreeNode; depth?: number };
  let { node, depth = 0 }: Props = $props();

  // Expand the top level by default; deeper folders start collapsed. `depth`
  // is fixed for a given node, so we only read it for the initial value.
  let expanded = $state(untrack(() => depth < 1));

  const isActive = $derived(!node.isDir && node.path === project.openPath);
  const isDirty = $derived(isActive && project.isDirty);

  const FileIcon = $derived.by(() => {
    if (node.name.endsWith(".tex")) return RiFileTextLine;
    if (node.name.endsWith(".bib")) return RiBook2Line;
    if (/\.(png|jpe?g|gif|svg|pdf|eps)$/i.test(node.name)) return RiImageLine;
    return RiFileLine;
  });

  const openable = $derived(fileKind(node.name) !== "other");

  function onclick() {
    if (node.isDir) expanded = !expanded;
    else if (openable) project.openFile(node.path);
  }
</script>

<button
  type="button"
  {onclick}
  title={node.name}
  style="padding-left: {depth * 12 + 8}px"
  class={cn(
    "flex w-full items-center gap-1.5 rounded-md py-1 pr-2 text-left text-sm outline-none transition-colors",
    "hover:bg-sidebar-accent/60 focus-visible:bg-sidebar-accent/60",
    isActive && "bg-sidebar-accent text-sidebar-accent-foreground",
    !node.isDir && !openable && "text-muted-foreground",
  )}
>
  {#if node.isDir}
    <RiArrowRightSLine
      width={14}
      height={14}
      class={cn("shrink-0 transition-transform", expanded && "rotate-90")}
    />
  {:else}
    <span class="shrink-0 text-muted-foreground">
      <FileIcon width={14} height={14} />
    </span>
  {/if}

  <span class="min-w-0 flex-1 truncate">{node.name}</span>

  {#if isDirty}
    <span class="size-1.5 shrink-0 rounded-full bg-foreground/60" aria-label="Unsaved changes"
    ></span>
  {/if}
</button>

{#if node.isDir && expanded && node.children}
  {#each node.children as child (child.path)}
    <Self node={child} depth={depth + 1} />
  {/each}
{/if}

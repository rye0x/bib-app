<script lang="ts">
  import { project, fileKind } from "$lib/project.svelte.js";
  import CodeMirror from "./codemirror.svelte";
  import FilePreview from "./file-preview.svelte";

  const kind = $derived(project.openPath ? fileKind(project.openPath) : "other");

  // Path relative to the project root, for the breadcrumb strip.
  const relPath = $derived.by(() => {
    if (!project.openPath) return "";
    const root = project.root;
    if (root && project.openPath.startsWith(root)) {
      return project.openPath.slice(root.length).replace(/^\//, "");
    }
    return project.openPath;
  });
</script>

<section class="flex h-full flex-col bg-background">
  {#if project.openPath}
    <div
      class="flex h-9 shrink-0 items-center gap-2 border-b border-border px-3 text-xs text-muted-foreground"
    >
      <span class="min-w-0 truncate">{relPath}</span>
      {#if kind === "text" && project.isDirty}
        <span class="size-1.5 rounded-full bg-foreground/60" aria-label="Unsaved changes"></span>
      {/if}
    </div>
    <div class="min-h-0 flex-1">
      {#if kind === "text"}
        <CodeMirror />
      {:else}
        <FilePreview path={project.openPath} />
      {/if}
    </div>
  {:else}
    <div
      class="flex h-full flex-col items-center justify-center gap-1 text-sm text-muted-foreground"
    >
      <p>Select a file to start editing</p>
    </div>
  {/if}
</section>

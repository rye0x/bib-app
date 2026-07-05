<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { fileKind } from "$lib/project.svelte.js";

  type Props = { path: string };
  let { path }: Props = $props();

  // The asset protocol serves the file straight to the webview, so PDFs and
  // images render natively without reading bytes across the IPC boundary.
  const src = $derived(convertFileSrc(path));
  const kind = $derived(fileKind(path));
  const name = $derived(path.split("/").pop() ?? path);
</script>

{#if kind === "pdf"}
  <iframe title={name} {src} class="h-full w-full border-0 bg-muted"></iframe>
{:else if kind === "image"}
  <div class="flex h-full items-center justify-center overflow-auto bg-muted/30 p-4">
    <img {src} alt={name} class="max-h-full max-w-full object-contain" />
  </div>
{:else}
  <div class="flex h-full items-center justify-center text-sm text-muted-foreground">
    <p>Can't preview this file type.</p>
  </div>
{/if}

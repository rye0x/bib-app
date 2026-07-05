<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { goto } from "$app/navigation";
  import * as Resizable from "$lib/components/ui/resizable/index.js";
  import { ProjectExplorer, EditorPanel } from "$lib/components/editor";
  import { project } from "$lib/project.svelte.js";

  // The editor is only meaningful with a project open. If we land here without
  // one (e.g. a reload), send the user back to the launcher.
  $effect(() => {
    if (!project.isOpen) goto("/");
  });

  onMount(() => {
    // Save is driven by the native File menu (⌘S) — same pattern as zoom.
    const unSave = listen("menu:save", () => project.save());
    return () => {
      unSave.then((f) => f());
    };
  });
</script>

<div class="h-screen overflow-hidden pt-8">
  {#if project.isOpen}
    <Resizable.PaneGroup direction="horizontal" class="h-full">
      <Resizable.Pane defaultSize={22} minSize={12} maxSize={40}>
        <ProjectExplorer />
      </Resizable.Pane>
      <Resizable.Handle />
      <Resizable.Pane defaultSize={78}>
        <EditorPanel />
      </Resizable.Pane>
    </Resizable.PaneGroup>
  {/if}
</div>

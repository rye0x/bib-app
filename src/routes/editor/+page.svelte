<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { goto } from "$app/navigation";
  import * as Resizable from "$lib/components/ui/resizable/index.js";
  import { ProjectExplorer, EditorPanel, FuzzyFinder, ProjectSearch } from "$lib/components/editor";
  import { project } from "$lib/project.svelte.js";

  // Keyboard-first overlays: quick-open (⌘P) and project-wide search (⌘⇧F).
  // Find & replace in the current file (⌘F) is owned by CodeMirror itself.
  let finderOpen = $state(false);
  let searchOpen = $state(false);

  // The editor is only meaningful with a project open. If we land here without
  // one (e.g. a reload), send the user back to the launcher.
  $effect(() => {
    if (!project.isOpen) goto("/");
  });

  function onkeydown(e: KeyboardEvent) {
    if (!(e.metaKey || e.ctrlKey)) return;
    const key = e.key.toLowerCase();
    if (key === "p" && !e.shiftKey) {
      e.preventDefault();
      searchOpen = false;
      finderOpen = true;
    } else if (key === "f" && e.shiftKey) {
      e.preventDefault();
      finderOpen = false;
      searchOpen = true;
    }
  }

  onMount(() => {
    // Save is driven by the native File menu (⌘S) — same pattern as zoom.
    const unSave = listen("menu:save", () => project.save());
    return () => {
      unSave.then((f) => f());
    };
  });
</script>

<svelte:window {onkeydown} />

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

    <FuzzyFinder bind:open={finderOpen} />
    <ProjectSearch bind:open={searchOpen} />
  {/if}
</div>

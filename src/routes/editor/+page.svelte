<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { goto } from "$app/navigation";
  import * as Resizable from "$lib/components/ui/resizable/index.js";
  import {
    ProjectExplorer,
    EditorPanel,
    PreviewPane,
    FuzzyFinder,
    ProjectSearch,
  } from "$lib/components/editor";
  import { project } from "$lib/project.svelte.js";
  import { compile, initCompile } from "$lib/compile.svelte.js";

  // Keyboard-first overlays: quick-open (⌘P) and project-wide search (⌘⇧F).
  // Find & replace in the current file (⌘F) is owned by CodeMirror itself.
  let finderOpen = $state(false);
  let searchOpen = $state(false);

  // The editor is only meaningful with a project open. If we land here without
  // one (e.g. a reload), send the user back to the launcher.
  $effect(() => {
    if (!project.isOpen) goto("/");
  });

  // Clear compile output when switching projects so a previous project's PDF
  // and diagnostics can't leak into a freshly opened one.
  let currentRoot = project.root;
  $effect(() => {
    if (project.root !== currentRoot) {
      currentRoot = project.root;
      compile.reset();
    }
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
    initCompile();
    // Save and Build are driven by the native File menu (⌘S / ⌘B) — same
    // pattern as zoom. On save we recompile when auto-build is on, so the
    // preview stays in step with the source ("live" / on-save recompile).
    const unSave = listen("menu:save", async () => {
      await project.save();
      if (compile.autoBuild) compile.compile();
    });
    const unBuild = listen("menu:build", () => compile.compile());
    return () => {
      unSave.then((f) => f());
      unBuild.then((f) => f());
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
      <Resizable.Pane defaultSize={48} minSize={25}>
        <EditorPanel />
      </Resizable.Pane>
      <Resizable.Handle />
      <Resizable.Pane defaultSize={30} minSize={18}>
        <PreviewPane />
      </Resizable.Pane>
    </Resizable.PaneGroup>

    <FuzzyFinder bind:open={finderOpen} />
    <ProjectSearch bind:open={searchOpen} />
  {/if}
</div>

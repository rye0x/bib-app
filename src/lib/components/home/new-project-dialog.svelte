<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import RiFolderOpenLine from "remixicon-svelte/icons/folder-open-line";
  import { Button } from "$lib/components/ui/button/index.js";
  import { project } from "$lib/project.svelte.js";

  type Props = {
    open: boolean;
    oncreated?: (root: string) => void;
  };
  let { open: isOpen = $bindable(), oncreated }: Props = $props();

  let name = $state("");
  let parent = $state<string | null>(null);
  let error = $state<string | null>(null);
  let creating = $state(false);
  let nameInput = $state<HTMLInputElement>();

  const canCreate = $derived(name.trim() !== "" && parent !== null && !creating);

  // Reset the form each time the dialog opens.
  $effect(() => {
    if (isOpen) {
      name = "";
      parent = null;
      error = null;
      creating = false;
      nameInput?.focus();
    }
  });

  async function chooseLocation() {
    const picked = await open({ directory: true, multiple: false });
    if (typeof picked === "string") parent = picked;
  }

  async function create() {
    if (!canCreate || parent === null) return;
    creating = true;
    error = null;
    try {
      const root = await project.createProject(parent, name.trim());
      isOpen = false;
      oncreated?.(root);
    } catch (e) {
      error = typeof e === "string" ? e : String(e);
    } finally {
      creating = false;
    }
  }

  function onkeydown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      isOpen = false;
    } else if (e.key === "Enter") {
      e.preventDefault();
      create();
    }
  }
</script>

{#if isOpen}
  <!-- Backdrop: click to dismiss. -->
  <div
    class="fixed inset-0 z-50 flex justify-center bg-black/30 pt-[18vh]"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) isOpen = false;
    }}
  >
    <div
      class="flex h-fit w-full max-w-[440px] flex-col gap-4 rounded-xl border border-border bg-popover p-5 text-popover-foreground shadow-2xl"
      role="dialog"
      aria-label="New LaTeX project"
    >
      <h2 class="text-sm font-semibold">New LaTeX Project</h2>

      <label class="flex flex-col gap-1.5">
        <span class="text-xs font-medium text-muted-foreground">Name</span>
        <input
          bind:this={nameInput}
          bind:value={name}
          {onkeydown}
          type="text"
          placeholder="my-paper"
          class="w-full rounded-md border border-border bg-transparent px-3 py-2 text-sm outline-none focus-visible:ring-2 focus-visible:ring-ring/40 placeholder:text-muted-foreground"
        />
      </label>

      <div class="flex flex-col gap-1.5">
        <span class="text-xs font-medium text-muted-foreground">Location</span>
        <button
          type="button"
          onclick={chooseLocation}
          class="flex w-full items-center gap-2 rounded-md border border-border px-3 py-2 text-left text-sm outline-none transition-colors hover:bg-accent focus-visible:ring-2 focus-visible:ring-ring/40"
        >
          <RiFolderOpenLine width={16} height={16} class="shrink-0 text-muted-foreground" />
          <span class="min-w-0 flex-1 truncate {parent ? '' : 'text-muted-foreground'}">
            {parent ?? "Choose a folder…"}
          </span>
        </button>
      </div>

      {#if error}
        <p class="text-xs text-destructive">{error}</p>
      {/if}

      <div class="flex justify-end gap-2">
        <Button variant="outline" size="sm" onclick={() => (isOpen = false)}>Cancel</Button>
        <Button size="sm" disabled={!canCreate} onclick={create}>Create</Button>
      </div>
    </div>
  </div>
{/if}

<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import RiPlayLine from "remixicon-svelte/icons/play-line";
  import RiLoader4Line from "remixicon-svelte/icons/loader-4-line";
  import RiCheckLine from "remixicon-svelte/icons/check-line";
  import RiCloseCircleLine from "remixicon-svelte/icons/close-circle-line";
  import RiErrorWarningLine from "remixicon-svelte/icons/error-warning-line";
  import RiFileList2Line from "remixicon-svelte/icons/file-list-2-line";
  import RiCheckboxLine from "remixicon-svelte/icons/checkbox-line";
  import RiCheckboxBlankLine from "remixicon-svelte/icons/checkbox-blank-line";
  import { Button } from "$lib/components/ui/button/index.js";
  import { project } from "$lib/project.svelte.js";
  import { compile, type Diagnostic } from "$lib/compile.svelte.js";

  // Bottom log drawer: "problems" (parsed diagnostics) or "log" (raw output).
  let logOpen = $state(false);
  let logTab = $state<"problems" | "log">("problems");

  // Cache-busted PDF source so a rebuild at the same path reloads in the webview.
  const pdfSrc = $derived.by(() => {
    const path = compile.result?.pdfPath;
    return path ? `${convertFileSrc(path)}?v=${compile.pdfVersion}` : null;
  });

  // Auto-reveal the drawer the moment a build finishes with problems, so errors
  // aren't hidden behind a collapsed panel.
  let lastVersion = 0;
  $effect(() => {
    const v = compile.pdfVersion;
    if (v !== lastVersion) {
      lastVersion = v;
      if (compile.diagnostics.length > 0) {
        logOpen = true;
        logTab = "problems";
      }
    }
  });

  // A diagnostic's absolute path, so we can open + reveal it in the editor.
  function jumpTo(d: Diagnostic) {
    if (!d.file || d.line === null || !project.root) return;
    const abs = `${project.root}/${d.file}`;
    project.openFileAt(abs, d.line);
  }
</script>

<section class="flex h-full flex-col bg-background">
  <!-- Toolbar -->
  <div class="flex h-9 shrink-0 items-center gap-2 border-b border-border pr-2 pl-3 text-xs">
    <div class="flex min-w-0 flex-1 items-center gap-1.5 text-muted-foreground">
      {#if compile.isCompiling}
        <RiLoader4Line class="size-3.5 animate-spin text-foreground" />
        <span>Building…</span>
      {:else if compile.status === "success"}
        <RiCheckLine class="size-3.5 text-primary" />
        <span class="min-w-0 truncate"
          >Built {compile.result?.mainFile} in {((compile.result?.durationMs ?? 0) / 1000).toFixed(
            1,
          )}s</span
        >
      {:else if compile.status === "error"}
        <RiCloseCircleLine class="size-3.5 text-destructive" />
        <span class="min-w-0 truncate">{compile.error ?? "Build failed"}</span>
      {:else}
        <span>Preview</span>
      {/if}
    </div>

    <button
      type="button"
      onclick={() => (logOpen = !logOpen)}
      aria-pressed={logOpen}
      class="flex items-center gap-1.5 rounded-md px-1.5 py-1 text-muted-foreground hover:bg-muted hover:text-foreground aria-pressed:bg-muted aria-pressed:text-foreground"
      title="Toggle build log"
    >
      <RiFileList2Line class="size-3.5" />
      {#if compile.errorCount > 0}
        <span class="text-destructive">{compile.errorCount}</span>
      {/if}
      {#if compile.warningCount > 0}
        <span class="text-primary">{compile.warningCount}</span>
      {/if}
    </button>

    <button
      type="button"
      onclick={() => compile.toggleAutoBuild()}
      aria-pressed={compile.autoBuild}
      class="flex items-center gap-1 rounded-md px-1.5 py-1 text-muted-foreground hover:bg-muted hover:text-foreground aria-pressed:text-foreground"
      title="Rebuild automatically on save"
    >
      {#if compile.autoBuild}
        <RiCheckboxLine class="size-3.5 text-primary" />
      {:else}
        <RiCheckboxBlankLine class="size-3.5" />
      {/if}
      <span>Auto</span>
    </button>

    <Button size="xs" onclick={() => compile.compile()} disabled={compile.isCompiling}>
      <RiPlayLine />
      Build
    </Button>
  </div>

  <!-- PDF viewer -->
  <div class="min-h-0 flex-1">
    {#if pdfSrc}
      <iframe title="Compiled PDF" src={pdfSrc} class="h-full w-full border-0 bg-muted"></iframe>
    {:else}
      <div
        class="flex h-full flex-col items-center justify-center gap-3 text-sm text-muted-foreground"
      >
        {#if compile.status === "error"}
          <p>No PDF produced — check the build log.</p>
        {:else}
          <p>Build the project to see a preview.</p>
        {/if}
        <Button
          size="sm"
          variant="outline"
          onclick={() => compile.compile()}
          disabled={compile.isCompiling}
        >
          <RiPlayLine />
          Build now
        </Button>
      </div>
    {/if}
  </div>

  <!-- Log / problems drawer -->
  {#if logOpen}
    <div class="flex h-2/5 shrink-0 flex-col border-t border-border">
      <div class="flex h-8 shrink-0 items-center gap-1 border-b border-border px-2 text-xs">
        <button
          type="button"
          onclick={() => (logTab = "problems")}
          aria-pressed={logTab === "problems"}
          class="rounded-md px-2 py-1 text-muted-foreground hover:text-foreground aria-pressed:text-foreground"
        >
          Problems {#if compile.diagnostics.length > 0}({compile.diagnostics.length}){/if}
        </button>
        <button
          type="button"
          onclick={() => (logTab = "log")}
          aria-pressed={logTab === "log"}
          class="rounded-md px-2 py-1 text-muted-foreground hover:text-foreground aria-pressed:text-foreground"
        >
          Log
        </button>
      </div>

      <div class="min-h-0 flex-1 overflow-auto">
        {#if logTab === "problems"}
          {#if compile.diagnostics.length === 0}
            <div class="flex h-full items-center justify-center text-xs text-muted-foreground">
              <p>{compile.status === "idle" ? "No build yet." : "No problems."}</p>
            </div>
          {:else}
            <ul class="divide-y divide-border/60 text-xs">
              {#each compile.diagnostics as d (d.severity + d.message + d.file + d.line)}
                <li>
                  <button
                    type="button"
                    onclick={() => jumpTo(d)}
                    disabled={!d.file || d.line === null}
                    class="flex w-full items-start gap-2 px-3 py-2 text-left hover:bg-muted enabled:cursor-pointer disabled:cursor-default"
                  >
                    {#if d.severity === "error"}
                      <RiCloseCircleLine class="mt-px size-3.5 shrink-0 text-destructive" />
                    {:else}
                      <RiErrorWarningLine class="mt-px size-3.5 shrink-0 text-primary" />
                    {/if}
                    <span class="min-w-0 flex-1">
                      <span class="text-foreground">{d.message}</span>
                      {#if d.hint}
                        <span class="mt-0.5 block text-muted-foreground">{d.hint}</span>
                      {/if}
                    </span>
                    {#if d.file}
                      <span class="shrink-0 font-mono text-[11px] text-muted-foreground">
                        {d.file}{#if d.line !== null}:{d.line}{/if}
                      </span>
                    {/if}
                  </button>
                </li>
              {/each}
            </ul>
          {/if}
        {:else}
          <pre
            class="px-3 py-2 font-mono text-[11px] leading-relaxed whitespace-pre-wrap text-muted-foreground">{compile
              .result?.log ?? "No build yet."}</pre>
        {/if}
      </div>
    </div>
  {/if}
</section>

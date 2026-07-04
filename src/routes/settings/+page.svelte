<script lang="ts">
  import RiSunLine from "remixicon-svelte/icons/sun-line";
  import RiMoonLine from "remixicon-svelte/icons/moon-line";
  import RiSubtractLine from "remixicon-svelte/icons/subtract-line";
  import RiAddLine from "remixicon-svelte/icons/add-line";

  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { theme } from "$lib/theme.svelte";
  import { zoom } from "$lib/zoom.svelte";
</script>

<main
  class="flex min-h-screen flex-col items-center bg-background px-6 pt-[72px] pb-12 text-foreground"
>
  <div class="flex w-full max-w-[560px] flex-col gap-8">
    <header class="flex flex-col gap-1">
      <h1 class="text-2xl font-semibold tracking-tight">Settings</h1>
      <p class="text-sm text-muted-foreground">
        Customize how Bib looks and behaves.
      </p>
    </header>

    <!-- Appearance -->
    <Card.Root>
      <Card.Header>
        <Card.Title>Appearance</Card.Title>
        <Card.Description>Choose a light or dark theme.</Card.Description>
      </Card.Header>
      <Card.Content>
        <div class="flex items-center justify-between gap-4">
          <div class="flex flex-col">
            <span class="text-sm font-medium">Theme</span>
            <span class="text-xs text-muted-foreground">
              Currently {theme.isDark ? "dark" : "light"}
            </span>
          </div>
          <div class="flex gap-1.5">
            <Button
              variant={theme.isDark ? "outline" : "default"}
              size="sm"
              onclick={() => theme.set("light")}
            >
              <RiSunLine width={16} height={16} />
              Light
            </Button>
            <Button
              variant={theme.isDark ? "default" : "outline"}
              size="sm"
              onclick={() => theme.set("dark")}
            >
              <RiMoonLine width={16} height={16} />
              Dark
            </Button>
          </div>
        </div>
      </Card.Content>
    </Card.Root>

    <!-- Editor -->
    <Card.Root>
      <Card.Header>
        <Card.Title>Editor</Card.Title>
        <Card.Description>Zoom and display preferences.</Card.Description>
      </Card.Header>
      <Card.Content>
        <div class="flex items-center justify-between gap-4">
          <div class="flex flex-col">
            <span class="text-sm font-medium">Zoom</span>
            <span class="text-xs text-muted-foreground">
              Or press <kbd class="font-sans">⌘ +</kbd>,
              <kbd class="font-sans">⌘ −</kbd>, <kbd class="font-sans">⌘ 0</kbd
              > to reset.
            </span>
          </div>
          <div class="flex items-center gap-1.5">
            <Button
              variant="outline"
              size="icon-sm"
              aria-label="Zoom out"
              disabled={!zoom.canZoomOut}
              onclick={() => zoom.out()}
            >
              <RiSubtractLine width={16} height={16} />
            </Button>
            <button
              type="button"
              class="w-14 text-center text-sm font-medium tabular-nums hover:text-primary"
              title="Reset zoom"
              onclick={() => zoom.reset()}
            >
              {zoom.percent}%
            </button>
            <Button
              variant="outline"
              size="icon-sm"
              aria-label="Zoom in"
              disabled={!zoom.canZoomIn}
              onclick={() => zoom.in()}
            >
              <RiAddLine width={16} height={16} />
            </Button>
          </div>
        </div>
      </Card.Content>
    </Card.Root>

    <p class="text-center text-xs text-muted-foreground">Bib · v0.1.0</p>
  </div>
</main>

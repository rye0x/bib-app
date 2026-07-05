<script lang="ts">
  import type { Component } from "svelte";
  import type { SVGAttributes } from "svelte/elements";
  import { cn } from "$lib/utils.js";

  type IconComponent = Component<SVGAttributes<SVGSVGElement>>;

  type Props = {
    icon?: IconComponent;
    label: string;
    hint?: string;
    shortcut?: string;
    class?: string;
    onclick?: () => void;
  };

  let { icon: Icon, label, hint, shortcut, class: className, onclick }: Props = $props();
</script>

<button
  type="button"
  {onclick}
  class={cn(
    "group flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-left transition-colors",
    "hover:bg-accent focus-visible:ring-ring/40 focus-visible:ring-2 outline-none",
    className,
  )}
>
  {#if Icon}
    <span class="text-muted-foreground transition-colors group-hover:text-foreground">
      <Icon width={18} height={18} />
    </span>
  {/if}

  <span class="min-w-0 flex-1 truncate text-sm font-medium">{label}</span>

  {#if hint}
    <span class="min-w-0 max-w-[45%] truncate text-xs text-muted-foreground">{hint}</span>
  {/if}

  {#if shortcut}
    <kbd class="shrink-0 font-sans text-xs text-muted-foreground/70">
      {shortcut}
    </kbd>
  {/if}
</button>

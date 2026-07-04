# Svelte 5 — Runes & Component Patterns

This project is on Svelte 5. Use runes exclusively. The legacy API (`export let`, `$:`,
`writable`/`readable` stores, `createEventDispatcher`, `<slot>`) is not used in new code and
mixing it in makes reactivity opaque — avoid it.

## Runes cheat sheet

| Rune | Use for |
|------|---------|
| `$state(x)` | mutable reactive value |
| `$derived(expr)` | value computed from other reactive values |
| `$derived.by(() => {…})` | derived value needing a function body |
| `$props()` | component inputs |
| `$bindable(default)` | a prop the parent can two-way bind |
| `$effect(() => {…})` | run side effects when dependencies change (DOM, subscriptions) |

`$effect.pre` runs before DOM updates; `$effect` runs after. `$effect` only works during
component init or inside another effect — for module-level lifecycle use `onMount`.

## Props

Always a single `type Props`, destructured from `$props()`. Rename `class` to `className`.
Give optional props defaults inline.

```svelte
<script lang="ts">
  type Props = {
    title: string;
    count?: number;
    class?: string;
    onchange?: (value: number) => void;
  };

  let { title, count = 0, class: className, onchange, ...rest }: Props = $props();
</script>
```

Spread the rest onto the root element (`{...rest}`) when the component wraps a native element so
callers can pass `id`, `aria-*`, `data-*`, etc. — this is exactly what the shadcn button does.

### Extending native element attributes

For wrapper components, extend the DOM attribute type instead of hand-listing props. Use the
repo's helpers from `$lib/utils.js`:

```svelte
<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements";
  import { type WithElementRef, cn } from "$lib/utils.js";

  let {
    class: className,
    ref = $bindable(null),
    children,
    ...rest
  }: WithElementRef<HTMLButtonAttributes> = $props();
</script>

<button bind:this={ref} class={cn("…", className)} {...rest}>
  {@render children?.()}
</button>
```

`WithElementRef<T>` adds a `ref` prop; `WithoutChildrenOrChild<T>` strips `children`/`child` when
you render them yourself. These live in `src/lib/utils.ts`.

## Children & snippets (not slots)

Content projection uses snippets. Declare `children` (and any named snippets) as props and render
with `{@render …}`.

```svelte
<script lang="ts">
  import type { Snippet } from "svelte";
  let { children, header }: { children: Snippet; header?: Snippet } = $props();
</script>

{#if header}{@render header()}{/if}
<div class="content">{@render children()}</div>
```

Render optionally with `{@render children?.()}`. Pass data via snippet parameters:
`{@render row(item)}` where `row: Snippet<[Item]>`.

## Events are callback props

No `createEventDispatcher`. Expose `on*` callback props and invoke them optionally so parents
that don't pass one don't crash:

```svelte
let { onselect }: { onselect?: (id: string) => void } = $props();
// ...
<button onclick={() => onselect?.(item.id)}>…</button>
```

Native DOM events use lowercase attributes directly: `onclick`, `oninput`, `onkeydown`.

## Shared reactive state — `*.svelte.ts` modules

This is the project's replacement for stores. A `.svelte.ts` file may use runes at module scope.
Own the `$state` privately and export an object exposing getters (and mutators). See
`src/lib/theme.svelte.ts` and `src/lib/zoom.svelte.ts` for the canonical shape:

```ts
// counter.svelte.ts
let count = $state(0);

export const counter = {
  get value() {
    return count;
  },
  increment() {
    count += 1;
  },
  reset() {
    count = 0;
  },
};
```

Consumers `import { counter } from "$lib/counter.svelte"` and read `counter.value` reactively in
markup. Keep DOM/`localStorage` side effects inside the module (as theme/zoom do) so there's one
source of truth. If initial state must be read from the DOM/storage, expose an `init*()` function
and call it once from `onMount`.

## Reactivity gotchas

- `$derived` must be **pure** — no side effects, no assignment. Put side effects in `$effect`.
- Don't destructure reactive state and expect the pieces to stay reactive — read through the
  object/getter at the point of use.
- Reassigning is what triggers updates. For arrays/objects, Svelte 5's deep proxies make
  `arr.push(x)` / `obj.k = v` reactive on `$state`, but reassigning (`arr = [...arr, x]`) is always
  safe and clear.
- `bind:this={ref}` where `ref` is a `$bindable(null)` prop is how components expose their DOM node.

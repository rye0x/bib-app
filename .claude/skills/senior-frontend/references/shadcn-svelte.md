# shadcn-svelte v1 in this project

Components are **copied into the repo**, not imported from a package — you own and edit them. They
live in `src/lib/components/ui/<component>/`. The config is `components.json`:

```jsonc
{
  "tailwind": { "css": "src/app.css", "baseColor": "neutral" },
  "aliases": {
    "components": "$lib/components",
    "utils": "$lib/utils",
    "ui": "$lib/components/ui",
    "hooks": "$lib/hooks",
    "lib": "$lib"
  },
  "typescript": true,
  "registry": "https://shadcn-svelte.com/registry",
  "style": "luma",
  "iconLibrary": "remixicon"
}
```

Note the `luma` style and `remixicon` icon library — keep both consistent when adding components.

## Adding a component

Use the CLI; never hand-write a primitive that the registry provides. It scaffolds the folder,
the `index.ts` barrel, and any dependencies.

```bash
pnpm dlx shadcn-svelte@latest add dialog
pnpm dlx shadcn-svelte@latest add dropdown-menu tooltip   # several at once
```

Installed so far: `button`, `card`. Anything else must be added before use.

## Using a component

Import via the `index.ts` barrel. Card-style components use a namespace import; simple ones use a
default:

```svelte
<script lang="ts">
  import * as Card from "$lib/components/ui/card/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
</script>

<Card.Root>
  <Card.Header>
    <Card.Title>Title</Card.Title>
    <Card.Description>Subtitle</Card.Description>
  </Card.Header>
  <Card.Content>…</Card.Content>
  <Card.Footer>
    <Button variant="outline" size="sm" onclick={save}>Save</Button>
  </Card.Footer>
</Card.Root>
```

The barrel exports both bare names (`Root`, `Content`) for `* as Card` and prefixed aliases
(`CardContent`) for named imports — either works; match the surrounding file.

## The `cn` helper

Every component merges classes with `cn` from `$lib/utils.js` (clsx + tailwind-merge). This lets
callers override styles without specificity fights — always run the final class list through it and
put a `className` prop last so caller classes win:

```svelte
class={cn("rounded-lg px-3 py-2 text-sm", className)}
```

## `tailwind-variants` (`tv`) for variant styling

Components with variants (like `button`) define a `tv({ base, variants, defaultVariants })` in a
`<script lang="ts" module>` block and export it plus its `VariantProps` types. Follow that exact
structure — see `src/lib/components/ui/button/button.svelte`:

```svelte
<script lang="ts" module>
  import { type VariantProps, tv } from "tailwind-variants";

  export const badgeVariants = tv({
    base: "inline-flex items-center rounded-md px-2 py-0.5 text-xs font-medium",
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground",
        outline: "border border-border text-foreground",
      },
    },
    defaultVariants: { variant: "default" },
  });

  export type BadgeVariant = VariantProps<typeof badgeVariants>["variant"];
</script>
```

The `module` block runs once and is where shared/exported values (variant definitions, types) go;
the instance `<script>` below it holds per-instance props and state.

## Conventions worth copying

- `data-slot="…"` attributes on component roots — the registry uses them for styling hooks; keep
  them when editing.
- `bind:this={ref}` with `ref = $bindable(null)` to expose the DOM node.
- Anchor/button polymorphism: the button renders `<a>` when `href` is set, else `<button>` — see
  its template if you need a similar dual-element component.

## Icons — remixicon-svelte

Import each icon as a default export from its own path and render as a component:

```svelte
<script lang="ts">
  import RiFileAddLine from "remixicon-svelte/icons/file-add-line";
</script>

<RiFileAddLine width={18} height={18} />
```

The naming maps from RemixIcon: `settings-3-line` → `remixicon-svelte/icons/settings-3-line`,
imported as `RiSettings3Line`. Size with `width`/`height` props or Tailwind `size-*` classes; color
follows `currentColor`, so `text-muted-foreground` on a wrapper tints the icon.

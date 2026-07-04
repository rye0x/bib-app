# Tailwind v4 (CSS-first) & Theming

This project uses **Tailwind v4**, configured entirely in CSS — there is no `tailwind.config.js`.
The single source is `src/app.css`, which starts with:

```css
@import "tailwindcss";
@import "tw-animate-css";
@import "shadcn-svelte/tailwind.css";
@import "@fontsource-variable/inter";

@custom-variant dark (&:is(.dark *));
```

Tailwind is wired through the Vite plugin (`@tailwindcss/vite`) — no PostCSS config.

## The token system — style with tokens, never raw values

Colors live as CSS custom properties defined twice: light values on `:root`, dark values on
`.dark`. The `@theme inline { … }` block maps each `--color-*` utility to its token
(`--color-primary: var(--primary)`), which is what makes `bg-primary`, `text-muted-foreground`,
`border-border`, etc. resolve.

**Always use the semantic utilities, never hard-coded colors.** `bg-primary` not `bg-[#F5A01F]`,
`text-muted-foreground` not `text-gray-500`. Because light/dark values are swapped on `:root`
vs `.dark`, semantic utilities adapt to the theme for free; raw colors don't and will look wrong in
one mode.

Available token families (each has matching `bg-`, `text-`, `border-`, `ring-` utilities where it
makes sense):

`background` / `foreground`, `card` (+ `-foreground`), `popover`, `primary`, `secondary`, `muted`,
`accent`, `destructive`, `border`, `input`, `ring`, `chart-1..5`, and the `sidebar-*` set.

Colors are authored in **oklch** and the palette is derived from the app logo (orange primary,
navy foreground). Opacity modifiers compose normally: `bg-primary/80`, `ring-ring/30`.

## Radius scale

Radii derive from one `--radius` var via `@theme`: `rounded-sm/md/lg/xl/2xl/3xl/4xl` scale off it.
Use these named utilities so corner rounding stays consistent (the button uses `rounded-4xl`).

## Dark mode

Dark mode is class-based: the `.dark` class on `<html>` flips every token. `@custom-variant dark`
lets you write `dark:` variants (`dark:bg-input/30`). The class is:
- applied **before paint** by an inline script in `app.html` (avoids a flash), and
- mirrored + toggled reactively by `src/lib/theme.svelte.ts`.

To change the theme in code, call `theme.set("dark")` / `theme.toggle()` from that module — don't
touch the `<html>` class directly, or the reactive mirror and `localStorage` get out of sync.

## Adding or changing tokens

1. Add the variable to **both** `:root` and `.dark` in `app.css` (light + dark value).
2. Map it under `@theme inline` as `--color-<name>: var(--<name>)` so the utility exists.
3. Then use `bg-<name>` / `text-<name>` in markup.

Keep new colors in oklch and in the existing hue family so they sit with the logo-derived palette.

## Fonts & base layer

`--font-sans` is Inter Variable (`@fontsource-variable/inter`), exposed as `font-sans`. The
`@layer base` block sets the default border color, focus ring, and body font — global element
defaults belong there, component styling stays in the component via `cn`/`tv`.

## Note: the `zoom` factor

UI scaling is done with CSS `zoom` on `<html>` (see `src/lib/zoom.svelte.ts`), not Tailwind. Don't
reimplement scaling with font-size/rem tricks — use the `zoom` state module.

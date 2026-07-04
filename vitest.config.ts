import { defineConfig } from "vitest/config";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { svelteTesting } from "@testing-library/svelte/vite";
import { fileURLToPath } from "node:url";

// Standalone Vitest config. We use the bare `svelte()` plugin (not the full
// `sveltekit()` plugin) so tests run against the client build without SSR /
// routing machinery. `$lib` is aliased manually since SvelteKit isn't loaded.
export default defineConfig({
  plugins: [svelte(), svelteTesting()],
  resolve: {
    alias: {
      $lib: fileURLToPath(new URL("./src/lib", import.meta.url)),
    },
  },
  test: {
    environment: "jsdom",
    globals: true,
    setupFiles: ["./vitest-setup.ts"],
    include: ["src/**/*.{test,spec}.{js,ts}"],
    exclude: ["e2e/**", "node_modules/**", "build/**", ".svelte-kit/**"],
  },
});

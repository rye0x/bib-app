import js from "@eslint/js";
import ts from "typescript-eslint";
import svelte from "eslint-plugin-svelte";
import prettier from "eslint-config-prettier";
import globals from "globals";
import svelteConfig from "./svelte.config.js";

export default ts.config(
  {
    ignores: [
      "build/",
      ".svelte-kit/",
      "dist/",
      "node_modules/",
      "src-tauri/",
      "static/",
      "playwright-report/",
      "test-results/",
    ],
  },
  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs.recommended,
  prettier,
  ...svelte.configs.prettier,
  {
    languageOptions: {
      globals: { ...globals.browser, ...globals.node },
    },
    rules: {
      // SPA with static routes and a fixed base path — resolve() adds noise
      // and would force edits to shadcn-generated components.
      "svelte/no-navigation-without-resolve": "off",
    },
  },
  {
    files: ["**/*.svelte", "**/*.svelte.ts", "**/*.svelte.js"],
    languageOptions: {
      parserOptions: {
        projectService: true,
        extraFileExtensions: [".svelte"],
        parser: ts.parser,
        svelteConfig,
      },
    },
  },
);

import { test as base, expect } from "@playwright/test";
import { installTauriMock } from "./tauri-mock.js";

// Every page in the E2E suite gets the Tauri runtime stub installed before load.
export const test = base.extend({
  page: async ({ page }, use) => {
    await page.addInitScript(installTauriMock);
    await use(page);
  },
});

export { expect };

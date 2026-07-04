import { test, expect } from "./fixtures.js";

// Base smoke coverage that holds regardless of feature work: the SPA boots
// under the mocked Tauri runtime and the app shell renders. Feature-specific
// e2e (home actions, settings) lives with the feature branch that adds it.
test.describe("app shell", () => {
  test("boots under the Tauri mock and renders the titlebar controls", async ({ page }) => {
    await page.goto("/");
    // These only render if getCurrentWindow() resolved at layout init — proof
    // the injected Tauri stub is working.
    await expect(page.getByRole("button", { name: "Close" })).toBeVisible();
    await expect(page.getByRole("button", { name: "Minimize" })).toBeVisible();
    await expect(page.getByRole("button", { name: "Maximize" })).toBeVisible();
  });

  test("renders a heading on the home route", async ({ page }) => {
    await page.goto("/");
    // Assert a heading renders without pinning its text, so this smoke test
    // survives home-page content changes on top of this branch.
    await expect(page.locator("h1").first()).toBeVisible();
  });
});

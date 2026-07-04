import { test, expect } from "./fixtures.js";

test.describe("settings", () => {
  test("navigates from home to settings and back", async ({ page }) => {
    await page.goto("/");

    await page.getByRole("button", { name: "Settings" }).click();
    await expect(page).toHaveURL(/\/settings$/);
    await expect(page.getByRole("heading", { name: "Settings" })).toBeVisible();

    await page.getByRole("button", { name: "Back" }).click();
    await expect(page).toHaveURL(/:1420\/$/);
    await expect(page.getByRole("heading", { name: "Bib" })).toBeVisible();
  });

  test("toggles theme and zoom controls", async ({ page }) => {
    await page.goto("/settings");

    await page.getByRole("button", { name: "Dark" }).click();
    await expect(page.locator("html")).toHaveClass(/dark/);

    await page.getByRole("button", { name: "Light" }).click();
    await expect(page.locator("html")).not.toHaveClass(/dark/);

    await expect(page.getByText("100%")).toBeVisible();
    await page.getByRole("button", { name: "Zoom in" }).click();
    await expect(page.getByText("110%")).toBeVisible();
  });
});

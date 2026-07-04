import { test, expect } from "./fixtures.js";

test.describe("home", () => {
  test("shows the brand and core actions", async ({ page }) => {
    await page.goto("/");

    await expect(page.getByRole("heading", { name: "Bib" })).toBeVisible();
    await expect(page.getByRole("button", { name: "New LaTeX Project" })).toBeVisible();
    await expect(page.getByRole("button", { name: "Open LaTeX Project" })).toBeVisible();
    await expect(page.getByRole("button", { name: "Choose Template" })).toBeVisible();
  });

  test("shows the empty recents state", async ({ page }) => {
    await page.goto("/");
    await expect(page.getByText("No recent projects yet.")).toBeVisible();
  });
});

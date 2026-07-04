import { defineConfig, devices } from "@playwright/test";
import process from "node:process";

// The SvelteKit frontend runs as a browser SPA here (Tauri is mocked per-page),
// so we drive the Vite dev server. Real Tauri E2E via tauri-driver isn't an
// option on macOS.
const HOST = "127.0.0.1";
const PORT = 1420;
const baseURL = `http://${HOST}:${PORT}`;

export default defineConfig({
  testDir: "./e2e",
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  reporter: process.env.CI ? "github" : "list",
  use: {
    baseURL,
    trace: "on-first-retry",
  },
  projects: [{ name: "chromium", use: { ...devices["Desktop Chrome"] } }],
  webServer: {
    command: "pnpm dev",
    url: baseURL,
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});

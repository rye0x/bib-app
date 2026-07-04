import { describe, it, expect, beforeEach } from "vitest";
import { theme, initTheme } from "./theme.svelte.js";

describe("theme", () => {
  beforeEach(() => {
    document.documentElement.classList.remove("dark");
    localStorage.removeItem("theme");
    initTheme();
  });

  it("initializes to light from a clean DOM", () => {
    expect(theme.value).toBe("light");
    expect(theme.isDark).toBe(false);
  });

  it("set('dark') flips the class and persists the choice", () => {
    theme.set("dark");
    expect(theme.isDark).toBe(true);
    expect(document.documentElement.classList.contains("dark")).toBe(true);
    expect(localStorage.getItem("theme")).toBe("dark");
  });

  it("toggle flips between light and dark", () => {
    theme.set("light");
    theme.toggle();
    expect(theme.value).toBe("dark");
    theme.toggle();
    expect(theme.value).toBe("light");
  });
});

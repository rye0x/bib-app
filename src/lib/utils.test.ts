import { describe, it, expect } from "vitest";
import { cn } from "./utils.js";

describe("cn", () => {
  it("joins class names", () => {
    expect(cn("a", "b")).toBe("a b");
  });

  it("last conflicting tailwind class wins (twMerge)", () => {
    expect(cn("px-2", "px-4")).toBe("px-4");
  });

  it("drops falsy conditionals", () => {
    const show = false as boolean;
    expect(cn("base", show && "hidden", null, undefined, "block")).toBe("base block");
  });
});

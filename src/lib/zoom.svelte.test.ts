import { describe, it, expect, beforeEach } from "vitest";
import { zoom } from "./zoom.svelte.js";

describe("zoom", () => {
  beforeEach(() => zoom.reset());

  it("clamps above the max of 3", () => {
    zoom.set(99);
    expect(zoom.factor).toBe(3);
    expect(zoom.canZoomIn).toBe(false);
  });

  it("clamps below the min of 0.5", () => {
    zoom.set(0);
    expect(zoom.factor).toBe(0.5);
    expect(zoom.canZoomOut).toBe(false);
  });

  it("steps by 0.1 in and out", () => {
    zoom.in();
    expect(zoom.factor).toBe(1.1);
    zoom.out();
    expect(zoom.factor).toBe(1);
  });

  it("reports percent", () => {
    zoom.set(1.5);
    expect(zoom.percent).toBe(150);
  });

  it("applies the factor to the document root", () => {
    zoom.set(1.2);
    expect(document.documentElement.style.zoom).toBe("1.2");
  });
});

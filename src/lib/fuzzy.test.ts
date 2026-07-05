import { describe, it, expect } from "vitest";
import { fuzzyMatch } from "./fuzzy.js";

describe("fuzzyMatch", () => {
  it("matches a subsequence and reports positions", () => {
    const m = fuzzyMatch("mtx", "main.tex");
    expect(m).not.toBeNull();
    expect(m!.positions.map((i) => "main.tex"[i]).join("")).toBe("mtx");
  });

  it("returns null when not a subsequence", () => {
    expect(fuzzyMatch("zzz", "main.tex")).toBeNull();
  });

  it("is case-insensitive", () => {
    expect(fuzzyMatch("MAIN", "main.tex")).not.toBeNull();
  });

  it("treats an empty query as a match", () => {
    const m = fuzzyMatch("", "anything");
    expect(m).toEqual({ score: 0, positions: [] });
  });

  it("ranks a contiguous run above the same letters scattered mid-word", () => {
    const contiguous = fuzzyMatch("abcde", "abcde.tex")!;
    const scattered = fuzzyMatch("abcde", "axbxcxdxe.tex")!;
    expect(contiguous.score).toBeGreaterThan(scattered.score);
  });

  it("prefers the shorter of two otherwise-equal matches", () => {
    const short = fuzzyMatch("ref", "ref.bib")!;
    const long = fuzzyMatch("ref", "ref-appendix-longname.bib")!;
    expect(short.score).toBeGreaterThan(long.score);
  });
});

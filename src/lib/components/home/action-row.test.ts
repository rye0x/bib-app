import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import ActionRow from "./action-row.svelte";

describe("ActionRow", () => {
  it("renders the label", () => {
    render(ActionRow, { props: { label: "New Project" } });
    expect(screen.getByText("New Project")).toBeInTheDocument();
  });

  it("renders shortcut and hint when provided", () => {
    render(ActionRow, { props: { label: "Open", hint: "/path/to/proj", shortcut: "⌘O" } });
    expect(screen.getByText("⌘O")).toBeInTheDocument();
    expect(screen.getByText("/path/to/proj")).toBeInTheDocument();
  });

  it("fires onclick when pressed", async () => {
    const onclick = vi.fn();
    render(ActionRow, { props: { label: "Go", onclick } });
    await fireEvent.click(screen.getByRole("button"));
    expect(onclick).toHaveBeenCalledOnce();
  });
});

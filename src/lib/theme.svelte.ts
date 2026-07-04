// Shared theme state. The initial `.dark` class is applied before paint by an
// inline script in app.html; this module keeps a reactive mirror of that state
// and is the single place that flips the class + persists the choice.

type Theme = "light" | "dark";

let current = $state<Theme>("light");

/** Read the current DOM state into the reactive store. Call once on mount. */
export function initTheme() {
  current = document.documentElement.classList.contains("dark") ? "dark" : "light";
}

export const theme = {
  get value(): Theme {
    return current;
  },
  get isDark(): boolean {
    return current === "dark";
  },
  set(next: Theme) {
    current = next;
    document.documentElement.classList.toggle("dark", next === "dark");
    try {
      localStorage.setItem("theme", next);
    } catch {
      /* storage unavailable — ignore */
    }
  },
  toggle() {
    this.set(current === "dark" ? "light" : "dark");
  },
};

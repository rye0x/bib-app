import "@testing-library/jest-dom/vitest";
import { afterEach, beforeEach } from "vitest";
import { cleanup } from "@testing-library/svelte";

// jsdom's localStorage isn't a complete Storage here, so back it with a simple
// in-memory implementation for deterministic persistence assertions.
function createStorage(): Storage {
  const map = new Map<string, string>();
  return {
    get length() {
      return map.size;
    },
    key: (i) => [...map.keys()][i] ?? null,
    getItem: (k) => (map.has(k) ? map.get(k)! : null),
    setItem: (k, v) => void map.set(k, String(v)),
    removeItem: (k) => void map.delete(k),
    clear: () => map.clear(),
  };
}

beforeEach(() => {
  Object.defineProperty(window, "localStorage", {
    value: createStorage(),
    configurable: true,
    writable: true,
  });
});

afterEach(() => {
  cleanup();
});

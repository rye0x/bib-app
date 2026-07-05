// Recently opened projects, persisted to localStorage (same idiom as
// theme/zoom). Call initRecents() once on mount to hydrate from storage.

export type Recent = { name: string; path: string; openedAt: number };

const KEY = "recent-projects";
const MAX = 8;

let items = $state<Recent[]>([]);

function persist() {
  try {
    localStorage.setItem(KEY, JSON.stringify(items));
  } catch {
    /* storage unavailable — ignore */
  }
}

/** Read persisted recents into the store. Call once on mount. */
export function initRecents() {
  try {
    const raw = localStorage.getItem(KEY);
    if (raw) items = JSON.parse(raw) as Recent[];
  } catch {
    /* corrupt or unavailable — ignore */
  }
}

export const recents = {
  get items(): Recent[] {
    return items;
  },
  add(path: string) {
    const name = path.split("/").pop() || path;
    items = [{ name, path, openedAt: Date.now() }, ...items.filter((r) => r.path !== path)].slice(
      0,
      MAX,
    );
    persist();
  },
  remove(path: string) {
    items = items.filter((r) => r.path !== path);
    persist();
  },
};

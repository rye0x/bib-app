// UI zoom, VS Code / Zed style: Cmd/Ctrl +, -, and 0 scale the whole interface.
// Implemented with CSS `zoom` on the document root so it works in any webview
// (the Tauri native zoom hotkeys are unreliable across platforms). The chosen
// level is persisted and re-applied before paint by an inline script in app.html.

const MIN = 0.5;
const MAX = 3;
const STEP = 0.1;

function clamp(v: number): number {
  return Math.min(MAX, Math.max(MIN, Math.round(v * 100) / 100));
}

let factor = $state(1);

function apply() {
  // `zoom` is non-standard but supported by WebKit (WKWebView) and Chromium.
  document.documentElement.style.zoom = String(factor);
  try {
    localStorage.setItem("zoom", String(factor));
  } catch {
    /* storage unavailable — ignore */
  }
}

/** Read the persisted zoom into the store and apply it. Call once on mount. */
export function initZoom() {
  try {
    const saved = parseFloat(localStorage.getItem("zoom") ?? "");
    if (!Number.isNaN(saved)) factor = clamp(saved);
  } catch {
    /* ignore */
  }
  document.documentElement.style.zoom = String(factor);
}

export const zoom = {
  get factor(): number {
    return factor;
  },
  get percent(): number {
    return Math.round(factor * 100);
  },
  get canZoomIn(): boolean {
    return factor < MAX;
  },
  get canZoomOut(): boolean {
    return factor > MIN;
  },
  set(v: number) {
    factor = clamp(v);
    apply();
  },
  in() {
    this.set(factor + STEP);
  },
  out() {
    this.set(factor - STEP);
  },
  reset() {
    this.set(1);
  },
};

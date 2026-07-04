// Injected into every E2E page BEFORE it loads so the SvelteKit SPA can boot
// without a real Tauri runtime. The frontend calls getCurrentWindow() at layout
// init and listen() on mount — both read window.__TAURI_INTERNALS__, which the
// browser doesn't have. This stubs the pieces @tauri-apps/api actually touches:
// metadata (getCurrentWindow), invoke, and transformCallback (event listeners).
//
// This runs in the browser context via page.addInitScript, so it must be fully
// self-contained — no imports or outer-scope references.
export function installTauriMock() {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const w = window as any;
  const calls: { cmd: string; args: unknown }[] = [];
  w.__TAURI_MOCK_CALLS__ = calls;

  let counter = 0;

  w.__TAURI_INTERNALS__ = {
    metadata: {
      currentWindow: { label: "main" },
      currentWebview: { windowLabel: "main", label: "main" },
    },
    plugins: {},
    transformCallback(callback: (v: unknown) => void, once = false) {
      const id = ++counter;
      const prop = `_${id}`;
      Object.defineProperty(w, prop, {
        value: (result: unknown) => {
          if (once) Reflect.deleteProperty(w, prop);
          return callback(result);
        },
        writable: true,
        configurable: true,
      });
      return id;
    },
    invoke(cmd: string, args: unknown) {
      calls.push({ cmd, args });
      // Event listeners expect a numeric event id back from the listen command.
      if (cmd === "plugin:event|listen") return Promise.resolve(++counter);
      return Promise.resolve(null);
    },
    convertFileSrc(src: string) {
      return src;
    },
  };
}

// Compile state for the open project: run the LaTeX build, hold the produced
// PDF, the raw log, and parsed diagnostics. Cross-component reactive state in
// the project's *.svelte.ts idiom — a module `$state` exposed through an object
// of getters/mutators. Compilation itself runs in Rust (`compile_project`).
import { invoke } from "@tauri-apps/api/core";
import { project } from "./project.svelte.js";

export type Diagnostic = {
  severity: "error" | "warning";
  file: string | null; // relative to the project root when known
  line: number | null;
  message: string;
  hint: string | null;
};

// Mirrors the Rust `CompileResult`.
export type CompileResult = {
  success: boolean;
  pdfPath: string | null;
  mainFile: string;
  log: string;
  diagnostics: Diagnostic[];
  durationMs: number;
};

export type CompileStatus = "idle" | "compiling" | "success" | "error";

const AUTO_BUILD_KEY = "auto-build";

let status = $state<CompileStatus>("idle");
let result = $state<CompileResult | null>(null);
let error = $state<string | null>(null); // spawn-level failure (no compiler, etc.)
let autoBuild = $state(false);
// Bumped after every build so the PDF viewer can bust the webview's cache and
// reload the freshly written file at the same path.
let pdfVersion = $state(0);

/** Read the persisted auto-build preference. Call once on mount. */
export function initCompile() {
  try {
    autoBuild = localStorage.getItem(AUTO_BUILD_KEY) === "true";
  } catch {
    /* storage unavailable — ignore */
  }
}

export const compile = {
  get status(): CompileStatus {
    return status;
  },
  get result(): CompileResult | null {
    return result;
  },
  get error(): string | null {
    return error;
  },
  get autoBuild(): boolean {
    return autoBuild;
  },
  get pdfVersion(): number {
    return pdfVersion;
  },
  get isCompiling(): boolean {
    return status === "compiling";
  },
  get diagnostics(): Diagnostic[] {
    return result?.diagnostics ?? [];
  },
  get errorCount(): number {
    return this.diagnostics.filter((d) => d.severity === "error").length;
  },
  get warningCount(): number {
    return this.diagnostics.filter((d) => d.severity === "warning").length;
  },

  /** Compile the project's main document. No-ops without a project or mid-build. */
  async compile() {
    const root = project.root;
    if (root === null || status === "compiling") return;
    status = "compiling";
    error = null;
    try {
      const res = await invoke<CompileResult>("compile_project", { root, mainFile: null });
      result = res;
      pdfVersion += 1;
      status = res.success ? "success" : "error";
    } catch (e) {
      error = typeof e === "string" ? e : String(e);
      status = "error";
    }
  },

  toggleAutoBuild() {
    autoBuild = !autoBuild;
    try {
      localStorage.setItem(AUTO_BUILD_KEY, String(autoBuild));
    } catch {
      /* storage unavailable — ignore */
    }
  },

  /** Reset when the project closes so stale output can't leak into a new one. */
  reset() {
    status = "idle";
    result = null;
    error = null;
    pdfVersion = 0;
  },
};

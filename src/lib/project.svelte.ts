// The open project: its root, the filtered source tree, and the currently
// open file with its editing/saved state. Cross-component reactive state in
// the project's *.svelte.ts idiom — a module `$state` exposed through an
// object of getters/mutators. File I/O goes through custom Rust commands.
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { recents } from "./recents.svelte.js";

export type TreeNode = {
  name: string;
  path: string;
  isDir: boolean;
  children?: TreeNode[];
};

// How a file opens: editable text, or a read-only preview (PDF / image).
export type FileKind = "text" | "pdf" | "image" | "other";

export function fileKind(path: string): FileKind {
  const p = path.toLowerCase();
  if (/\.(tex|bib|cls|sty)$/.test(p)) return "text";
  if (p.endsWith(".pdf")) return "pdf";
  if (/\.(png|jpe?g|gif|svg)$/.test(p)) return "image";
  return "other";
}

let root = $state<string | null>(null);
let tree = $state<TreeNode | null>(null);
let openPath = $state<string | null>(null);
let content = $state(""); // live editor doc — the source of truth for save
let savedContent = $state(""); // last-saved snapshot, for dirty tracking
let loading = $state(false);

export const project = {
  get root(): string | null {
    return root;
  },
  get tree(): TreeNode | null {
    return tree;
  },
  get openPath(): string | null {
    return openPath;
  },
  get content(): string {
    return content;
  },
  get loading(): boolean {
    return loading;
  },
  get isOpen(): boolean {
    return root !== null;
  },
  get isDirty(): boolean {
    return openPath !== null && content !== savedContent;
  },

  /** Prompt for a folder and open it as the project. */
  async openProjectDialog() {
    const picked = await open({ directory: true, multiple: false });
    if (typeof picked === "string") await this.openProject(picked);
  },

  async openProject(path: string) {
    loading = true;
    try {
      tree = await invoke<TreeNode>("read_project_tree", { root: path });
      root = path;
      openPath = null;
      content = "";
      savedContent = "";
      recents.add(path);
    } finally {
      loading = false;
    }
  },

  /**
   * Open a file. Text files load into the editor; PDFs and images open as a
   * read-only preview (rendered from the path via the asset protocol). Other
   * file types are ignored.
   */
  async openFile(path: string) {
    const kind = fileKind(path);
    if (kind === "text") {
      const text = await invoke<string>("read_text_file", { path });
      openPath = path;
      content = text;
      savedContent = text;
    } else if (kind === "pdf" || kind === "image") {
      openPath = path;
      content = "";
      savedContent = "";
    }
  },

  /** Called by the editor on every doc change. */
  setContent(next: string) {
    content = next;
  },

  async save() {
    if (openPath === null || content === savedContent) return;
    await invoke("write_text_file", { path: openPath, contents: content });
    savedContent = content;
  },

  close() {
    root = null;
    tree = null;
    openPath = null;
    content = "";
    savedContent = "";
  },
};

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

// A file flattened out of the tree, with its path relative to the project root
// — the shape the fuzzy finder and search overlays work against.
export type ProjectFile = {
  name: string;
  path: string; // absolute
  relPath: string; // relative to the project root
};

// One hit from a project-wide search, mirroring the Rust `SearchMatch`.
export type SearchMatch = {
  path: string;
  relPath: string;
  line: number;
  lineText: string;
  matchStart: number;
  matchLen: number;
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
// A one-shot request to reveal a line in the editor (from a search hit). The
// nonce lets the editor re-run the reveal even when the same line is requested
// twice in a row.
let reveal = $state<{ line: number; nonce: number } | null>(null);

// Depth-first flatten of the source tree into a relative-path file list.
function flatten(node: TreeNode, rootPath: string): ProjectFile[] {
  if (!node.isDir) {
    const rel = node.path.startsWith(rootPath)
      ? node.path.slice(rootPath.length).replace(/^\//, "")
      : node.path;
    return [{ name: node.name, path: node.path, relPath: rel }];
  }
  return (node.children ?? []).flatMap((c) => flatten(c, rootPath));
}

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
  /** Every source file in the project, flattened for quick-open and search. */
  get files(): ProjectFile[] {
    if (!tree || root === null) return [];
    return flatten(tree, root);
  },
  /** A pending request to scroll to / select a line, consumed by the editor. */
  get reveal(): { line: number; nonce: number } | null {
    return reveal;
  },

  /** Prompt for a folder and open it as the project. */
  async openProjectDialog() {
    const picked = await open({ directory: true, multiple: false });
    if (typeof picked === "string") await this.openProject(picked);
  },

  /**
   * Scaffold a new project folder (`parent/name`) with a starter `main.tex`
   * and open it. Returns the new project's root path.
   */
  async createProject(parent: string, name: string): Promise<string> {
    const newRoot = await invoke<string>("create_project", { parent, name });
    await this.openProject(newRoot);
    return newRoot;
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

  /**
   * Open a text file and reveal a specific 1-based line — used when jumping to
   * a project-search hit. No-ops for non-text files.
   */
  async openFileAt(path: string, line: number) {
    if (fileKind(path) !== "text") return;
    if (path !== openPath) await this.openFile(path);
    reveal = { line, nonce: (reveal?.nonce ?? 0) + 1 };
  },

  /** Case-insensitive plain-text search across the project's source files. */
  async searchProject(query: string): Promise<SearchMatch[]> {
    if (root === null || query.trim() === "") return [];
    return invoke<SearchMatch[]>("search_project", { root, query });
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

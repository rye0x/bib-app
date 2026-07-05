// LSP-style completion for LaTeX, wired into CodeMirror's autocomplete.
//
// Three sources, chosen by what's under the cursor:
//   1. Environments — after `\begin{` or `\end{` (the built-in list).
//   2. References  — after `\ref{`, `\eqref{`, `\pageref{`, `\autoref{`,
//      completed from every `\label{…}` in the current document.
//   3. Citations   — after `\cite…{`, completed from `\bibitem{…}` keys in the
//      current document.
//   4. Commands    — after a bare backslash, from a curated command list.
//
// The label/citation sources scan the live document, so completions stay in
// sync with what the author has written without a separate index.
import type { Completion, CompletionContext, CompletionResult } from "@codemirror/autocomplete";

// Curated set of common LaTeX commands. `apply` carries a snippet-ish default
// where a brace pair is helpful; the plain `label` is inserted otherwise.
const COMMANDS: Completion[] = [
  { label: "\\section", type: "keyword", info: "Top-level section" },
  { label: "\\subsection", type: "keyword", info: "Second-level section" },
  { label: "\\subsubsection", type: "keyword", info: "Third-level section" },
  { label: "\\paragraph", type: "keyword" },
  { label: "\\chapter", type: "keyword", info: "Chapter (book/report)" },
  { label: "\\textbf", type: "function", info: "Bold text" },
  { label: "\\textit", type: "function", info: "Italic text" },
  { label: "\\texttt", type: "function", info: "Monospace text" },
  { label: "\\emph", type: "function", info: "Emphasis" },
  { label: "\\underline", type: "function" },
  { label: "\\begin", type: "keyword", info: "Open an environment" },
  { label: "\\end", type: "keyword", info: "Close an environment" },
  { label: "\\item", type: "keyword", info: "List item" },
  { label: "\\label", type: "function", info: "Anchor for \\ref" },
  { label: "\\ref", type: "function", info: "Reference a label" },
  { label: "\\eqref", type: "function", info: "Reference an equation" },
  { label: "\\pageref", type: "function" },
  { label: "\\autoref", type: "function" },
  { label: "\\cite", type: "function", info: "Citation" },
  { label: "\\citep", type: "function", info: "Parenthetical citation" },
  { label: "\\citet", type: "function", info: "Textual citation" },
  { label: "\\footnote", type: "function" },
  { label: "\\caption", type: "function" },
  { label: "\\includegraphics", type: "function", info: "Insert an image" },
  { label: "\\usepackage", type: "keyword" },
  { label: "\\documentclass", type: "keyword" },
  { label: "\\newcommand", type: "keyword" },
  { label: "\\frac", type: "function", info: "Fraction" },
  { label: "\\sqrt", type: "function", info: "Square root" },
  { label: "\\sum", type: "function" },
  { label: "\\int", type: "function" },
  { label: "\\alpha", type: "constant" },
  { label: "\\beta", type: "constant" },
  { label: "\\gamma", type: "constant" },
  { label: "\\delta", type: "constant" },
  { label: "\\lambda", type: "constant" },
  { label: "\\mu", type: "constant" },
  { label: "\\pi", type: "constant" },
  { label: "\\sigma", type: "constant" },
  { label: "\\theta", type: "constant" },
];

// Common environments offered after `\begin{`.
const ENVIRONMENTS: Completion[] = [
  "document",
  "abstract",
  "itemize",
  "enumerate",
  "description",
  "figure",
  "table",
  "tabular",
  "equation",
  "align",
  "gather",
  "matrix",
  "cases",
  "center",
  "quote",
  "verbatim",
  "theorem",
  "proof",
  "lemma",
  "definition",
].map((name) => ({ label: name, type: "class" }));

// Pull every `\label{key}` out of the document for \ref-family completion.
function labelsIn(doc: string): Completion[] {
  const found = new Map<string, Completion>();
  const re = /\\label\{([^}]+)\}/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(doc)) !== null) {
    const key = m[1];
    if (!found.has(key)) found.set(key, { label: key, type: "variable", info: "label" });
  }
  return [...found.values()];
}

// Pull `\bibitem{key}` entries for \cite-family completion.
function citeKeysIn(doc: string): Completion[] {
  const found = new Map<string, Completion>();
  const re = /\\bibitem(?:\[[^\]]*\])?\{([^}]+)\}/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(doc)) !== null) {
    const key = m[1];
    if (!found.has(key)) found.set(key, { label: key, type: "variable", info: "citation" });
  }
  return [...found.values()];
}

/**
 * A single completion source covering commands, environments, refs and cites.
 * Returns null when nothing sensible applies so other sources (and the default
 * no-op) can take over.
 */
export function latexCompletions(context: CompletionContext): CompletionResult | null {
  const doc = context.state.doc.toString();

  // Inside `\begin{…}` / `\end{…}` → environment names.
  const brace = context.matchBefore(/\\(?:begin|end)\{([^}]*)$/);
  if (brace) {
    const from = brace.from + brace.text.indexOf("{") + 1;
    return { from, options: ENVIRONMENTS, validFor: /^[^}]*$/ };
  }

  // Inside a \ref-family argument → labels from the document.
  const ref = context.matchBefore(/\\(?:ref|eqref|pageref|autoref)\{([^}]*)$/);
  if (ref) {
    const from = ref.from + ref.text.indexOf("{") + 1;
    return { from, options: labelsIn(doc), validFor: /^[^}]*$/ };
  }

  // Inside a \cite-family argument → bibitem keys from the document.
  const cite = context.matchBefore(/\\cite[a-zA-Z]*(?:\[[^\]]*\])?\{([^}]*)$/);
  if (cite) {
    const from = cite.from + cite.text.lastIndexOf("{") + 1;
    return { from, options: citeKeysIn(doc), validFor: /^[^},]*$/ };
  }

  // A bare command: from the backslash onward.
  const cmd = context.matchBefore(/\\[a-zA-Z]*/);
  if (cmd) {
    // Don't pop up on an empty trigger unless explicitly requested.
    if (cmd.from === cmd.to && !context.explicit) return null;
    return { from: cmd.from, options: COMMANDS, validFor: /^\\[a-zA-Z]*$/ };
  }

  return null;
}

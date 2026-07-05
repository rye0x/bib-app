<script lang="ts">
  import { onMount } from "svelte";
  import { EditorView, keymap, lineNumbers, highlightActiveLine } from "@codemirror/view";
  import { EditorState, EditorSelection } from "@codemirror/state";
  import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
  import { search, searchKeymap, highlightSelectionMatches } from "@codemirror/search";
  import {
    autocompletion,
    completionKeymap,
    closeBrackets,
    closeBracketsKeymap,
  } from "@codemirror/autocomplete";
  import {
    StreamLanguage,
    syntaxHighlighting,
    defaultHighlightStyle,
    bracketMatching,
    indentOnInput,
  } from "@codemirror/language";
  import { stex } from "@codemirror/legacy-modes/mode/stex";
  import { project } from "$lib/project.svelte.js";
  import { latexCompletions } from "$lib/editor/latex-completion.js";

  let el = $state<HTMLDivElement>();
  let view: EditorView | undefined;
  // The file whose contents `view` currently holds, so we can tell a real file
  // switch apart from the per-keystroke self-echo of setContent.
  let loadedPath: string | null = null;

  // All language/editor config in one place so the LaTeX mode can be swapped
  // for a richer grammar later without touching the mount logic.
  function extensions() {
    return [
      lineNumbers(),
      history(),
      highlightActiveLine(),
      highlightSelectionMatches(),
      bracketMatching(),
      closeBrackets(),
      indentOnInput(),
      StreamLanguage.define(stex),
      syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
      // Find & replace (⌘F / ⌘⌥F), panel anchored to the top of the editor.
      search({ top: true }),
      // LSP-style LaTeX completion: commands, environments, refs, cites.
      autocompletion({ override: [latexCompletions], icons: false }),
      // No ⌘S here — Save is owned by the native File menu.
      keymap.of([
        ...closeBracketsKeymap,
        ...defaultKeymap,
        ...historyKeymap,
        ...searchKeymap,
        ...completionKeymap,
      ]),
      EditorView.updateListener.of((u) => {
        if (u.docChanged) project.setContent(u.state.doc.toString());
      }),
      EditorView.lineWrapping,
    ];
  }

  onMount(() => {
    view = new EditorView({
      parent: el!,
      state: EditorState.create({ doc: project.content, extensions: extensions() }),
    });
    loadedPath = project.openPath;
    return () => view?.destroy();
  });

  // Reload the document when a different file is opened. A fresh EditorState
  // resets the undo history too, so undo can never bleed across files. We only
  // do this on a real path change — never on the per-keystroke self-echo.
  $effect(() => {
    const path = project.openPath;
    const incoming = project.content;
    if (!view) return;
    if (path !== loadedPath) {
      view.setState(EditorState.create({ doc: incoming, extensions: extensions() }));
      loadedPath = path;
    } else if (view.state.doc.toString() !== incoming) {
      view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: incoming } });
    }
  });

  // Reveal a line requested from a search hit: select it and scroll it to the
  // center. The nonce makes repeat requests for the same line re-fire.
  $effect(() => {
    const target = project.reveal;
    if (!view || !target) return;
    const doc = view.state.doc;
    const lineNo = Math.max(1, Math.min(target.line, doc.lines));
    const line = doc.line(lineNo);
    view.dispatch({
      selection: EditorSelection.cursor(line.from),
      effects: EditorView.scrollIntoView(line.from, { y: "center" }),
    });
    view.focus();
  });
</script>

<div bind:this={el} class="cm-host h-full w-full overflow-hidden"></div>

<style>
  .cm-host :global(.cm-editor) {
    height: 100%;
  }
  .cm-host :global(.cm-scroller) {
    font-family: ui-monospace, "SF Mono", "Cascadia Code", "JetBrains Mono", Menlo, monospace;
    font-size: 13px;
    line-height: 1.6;
  }
  .cm-host :global(.cm-editor.cm-focused) {
    outline: none;
  }
  .cm-host :global(.cm-gutters) {
    background: transparent;
    border-right: 1px solid var(--border);
    color: var(--muted-foreground);
  }
  .cm-host :global(.cm-activeLine),
  .cm-host :global(.cm-activeLineGutter) {
    background: color-mix(in oklch, var(--accent) 50%, transparent);
  }

  /* Search panel + completion tooltip: match the app's theme tokens instead of
     CodeMirror's default light chrome, so they read right in dark mode too. */
  .cm-host :global(.cm-panels) {
    background: var(--card);
    color: var(--card-foreground);
    border-bottom: 1px solid var(--border);
  }
  .cm-host :global(.cm-panel.cm-search input),
  .cm-host :global(.cm-panel.cm-search button),
  .cm-host :global(.cm-panel.cm-search label) {
    font-size: 12px;
  }
  .cm-host :global(.cm-panel.cm-search input) {
    background: var(--background);
    color: var(--foreground);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 2px 6px;
  }
  .cm-host :global(.cm-tooltip.cm-tooltip-autocomplete) {
    background: var(--popover);
    color: var(--popover-foreground);
    border: 1px solid var(--border);
    border-radius: 6px;
    box-shadow: 0 4px 16px rgb(0 0 0 / 0.18);
  }
  .cm-host :global(.cm-tooltip-autocomplete ul li[aria-selected]) {
    background: var(--accent);
    color: var(--accent-foreground);
  }
  .cm-host :global(.cm-searchMatch) {
    background: color-mix(in oklch, var(--primary) 30%, transparent);
  }
  .cm-host :global(.cm-searchMatch-selected) {
    background: color-mix(in oklch, var(--primary) 55%, transparent);
  }
  .cm-host :global(.cm-selectionMatch) {
    background: color-mix(in oklch, var(--accent) 70%, transparent);
  }
</style>

<script lang="ts">
  import { onMount } from "svelte";
  import { EditorView, keymap, lineNumbers, highlightActiveLine } from "@codemirror/view";
  import { EditorState } from "@codemirror/state";
  import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
  import {
    StreamLanguage,
    syntaxHighlighting,
    defaultHighlightStyle,
    bracketMatching,
    indentOnInput,
  } from "@codemirror/language";
  import { stex } from "@codemirror/legacy-modes/mode/stex";
  import { project } from "$lib/project.svelte.js";

  let el = $state<HTMLDivElement>();
  let view: EditorView | undefined;

  // All language/editor config in one place so the LaTeX mode can be swapped
  // for a richer grammar later without touching the mount logic.
  function extensions() {
    return [
      lineNumbers(),
      history(),
      highlightActiveLine(),
      bracketMatching(),
      indentOnInput(),
      StreamLanguage.define(stex),
      syntaxHighlighting(defaultHighlightStyle, { fallback: true }),
      // No ⌘S here — Save is owned by the native File menu.
      keymap.of([...defaultKeymap, ...historyKeymap]),
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
    return () => view?.destroy();
  });

  // Reload the document when a different file is opened. Guard against the
  // per-keystroke self-echo from setContent so we only replace on a real
  // file switch and never clobber the cursor mid-edit.
  $effect(() => {
    // track the open file and its loaded content
    void project.openPath;
    const incoming = project.content;
    if (!view) return;
    if (view.state.doc.toString() !== incoming) {
      view.dispatch({
        changes: { from: 0, to: view.state.doc.length, insert: incoming },
      });
    }
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
</style>

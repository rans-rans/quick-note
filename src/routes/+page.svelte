<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import {
    handleFileDrop,
    handleSave,
    changeWindowTitle,
  } from "./window_actions";

  let docText = $state(null as string | null);
  let docProps = $state({
    path: null as string | null,
    originalText: null as string | null,
  });

  let fileModified = $derived.by(() => {
    if (
      docProps.originalText === null &&
      (docText === "" || docText === null)
    ) {
      return false;
    }
    const modified = docText !== docProps.originalText;
    return modified;
  });

  // An effect to update the window title based on the document's modified state
  $effect(() => {
    const fileName = docProps.path
      ? docProps.path.split("/").pop()
      : "Untitled Document";
    if (fileModified) {
      changeWindowTitle(`${fileName}*`);
    } else {
      changeWindowTitle(fileName ?? "Untitled Document");
    }
  });

  async function onSave(e: KeyboardEvent) {
    if (
      e.key === "s" &&
      (e.metaKey || e.ctrlKey) &&
      fileModified &&
      docText !== null
    ) {
      e.preventDefault();
      handleSave(
        docText ?? "",
        () => {
          docProps.originalText = docText;
        },
        docProps.path ?? undefined,
      );
    }
  }

  onMount(() => {
    // Listen for the "save" event from the main process
    const listenToSaveButton = listen("save", (_) => {
      onSave(
        new KeyboardEvent("keydown", {
          key: "s",
          ctrlKey: true,
        }),
      );
    });

    // Listen for file drop events for .txt files
    const listenToFileDrop = listen("tauri://drag-drop", async (_) => {
      handleFileDrop((filePath, content) => {
        docText = content;
        docProps.path = filePath;
        docProps.originalText = content;
      });
    });

    return () => {
      listenToSaveButton.then((unlisten) => unlisten());
      listenToFileDrop.then((unlisten) => unlisten());
    };
  });
</script>

<svelte:window on:keydown={onSave} />
<div>
  <textarea class="editor-textarea" bind:value={docText}
  ></textarea>
</div>

<style>
  .editor-textarea {
    display: block;
    width: 100%;
    height: 100vh;
    height: 100dvh;
    border: none;
    resize: none;
    padding: 0.5rem;
    font-size: 16px;
    line-height: 1.5;
    outline: none;
  }

  .editor-textarea:focus {
    outline: none;
  }
</style>

<script lang="ts">
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import {
    handleFileOpen,
    handleSave,
    changeWindowTitle,
  } from "./window_actions";
  import { createHistoryStore } from "./history";

  let docText = $state(null as string | null);
  let timer: any; // 500ms debounce timer
  let docProps = $state({
    path: null as string | null,
    originalText: null as string | null,
  });

  let fileModified = $derived.by(() => {
    if (docProps.originalText === null && !docText) return false;
    return docText !== docProps.originalText;
  });

  // svelte-ignore state_referenced_locally
  const doc = createHistoryStore(docText);

  // An effect to update the window title based on the document's modified state
  $effect(() => {
    try {
      const fileName = docProps.path
        ? docProps.path.split("/").pop()
        : "Untitled Document";
      if (fileModified) {
        changeWindowTitle(`${fileName}*`);
      } else {
        changeWindowTitle(fileName ?? "Untitled Document");
      }
    } catch (error) {
      console.error("Error updating window title:", error);
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
      const savedText = docText;
      await handleSave(
        savedText,
        (savedPath) => {
          docProps.originalText = savedText;
          docProps.path = savedPath;
        },
        docProps.path ?? undefined,
      );
    }
  }

  function handleInput(e: Event) {
    const target = e.target as HTMLTextAreaElement;

    // Debounce: Wait 500ms after user stops typing before saving state
    clearTimeout(timer);
    timer = setTimeout(() => {
      doc.push(target.value);
    }, 450);
  }

  function handleKeyDown(e: KeyboardEvent) {
    const isMod = e.ctrlKey || e.metaKey; // Ctrl on Win, Cmd on Mac

    if (isMod && e.key.toLowerCase() === "z") {
      e.preventDefault();
      if (e.shiftKey) {
        doc.redo();
      } else {
        doc.undo();
      }
      docText = doc.state();
    } else if (isMod && e.key.toLowerCase() === "y") {
      e.preventDefault();
      doc.redo();
      docText = doc.state();
    } else if (isMod && e.key.toLowerCase() === "s") {
      onSave(e);
    }
  }

  onMount(() => {
    // Check if there's a pending file to open from command line arguments
    (async () => {
      try {
        const pendingFile = await invoke<string | null>("get_pending_file");
        if (pendingFile) {
          handleFileOpen(pendingFile, (filePath, content) => {
            docText = content;
            docProps = { ...docProps, path: filePath, originalText: content };
            doc.reset(content);
          });
        }
      } catch (error) {
        alert("Error loading file. Please try opening the file manually.");
      }
    })();

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
    const listenToFileDrop = listen("tauri://drag-drop", async (event) => {
      try {
        const payload = event.payload as Record<string, unknown>;
        if (!payload.paths || !(payload.paths as string[]).length) {
          return;
        }

        const filePath = (payload.paths as string[])[0];
        if (!filePath || !filePath.endsWith(".txt")) {
          return;
        }
        handleFileOpen(filePath, (filePath, content) => {
          docText = content;
          docProps.path = filePath;
          docProps.originalText = content;
          doc.reset(content);
        });
      } catch (error) {
        alert("Error handling dropped file");
      }
    });

    return () => {
      listenToSaveButton.then((unlisten) => unlisten());
      listenToFileDrop.then((unlisten) => unlisten());
    };
  });
</script>

<svelte:window on:keydown={handleKeyDown} />
<div>
  <textarea class="editor-textarea" bind:value={docText} oninput={handleInput}
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

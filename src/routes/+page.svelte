<script lang="ts">
  import { save } from "@tauri-apps/plugin-dialog";
  import {
    writeTextFile,
    BaseDirectory,
    readTextFileLines,
  } from "@tauri-apps/plugin-fs";
  import CustomCaretTextarea from "./components/CustomCaretTextarea.svelte";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import { documentDir } from "@tauri-apps/api/path";

  let docText = $state("");

  async function saveDocument(e: KeyboardEvent) {
    if (e.key === "s" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      if (docText.length === 0) {
        return;
      }
      const docsDir: string = await documentDir();
      const path = await save({
        title: "Save Document",
        defaultPath: docsDir,
        filters: [
          {
            name: "Text Files",
            extensions: ["txt"],
          },
        ],
      });
      if (path === null) return;
      const normalizedPath = path.toLowerCase().endsWith(".txt")
        ? path
        : `${path}.txt`;

      await writeTextFile(normalizedPath, docText, {
        baseDir: BaseDirectory.Home,
      });
    }
  }

  onMount(() => {
    // Listen for the "save" event from the main process
    const listenToSaveButton = listen("save", (_) => {
      console.log("Received save-doc event, triggering saveDocument");
      saveDocument(
        new KeyboardEvent("keydown", {
          key: "s",
          ctrlKey: true,
        }),
      );
    });

    // Listen for file drop events for .txt files
    const listenToFileDrop = listen("tauri://drag-drop", (event) => {
      const payload = event.payload as Record<string, unknown>;
      const filePath = (payload.paths as string[])[0];
      if (!filePath || !filePath.endsWith(".txt")) return;
      docText = "";
      readTextFileLines(filePath).then(async (lines) => {
        for await (const line of lines) {
          docText += line + "\n";
        }
      });
    });

    return () => {
      listenToSaveButton.then((unlisten) => unlisten());
      listenToFileDrop.then((unlisten) => unlisten());
    };
  });
</script>

<svelte:window on:keydown={saveDocument} />
<div>
  <CustomCaretTextarea bind:value={docText} />
</div>

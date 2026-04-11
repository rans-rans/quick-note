<script lang="ts">
  import { save } from "@tauri-apps/plugin-dialog";
  import { writeTextFile,BaseDirectory } from "@tauri-apps/plugin-fs";
  import CustomCaretTextarea from "./components/CustomCaretTextarea.svelte";

  let docText = $state("");

  async function saveDocument(e: KeyboardEvent) {
    if (e.key === "s" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      if (docText.length === 0) {
        return;
      }
      const path = await save({
        title: "Save Document",
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

      await writeTextFile(normalizedPath, docText,{
        baseDir: BaseDirectory.Document,
      });

    }
  }
</script>

<svelte:window on:keydown={saveDocument} />
<div>
  <CustomCaretTextarea bind:value={docText} />
</div>

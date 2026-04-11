import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { BaseDirectory, documentDir } from "@tauri-apps/api/path";
import { save } from "@tauri-apps/plugin-dialog";
import { readTextFileLines, writeTextFile } from "@tauri-apps/plugin-fs";

async function changeWindowTitle(newTitle: string) {
    try {
        await invoke("update_title", { title: newTitle });
    } catch (e) {
        console.error("Failed to change title:", e);
    }
}

async function handleSave(docText: string) {
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

async function handleFileDrop(
    onDone: (filePath: string, content: string) => void,
) {
    listen("tauri://drag-drop", async (event) => {
        const payload = event.payload as Record<string, unknown>;
        if (!payload.paths || !(payload.paths as string[]).length) {
            return;
        }
        const filePath = (payload.paths as string[])[0];
        if (!filePath || !filePath.endsWith(".txt")) {
            return;
        }

        let documentData = "";
        const lines = await readTextFileLines(filePath);
        for await (const line of lines) {
            documentData += line + "\n";
        }

        const docTitle = filePath.split("/").pop() || "Untitled Document";

        onDone(filePath, documentData);
        changeWindowTitle(docTitle);
    });
}

export { changeWindowTitle, handleFileDrop, handleSave };

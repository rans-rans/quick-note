import { invoke } from "@tauri-apps/api/core";
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

async function handleSave(
    docText: string,
    onDone: () => void,
    filePath?: string,
) {
    if (docText.length === 0) {
        return;
    }
    const docsDir: string = await documentDir();

    const path = filePath ?? await save({
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
    onDone();
}

async function handleFileOpen(
    filePath: string,
    onDone: (filePath: string, content: string) => void,
) {
    try {
        let documentData = "";
        const lines = await readTextFileLines(filePath);
        for await (const line of lines) {
            documentData += line + "\n";
        }

        console.log(`here is your new title ${filePath}`);
        const docTitle = filePath.split("/").pop() || "Untitled Document";

        onDone(filePath, documentData);
        changeWindowTitle(docTitle);
    } catch (error) {
        alert("Error opening file. Check if file exists");
    }
}

export { changeWindowTitle, handleFileOpen, handleSave };

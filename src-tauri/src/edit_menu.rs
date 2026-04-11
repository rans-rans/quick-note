use tauri::menu::{MenuEvent, MenuItem, Submenu};
use tauri::{menu::SubmenuBuilder, App, Runtime};
use tauri::{AppHandle, Manager, Result};
use tauri_plugin_clipboard_manager::ClipboardExt;

pub fn create_edit_menu<R: Runtime>(app: &mut App<R>) -> Result<Submenu<R>> {
    SubmenuBuilder::new(app, "Edit")
        .item(&MenuItem::with_id(
            app,
            "undo",
            "Undo",
            true,
            Some("CmdOrCtrl+Z"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "redo",
            "Redo",
            true,
            Some("CmdOrCtrl+Y"),
        )?)
        .separator()
        .item(&MenuItem::with_id(
            app,
            "cut",
            "Cut",
            true,
            Some("CmdOrCtrl+X"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "copy",
            "Copy",
            true,
            Some("CmdOrCtrl+C"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "paste",
            "Paste",
            true,
            Some("CmdOrCtrl+V"),
        )?)
        .item(&MenuItem::with_id(
            app,
            "select_all",
            "Select All",
            true,
            Some("CmdOrCtrl+A"),
        )?)
        .build()
}

pub fn handle_edit_menu_event<R: Runtime>(app: &AppHandle<R>, event: &MenuEvent) {
    if event.id().as_ref() == "paste" {
        let app_handle = app.clone();

        tauri::async_runtime::spawn(async move {
            // Reading the clipboard synchronously in this callback can block on Linux
            // when our own webview is clipboard owner. Do it off the callback thread.
            let Ok(clipboard_text) = app_handle.clipboard().read_text() else {
                return;
            };

            // Window might not be ready on initial app startup; retry a few times.
            let mut window = Option::None;
            for attempt in 0..5 {
                if let Some(w) = app_handle.get_webview_window("main") {
                    window = Some(w);
                    break;
                }
                if attempt < 4 {
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            }

            let Some(window) = window else {
                return;
            };
            let Ok(js_text) = serde_json::to_string(&clipboard_text) else {
                return;
            };

            let js = format!(
                "(function() {{\n  const text = {js_text};\n  const active = document.activeElement;\n  const isTextInput = active && (active.tagName === 'TEXTAREA' || (active.tagName === 'INPUT' && /^(text|search|url|tel|password|email)$/i.test(active.type)));\n\n  if (active && active.isContentEditable) {{\n    document.execCommand('insertText', false, text);\n    return;\n  }}\n\n  if (isTextInput) {{\n    const start = active.selectionStart ?? active.value.length;\n    const end = active.selectionEnd ?? active.value.length;\n    active.setRangeText(text, start, end, 'end');\n    active.dispatchEvent(new Event('input', {{ bubbles: true }}));\n    return;\n  }}\n\n  const editor = document.querySelector('[contenteditable]');\n  if (editor) {{\n    editor.focus();\n    document.execCommand('insertText', false, text);\n  }}\n}})();"
            );
            let _ = window.eval(&js);
        });
        return;
    }

    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    let js = match event.id().as_ref() {
        "undo" => "document.execCommand('undo')",
        "redo" => "document.execCommand('redo')",
        "cut" => "document.execCommand('cut')",
        "copy" => "document.execCommand('copy')",
        "select_all" => "document.querySelector('[contenteditable]')?.focus(); document.execCommand('selectAll')",
        _ => "",
    };
    if js != "" {
        let _ = window.eval(js);
    }
}

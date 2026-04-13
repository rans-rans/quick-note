use tauri::menu::{MenuEvent, MenuItem, Submenu};
use tauri::{menu::SubmenuBuilder, App, Runtime};
use tauri::{AppHandle, Manager, Result};

pub fn create_edit_menu<R: Runtime>(app: &mut App<R>) -> Result<Submenu<R>> {
    SubmenuBuilder::new(app, "Edit")
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
    let Some(window) = app.get_webview_window("main") else {
        return;
    };

    let js = match event.id().as_ref() {
        "cut" => "document.execCommand('cut')",
        "paste" => "document.execCommand('paste')",
        "copy" => "document.execCommand('copy')",
        "select_all" => "document.querySelector('[contenteditable]')?.focus(); document.execCommand('selectAll')",
        _ => "",
    };
    if js != "" {
        let _ = window.eval(js);
    }
}

use tauri::menu::{MenuEvent, Submenu};
use tauri::{image::Image, include_image, menu::SubmenuBuilder, App, Runtime};
use tauri::{AppHandle, Emitter, Result};

pub fn create_file_menu<R: Runtime>(app: &mut App<R>) -> Result<Submenu<R>> {
    let file_menu_icon = Image::from(include_image!("icons/file.ico"));

    SubmenuBuilder::new(app, "File")
        .submenu_icon(file_menu_icon)
        .text("new", "New")
        .text("open", "Open")
        .text("save", "Save")
        .build()
}

#[tauri::command]
pub fn handle_file_menu_events<R: Runtime>(app: &AppHandle<R>, event: &MenuEvent) {
    if event.id().as_ref() == "save" {
        let _ = app.emit("save", "");
    }
}

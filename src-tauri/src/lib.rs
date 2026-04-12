mod app_menu;
mod edit_menu;
mod file_menu;
mod file_open_listener;

#[tauri::command]
fn update_title(window: tauri::Window, title: String) {
    window.set_title(&title).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // This is need so that the title bar will update when the document changes
    if cfg!(target_os = "linux") {
        std::env::set_var("GDK_BACKEND", "x11");
    }
    tauri::Builder::default()
        .on_menu_event(app_menu::handle_menu_event)
        .setup(|app| {
            app_menu::setup_menu(app)?;
            file_open_listener::handle_file_open(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            update_title,
            file_open_listener::get_pending_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

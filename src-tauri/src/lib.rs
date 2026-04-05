use tauri::{
    image::Image,
    include_image,
    menu::{MenuBuilder, SubmenuBuilder},
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let file_menu_icon = Image::from(include_image!("icons/file.ico"));
            let file_menu = SubmenuBuilder::new(app, "File")
                .submenu_icon(file_menu_icon)
                .text("new", "New")
                .text("open", "Open")
                .text("save", "Save")
                .text("save_as", "Save As")
                .build()?;
            let menu = MenuBuilder::new(app)
                .items(&[&file_menu])
                .text("edit", "Edit")
                .text("view", "View")
                .text("help", "Help")
                .build()?;
            app.set_menu(menu)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use tauri::{
    menu::{MenuBuilder, MenuEvent},
    App, AppHandle, Runtime,
};

use crate::{edit_menu, file_menu};

pub fn setup_menu<R: Runtime>(app: &mut App<R>) -> tauri::Result<()> {
    let file_menu = file_menu::create_file_menu(app)?;
    let edit_menu = edit_menu::create_edit_menu(app)?;

    let menu = MenuBuilder::new(app)
        .items(&[&file_menu, &edit_menu])
        .text("view", "View")
        .text("help", "Help")
        .build()?;
    app.set_menu(menu)?;
    Ok(())
}

pub fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, event: MenuEvent) {
    file_menu::handle_file_menu_events(app, &event);
    edit_menu::handle_edit_menu_event(app, &event);
}

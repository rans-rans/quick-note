use std::sync::Mutex;
use tauri::{Manager, Runtime};

pub struct FileOpenState {
    pub pending_file: Mutex<Option<String>>,
}

#[tauri::command]
pub fn get_pending_file(state: tauri::State<FileOpenState>) -> Option<String> {
    let mut pending = state.pending_file.lock().unwrap();
    pending.take()
}

pub fn handle_file_open<R: Runtime>(
    app: &mut tauri::App<R>,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args: Vec<String> = std::env::args().collect();
    // args[0] is the executable, args[1] is usually the file path
    let file_path = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    // Store the file path in app state so the frontend can retrieve it
    app.manage(FileOpenState {
        pending_file: Mutex::new(file_path),
    });

    Ok(())
}

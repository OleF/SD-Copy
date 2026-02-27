// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod import;
mod types;
mod utils;
mod volume;

use import::ImportManager;
use std::sync::Mutex;
use tauri::{Manager, State};
use types::ScanResult;

struct AppState {
    import_manager: Mutex<ImportManager>,
}

#[tauri::command]
fn list_volumes() -> Result<Vec<String>, String> {
    volume::list_volumes()
}

#[tauri::command]
fn scan_volume_for_images(volume_path: String) -> Result<ScanResult, String> {
    volume::scan_volume_for_images(&volume_path)
}

#[tauri::command]
fn start_import(
    app: tauri::AppHandle,
    state: State<AppState>,
    volume_path: String,
    destination_root: String,
    folder_name: String,
) -> Result<String, String> {
    let import_id = uuid::Uuid::new_v4().to_string();

    let manager = state.import_manager.lock().unwrap();
    manager.start_import(
        app,
        import_id.clone(),
        volume_path,
        destination_root,
        folder_name,
    );

    Ok(import_id)
}

#[tauri::command]
fn cancel_import(state: State<AppState>, import_id: String) -> Result<(), String> {
    let manager = state.import_manager.lock().unwrap();
    manager.cancel_import(&import_id)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            app.manage(AppState {
                import_manager: Mutex::new(ImportManager::new()),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_volumes,
            scan_volume_for_images,
            start_import,
            cancel_import,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


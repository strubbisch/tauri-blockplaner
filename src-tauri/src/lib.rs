mod import;
mod state;

use blockplaner_core::data::AppData;
use state::AppState;

#[tauri::command]
fn get_app_data(app_state: tauri::State<AppState>) -> Result<String, String> {
    let app_data = app_state
        .app_data
        .lock()
        .map_err(|e| format!("Failed to lock app_data: {}", e))?;

    serde_json::to_string(&*app_data).map_err(|e| format!("Failed to serialize app_data: {}", e))
}

#[tauri::command]
fn clear_app_data(app_state: tauri::State<AppState>) -> Result<(), String> {
    let mut app_data = app_state
        .app_data
        .lock()
        .map_err(|e| format!("Failed to lock app_data: {}", e))?;

    *app_data = AppData::default();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_app_data,
            import::import_xml,
            import::import_json,
            clear_app_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

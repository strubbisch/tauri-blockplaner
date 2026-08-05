mod state;

use blockplaner_core::import::{import_from_xml, import_from_json};
use blockplaner_core::data::AppData;
use state::AppState;
use std::path::PathBuf;

#[tauri::command]
fn get_app_data(app_state: tauri::State<AppState>) -> Result<String, String> {
    let app_data = app_state.app_data.lock()
        .map_err(|e| format!("Failed to lock app_data: {}", e))?;
    
    serde_json::to_string(&*app_data)
        .map_err(|e| format!("Failed to serialize app_data: {}", e))
}

#[tauri::command]
fn import_xml(file_path: String, app_state: tauri::State<AppState>) -> Result<String, String> {
    let path = PathBuf::from(&file_path);
    match import_from_xml(path) {
        Ok(import_result) => {
            let mut app_data = app_state.app_data.lock()
                .map_err(|e| format!("Failed to lock app_data: {}", e))?;
            
            app_data.course_groups = import_result.course_groups;
            app_data.students = import_result.students;
            
            match serde_json::to_string(&*app_data) {
                Ok(json) => Ok(json),
                Err(e) => Err(format!("Failed to serialize result: {}", e)),
            }
        }
        Err(e) => Err(format!("Import failed: {}", e)),
    }
}

#[tauri::command]
fn import_json(file_path: String, app_state: tauri::State<AppState>) -> Result<String, String> {
    let path = PathBuf::from(&file_path);
    match import_from_json(path) {
        Ok(new_app_data) => {
            let mut app_data = app_state.app_data.lock()
                .map_err(|e| format!("Failed to lock app_data: {}", e))?;
            
            *app_data = new_app_data;
            
            match serde_json::to_string(&*app_data) {
                Ok(json) => Ok(json),
                Err(e) => Err(format!("Failed to serialize result: {}", e)),
            }
        }
        Err(e) => Err(format!("Import failed: {}", e)),
    }
}

#[tauri::command]
fn clear_app_data(app_state: tauri::State<AppState>) -> Result<(), String> {
    let mut app_data = app_state.app_data.lock()
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
            import_xml,
            import_json,
            clear_app_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

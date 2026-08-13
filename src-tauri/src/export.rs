use std::path::PathBuf;

use blockplaner_core::{data::AppData, export::export_to_json};

use crate::state::AppState;

#[tauri::command]
pub fn export_json(
    file_path: String,
    app_state: tauri::State<AppState>
) -> Result<AppData, String> {
    let path = PathBuf::from(&file_path);
    let app_data = app_state.app_data.lock().unwrap();
    
    println!("[DEBUG] Exporting to: {}", file_path);
    println!("[DEBUG] Current app_data has {} course_groups", app_data.course_groups.len());
    
    match export_to_json(path, &app_data) {
        Ok(_) => {
            let result = app_data.clone();
            println!("[DEBUG] Export successful, returning {} course_groups", result.course_groups.len());
            Ok(result)
        },
        Err(e) => {
            eprintln!("[ERROR] Export failed: {}", e);
            Err(format!("Fehler beim Exportieren der Daten: {e}"))
        }
    }
}
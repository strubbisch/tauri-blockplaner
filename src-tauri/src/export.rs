use std::path::PathBuf;

use blockplaner_core::export::export_to_json;

use crate::state::AppState;

#[tauri::command]
pub fn export_json(file_path: String, app_state: tauri::State<AppState>) -> Result<(), String> {
    let path = PathBuf::from(&file_path);
    let app_data = app_state.app_data.lock().unwrap();

    match export_to_json(path, &app_data) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Fehler beim Exportieren der Daten: {e}")),
    }
}

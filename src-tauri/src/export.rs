use blockplaner_core::export::{export_to_json, export_to_pdf};

use crate::state::AppState;

#[tauri::command]
pub fn export_json(file_path: String, app_state: tauri::State<AppState>) -> Result<(), String> {
    let app_data = app_state.app_data.lock().unwrap();

    export_to_json(file_path, &app_data)
        .map_err(|e| format!("Fehler beim Exportieren der Daten: {e}"))
}

#[tauri::command]
pub fn export_pdf(file_path: String, app_state: tauri::State<AppState>) -> Result<(), String> {
    let app_data = &app_state.app_data;

    export_to_pdf(file_path, &mut *app_data.lock().unwrap())
        .map_err(|e| format!("Fehler beim Exportieren der Daten: {e}"))
}

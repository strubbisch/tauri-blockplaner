use std::path::PathBuf;

use blockplaner_core::{
    data::AppData,
    import::{import_from_json, import_from_xml},
};

use crate::state::AppState;

#[tauri::command]
pub fn import_xml(file_path: String, app_state: tauri::State<AppState>) -> Result<AppData, String> {
    let path = PathBuf::from(&file_path);
    match import_from_xml(path) {
        Ok(import_result) => {
            let mut app_data = app_state
                .app_data
                .lock()
                .map_err(|e| format!("Failed to lock app_data: {}", e))?;

            app_data.course_groups = import_result.course_groups;
            app_data.students = import_result.students;

            Ok(app_data.clone())
        }
        Err(e) => Err(format!("Import fehlgeschlagen: {}", e)),
    }
}

#[tauri::command]
pub fn import_json(
    file_path: String,
    app_state: tauri::State<AppState>,
) -> Result<AppData, String> {
    let path = PathBuf::from(&file_path);
    match import_from_json(path) {
        Ok(new_app_data) => {
            let mut app_data = app_state
                .app_data
                .lock()
                .map_err(|e| format!("Failed to lock app_data: {}", e))?;

            *app_data = new_app_data;

            Ok(app_data.clone())
        }
        Err(e) => Err(format!("Import fehlgeschlagen: {}", e)),
    }
}

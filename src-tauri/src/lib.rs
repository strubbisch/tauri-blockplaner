mod course;
mod export;
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

#[tauri::command]
fn update_app_settings(
    app_state: tauri::State<AppState>,
    number_of_blocks: Option<u8>,
    total_hours_per_week: Option<u8>,
    max_course_students_difference: Option<u8>,
    max_students_per_course: Option<u8>,
) -> Result<(), String> {
    let mut app_data = app_state
        .app_data
        .lock()
        .map_err(|e| format!("Failed to lock app_data: {}", e))?;

    if let Some(value) = number_of_blocks {
        app_data.number_of_blocks = value;
    }
    if let Some(value) = total_hours_per_week {
        app_data.total_hours_per_week = value;
    }
    if let Some(value) = max_course_students_difference {
        app_data.max_course_students_difference = value;
    }
    if let Some(value) = max_students_per_course {
        app_data.max_students_per_course = value;
    }

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
            clear_app_data,
            update_app_settings,
            import::import_xml,
            import::import_json,
            course::update_course,
            course::create_parallel_course,
            course::delete_course,
            course::create_courses,
            export::export_json,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

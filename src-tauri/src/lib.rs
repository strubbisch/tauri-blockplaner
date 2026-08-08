mod import;
mod state;

use blockplaner_core::{data::AppData, model::course::Course};
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
fn update_course(
    course_id: usize,
    teacher: String,
    room: String,
    hours_per_week: u8,
    app_state: tauri::State<AppState>,
) -> Result<AppData, String> {
    let mut app_data = app_state
        .app_data
        .lock()
        .map_err(|e| format!("Failed to lock app_data: {}", e))?;

    let course = app_data
        .courses
        .iter_mut()
        .find(|c| c.id == course_id)
        .ok_or_else(|| format!("Kurs mit der ID {course_id} nicht gefunden!"))?;

    course.hours_per_week = hours_per_week;
    if teacher.is_empty() {
        course.teacher = None
    } else {
        course.teacher = Some(teacher)
    }

    if room.is_empty() {
        course.room = None
    } else {
        course.room = Some(room)
    }

    Ok(app_data.clone())
}

#[tauri::command]
fn create_parallel_course(
    course_group_id: usize,
    hours_per_week: u8,
    app_state: tauri::State<AppState>,
) -> Result<AppData, String> {
    let mut app_data = app_state.app_data.lock().unwrap();

    let next_id = app_data
        .courses
        .iter()
        .map(|c| c.id)
        .max()
        .unwrap_or(0)
        + 1;
    let parallel_number = app_data
        .courses
        .iter()
        .filter(|c| c.course_group == course_group_id)
        .count()
        + 1;
    let new_course = Course {
        course_group: course_group_id,
        id: next_id,
        number: parallel_number as u8,
        teacher: None,
        room: None,
        hours_per_week,
        students: Vec::new(),
    };
    app_data.courses.push(new_course);
    Ok(app_data.clone())
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
            clear_app_data,
            update_course,
            create_parallel_course
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use blockplaner_core::{data::AppData, model::block::CourseBlock};

use crate::state::AppState;

#[tauri::command]
pub fn new_course_block(app_state: tauri::State<AppState>) -> Result<AppData, String> {
    let mut app_data = app_state.app_data.lock().unwrap();
    let next_id = app_data.course_blocks
        .iter()
        .map(|cb| cb.id)
        .max()
        .unwrap_or_default()
        + 1;
    let new_course_block = CourseBlock::new(next_id);
    app_data.course_blocks.push(new_course_block);
    Ok(app_data.clone())
}

#[tauri::command]
pub fn update_course_block(id: usize, name: String, courses: Vec<usize>, app_state: tauri::State<AppState>) -> Result<AppData, String> {
    let mut app_data = app_state.app_data.lock().unwrap();
    let course_block = app_data.course_blocks
        .iter_mut()
        .find(|cb| cb.id == id)
        .ok_or_else(|| format!("kursblock mit der ID {id} nicht gefunden!"))?;

    course_block.name = Some(name);
    course_block.courses = courses;
    Ok(app_data.clone())
}
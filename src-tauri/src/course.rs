use blockplaner_core::{data::AppData, model::course::Course};

use crate::state::AppState;

#[tauri::command]
pub fn update_course(
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
pub fn create_parallel_course(
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
    app_data.courses.sort();
    Ok(app_data.clone())
}

#[tauri::command]
pub fn delete_course(
    course_group_id: usize,
    app_state: tauri::State<AppState>
) -> Result<AppData, String> {
    let mut app_data = app_state.app_data.lock().unwrap();
    
    let index = app_data
        .courses
        .iter()
        .rposition(|c| c.course_group == course_group_id)
        .ok_or_else(|| format!("Kursgruppe mit der ID {course_group_id} enthält keinen Kurs!"))?;
    app_data.courses.remove(index);
    app_data.courses.sort();
    Ok(app_data.clone())
}

#[tauri::command]
pub fn create_courses(app_state: tauri::State<AppState>) -> Result<AppData, String> {
    let mut app_data = app_state.app_data.lock().unwrap();

    let courses = blockplaner_core::model::course::create_courses(&app_data.course_groups, app_data.max_students_per_course, &app_data.students);
    app_data.courses = courses;
    Ok(app_data.clone())
}
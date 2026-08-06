use std::collections::HashMap;

use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
struct CourseGroup {
    id: usize,
    subject: String,
    course_type: String,
    students_count: usize,
}

#[tauri::command]
pub fn get_course_groups(app_state: tauri::State<AppState>) -> Result<String, String> {
    let app_data = app_state.app_data.lock().unwrap();

    let mut students_count_by_id: HashMap<usize, usize> = HashMap::new();
    
    for student in &app_data.students {
        for &choice in &student.choices {
            *students_count_by_id
                .entry(choice)
                .or_default() += 1;
        }
    }

    let course_groups: Vec<_> = app_data.course_groups
        .iter()
        .map(|c| {
            let students_count = students_count_by_id.get(&c.id).copied().unwrap_or_default();
            CourseGroup {
                id: c.id,
                subject: c.subject.clone(),
                course_type: c.course_type.to_string(),
                students_count
            }
        })
        .collect();
    serde_json::to_string(&course_groups).map_err(|e| format!("Fehler beim Serialisieren der Kursgruppen: {e}"))

}
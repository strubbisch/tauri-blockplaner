use blockplaner_core::data::AppData;
use std::sync::{Arc, Mutex};

pub struct AppState {
    pub app_data: Arc<Mutex<AppData>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            app_data: Arc::clone(&self.app_data),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            app_data: Arc::new(Mutex::new(AppData::default())),
        }
    }
}

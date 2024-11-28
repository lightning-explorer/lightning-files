use crate::directory_nav_service::{dtos::inline_query_dto::InlineQueryDTO, models::sys_file_model::SystemFileModel};
use std::sync::{Arc, RwLock, RwLockWriteGuard};

pub struct FilesDisplayState {
    files_display: Vec<SystemFileModel>,
}

impl FilesDisplayState {
    pub fn new() -> Self {
        Self {
            files_display: Vec::new(),
        }
    }
    pub fn lock<'a>(
        arc_self: &'a Arc<RwLock<Self>>,
    ) -> Result<RwLockWriteGuard<'a, FilesDisplayState>, String> {
        match arc_self.write() {
            Ok(val) => Ok(val),
            Err(_) => Err("Failed to lock state".to_string()),
        }
    }
    pub fn add_file(&mut self, dto: SystemFileModel) {
        self.files_display.push(dto);
    }
    pub fn clear_files(&mut self) {
        self.files_display.clear();
    }
    pub fn query(&self, query: InlineQueryDTO) -> Vec<SystemFileModel> {
        self.files_display
            .iter()
            .filter(|dto| {
                dto.name
                    .to_lowercase()
                    .contains(&query.query.to_lowercase())
            })
            .cloned()
            .collect()
    }
}

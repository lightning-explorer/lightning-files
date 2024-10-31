use crate::directory_nav_service::dtos::inline_query_dto::InlineQueryDTO;
use crate::shared::dtos::file_dto::FileDTO;
use std::sync::{Arc, RwLock, RwLockWriteGuard};

pub struct FilesDisplayState {
    files_display: Vec<FileDTO>,
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
    pub fn add_dto(&mut self, dto: FileDTO) {
        self.files_display.push(dto);
    }
    pub fn clear_dtos(&mut self) {
        self.files_display.clear();
    }
    pub fn query(&self, query: InlineQueryDTO) -> Vec<FileDTO> {
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

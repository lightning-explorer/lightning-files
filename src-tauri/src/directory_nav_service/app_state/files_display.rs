use tokio::sync::RwLock;

use crate::directory_nav_service::dtos::inline_query_dto::InlineQueryDTO;
use crate::shared::models::sys_file_model::SystemFileModel;

pub struct FilesDisplayState {
    files_display: RwLock<Vec<SystemFileModel>>,
}

impl FilesDisplayState {
    pub fn new() -> Self {
        Self {
            files_display: RwLock::new(Vec::new()),
        }
    }
    pub async fn add_files(&self, files: &mut Vec<SystemFileModel>) {
        self.files_display.write().await.append(files);
    }
    pub async fn clear_files(&self) {
        self.files_display.write().await.clear();
    }
    pub async fn query(&self, query: InlineQueryDTO) -> Vec<SystemFileModel> {
        self.files_display
            .read()
            .await
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

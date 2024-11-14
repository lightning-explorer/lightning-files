use crate::tantivy_file_indexer::services::app_save::service::AppSaveService;

use super::tables::files::api::FilesTable;
use sea_orm::{Database, DatabaseConnection};

pub struct LocalDbService {
    files_table: FilesTable,
}

impl LocalDbService {
    pub async fn new_async(save_service: &AppSaveService) -> Self {
        let db_path = save_service.create_path("file_index.db");
        let db_url = db_path.to_string_lossy().to_string();

        let db: DatabaseConnection = Database::connect(&db_url)
            .await
            .expect("Failed to create initial database connection");
        let files_table = FilesTable::new_async(db.clone()).await;

        Self { files_table }
    }

    pub fn files_table(&self) -> &FilesTable {
        &self.files_table
    }
}

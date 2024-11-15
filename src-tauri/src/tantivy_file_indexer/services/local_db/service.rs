use crate::tantivy_file_indexer::services::app_save::service::AppSaveService;

use super::tables::{
    files::api::FilesTable, recently_indexed_dirs::api::RecentlyIndexedDirectoriesTable,
};
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::sync::Arc;

pub struct LocalDbService {
    pool: Arc<Pool<Sqlite>>,
    files_table: FilesTable,
    recently_indexed_dirs_table: RecentlyIndexedDirectoriesTable,
}

impl LocalDbService {
    // consider using a config here
    pub async fn new_async(save_service: &AppSaveService) -> Self {
        let db_path = save_service.create_path("file_index.db");
        let db_url = format!("sqlite://{}", db_path.to_string_lossy());

        let pool = Arc::new(SqlitePool::connect(&db_url).await.unwrap());

        // initialize the tables
        let files_table = FilesTable::new_async(Arc::clone(&pool)).await;
        let recently_indexed_dirs_table =
            RecentlyIndexedDirectoriesTable::new_async(Arc::clone(&pool)).await;

        Self { pool, files_table, recently_indexed_dirs_table }
    }

    pub fn files_table(&self) -> &FilesTable {
        &self.files_table
    }
    /**
     * Runs a command on the database to reclaim unused memory
     */
    pub async fn vacuum(&self) -> Result<(), String> {
        match sqlx::query("VACUUM").execute(&*self.pool).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}

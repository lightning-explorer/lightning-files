use std::sync::Arc;

use crate::tantivy_file_indexer::services::app_save::service::AppSaveService;

use super::tables::{
    crawler_queue::api::CrawlerQueueTable, 
    indexer_queue::api::IndexerQueueTable,
    recently_indexed_dirs::api::RecentlyIndexedDirectoriesTable,
};
use sea_orm::DatabaseConnection;
use sqlx::sqlite::SqlitePool;

pub struct LocalDbService {
    recently_indexed_dirs_table: RecentlyIndexedDirectoriesTable,
    crawler_queue_table: CrawlerQueueTable,
    indexer_queue_table: IndexerQueueTable,
}

impl LocalDbService {
    // consider using a config here
    pub async fn new_async(save_service: &AppSaveService) -> Self {
        let db_path = save_service.create_path("file_index.db");
        let db_url = format!("sqlite://{}", db_path.to_string_lossy());

        // Starts out as a SQLX pool, but 'into' is called to turn it into a Sea ORM database connection
        let db: Arc<DatabaseConnection> =
            Arc::new(SqlitePool::connect(&db_url).await.unwrap().into());

        // initialize the tables
        let recently_indexed_dirs_table =
            RecentlyIndexedDirectoriesTable::new_async(db.clone()).await;
        let crawler_queue_table = CrawlerQueueTable::new_async(db.clone()).await;
        let indexer_queue_table = IndexerQueueTable::new_async(db.clone()).await;

        Self {
            recently_indexed_dirs_table,
            crawler_queue_table,
            indexer_queue_table,
        }
    }

    pub fn recently_indexed_dirs_table(&self) -> &RecentlyIndexedDirectoriesTable {
        &self.recently_indexed_dirs_table
    }

    pub fn crawler_queue_table(&self) -> &CrawlerQueueTable {
        &self.crawler_queue_table
    }

    pub fn indexer_queue_table(&self) -> &IndexerQueueTable {
        &self.indexer_queue_table
    }
}

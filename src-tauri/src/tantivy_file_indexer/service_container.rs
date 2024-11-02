use super::services::{local_crawler::service::FileCrawlerService, local_db::service::SqlxService, search_index::service::SearchIndexService};
use std::sync::{Arc, RwLock};

use tauri::{AppHandle, Manager};

use super::configs::file_indexer_config::FileIndexerConfig;
use crate::FilesDisplayState;

pub struct AppServiceContainer {
    pub search_service: Arc<SearchIndexService>,
    pub sqlx_service: Arc<SqlxService>,
    pub crawler_service: Arc<FileCrawlerService>
}

impl AppServiceContainer {
    pub async fn new_async(handle: &AppHandle) -> Self {
        let files_display_state = Self::initialize_files_display_state();
        let config = Self::create_file_indexer_config();
        let search_service = Self::initialize_search_service(&config);
        let sqlx_service = Self::initialize_sqlx_service().await;
        let crawler_service = Self::initialize_crawler_service(8, search_service.clone(), sqlx_service.clone());

        handle.manage(files_display_state.clone());
        handle.manage(search_service.clone());
        handle.manage(sqlx_service.clone());
        handle.manage(crawler_service.clone());

        Self {
            search_service,
            sqlx_service,
            crawler_service,
        }
    }

    fn create_file_indexer_config() -> FileIndexerConfig {
        FileIndexerConfig {
            buffer_size: 50_000_000,
            indexer_batch_size: 64,
            indexer_tasks_limit: 6,
        }
    }

    fn initialize_files_display_state() -> Arc<RwLock<FilesDisplayState>> {
        Arc::new(RwLock::new(FilesDisplayState::new()))
    }

    fn initialize_search_service(config: &FileIndexerConfig) -> Arc<SearchIndexService> {
        Arc::new(SearchIndexService::new(config))
    }

    async fn initialize_sqlx_service() -> Arc<SqlxService> {
        Arc::new(SqlxService::new_async().await)
    }

    fn initialize_crawler_service(max_concurrent:usize,search_service: Arc<SearchIndexService>,
        sqlx_service: Arc<SqlxService>) -> Arc<FileCrawlerService>{
        Arc::new(FileCrawlerService::new(max_concurrent,search_service,sqlx_service))
    }
}

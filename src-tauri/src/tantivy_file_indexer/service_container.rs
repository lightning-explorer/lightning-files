use super::{db::sqlx_service::SqlxService, service::search_index_service::SearchIndexService};
use std::sync::{Arc, RwLock};

use tauri::{AppHandle, Manager};

use super::configs::file_indexer_config::FileIndexerConfig;
use crate::FilesDisplayState;

pub struct AppServiceContainer {
    pub search_service: Arc<SearchIndexService>,
    pub sqlx_service: Arc<SqlxService>,
}

impl AppServiceContainer {
    pub async fn new_async(handle: &AppHandle) -> Self {
        let files_display_state = Self::initialize_files_display_state();
        let config = Self::create_file_indexer_config();
        let search_service = Self::initialize_search_service(&config);
        let sqlx_service = Self::initialize_sqlx_service().await;

        Self::manage_services(handle, &files_display_state, &search_service, &sqlx_service);

        Self {
            search_service,
            sqlx_service,
        }
    }

    fn create_file_indexer_config() -> FileIndexerConfig {
        FileIndexerConfig {
            buffer_size: 50_000_000,
            indexer_batch_size: 128,
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

    fn manage_services(
        handle: &AppHandle,
        files_display_state: &Arc<RwLock<FilesDisplayState>>,
        search_service: &Arc<SearchIndexService>,
        sqlx_service: &Arc<SqlxService>,
    ) {
        handle.manage(files_display_state.clone());
        handle.manage(search_service.clone());
        handle.manage(sqlx_service.clone());
    }
}

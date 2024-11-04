use super::services::{
    app_save::service::{AppSavePath, AppSaveService},
    local_crawler::service::FileCrawlerService,
    local_db::service::SqlxService,
    search_index::service::SearchIndexService,
};
use std::sync::{Arc, RwLock};

use tauri::{AppHandle, Manager};

use super::configs::file_indexer_config::FileIndexerConfig;
use crate::FilesDisplayState;

pub struct AppServiceContainer {
    pub search_service: Arc<SearchIndexService>,
    pub sqlx_service: Arc<SqlxService>,
    pub crawler_service: Arc<FileCrawlerService>,
}

impl AppServiceContainer {
    pub async fn new_async(handle: &AppHandle) -> Self {
        let app_name = "DesktopSearch";

        let app_save_service = Self::initialize_app_save_service(
            AppSavePath::Other("D:\\DSearch".to_string()),
            app_name,
        );

        let files_display_state = Self::initialize_files_display_state();
        let config = Self::create_file_indexer_config(&app_save_service);
        let search_service = Self::initialize_search_service(&config);

        let sqlx_service = Self::initialize_sqlx_service(&app_save_service).await;
        let crawler_service = Self::initialize_crawler_service(
            8,
            search_service.clone(),
            sqlx_service.clone(),
            app_save_service.clone(),
        )
        .await;

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

    fn create_file_indexer_config(app_save_service: &Arc<AppSaveService>) -> FileIndexerConfig {
        FileIndexerConfig {
            buffer_size: 50_000_000,
            indexer_batch_size: 64,
            indexer_tasks_limit: 6,
            app_path: app_save_service.save_dir.clone(),
        }
    }

    fn initialize_files_display_state() -> Arc<RwLock<FilesDisplayState>> {
        Arc::new(RwLock::new(FilesDisplayState::new()))
    }

    fn initialize_search_service(config: &FileIndexerConfig) -> Arc<SearchIndexService> {
        Arc::new(SearchIndexService::new(config))
    }

    fn initialize_app_save_service(save_dir: AppSavePath, app_name: &str) -> Arc<AppSaveService> {
        Arc::new(AppSaveService::new(save_dir, app_name))
    }

    async fn initialize_sqlx_service(app_save_service: &Arc<AppSaveService>) -> Arc<SqlxService> {
        Arc::new(SqlxService::new_async(app_save_service).await)
    }

    async fn initialize_crawler_service(
        max_concurrent: usize,
        search_service: Arc<SearchIndexService>,
        sqlx_service: Arc<SqlxService>,
        save_service: Arc<AppSaveService>,
    ) -> Arc<FileCrawlerService> {
        Arc::new(
            FileCrawlerService::new_async(
                max_concurrent,
                8,
                search_service,
                sqlx_service,
                save_service,
            )
            .await,
        )
    }
}

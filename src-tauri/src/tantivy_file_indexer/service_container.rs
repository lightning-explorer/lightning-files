use super::services::{
    app_save::service::{AppSavePath, AppSaveService},
    local_crawler::service::FileCrawlerService,
    local_db::service::LocalDbService,
    search_index::service::SearchIndexService,
    vector_db::service::VectorDbService,
};
use std::sync::{Arc, RwLock};

use tauri::{AppHandle, Manager};

use super::configs::file_indexer_config::FileIndexerConfig;
use crate::FilesDisplayState;

pub struct AppServiceContainer {
    pub search_service: Arc<SearchIndexService>,
    pub local_db_service: Arc<LocalDbService>,
    pub crawler_service: Arc<FileCrawlerService>,
    pub vector_db_service: Arc<VectorDbService>,
}

impl AppServiceContainer {
    pub async fn new_async(handle: &AppHandle) -> Self {
        let app_name = "DesktopSearch";

        // AppSavePath::Other("D:\\DSearch".to_string())
        let app_save_service = Self::initialize_app_save_service(AppSavePath::AppData, app_name);

        let files_display_state = Self::initialize_files_display_state();
        let config = Self::create_file_indexer_config(&app_save_service);
        let vector_db_service = Self::initialize_vector_service();
        let search_service = Self::initialize_search_service(&config, &vector_db_service);

        // TODO: Remove this:
        vector_db_service.delete_all_collections().await;

        let local_db_service = Self::initialize_sqlx_service(&app_save_service).await;
        let crawler_service =
            Self::initialize_crawler_service(8, 512, Arc::clone(&local_db_service)).await;

        handle.manage(Arc::clone(&files_display_state));
        handle.manage(Arc::clone(&search_service));
        handle.manage(Arc::clone(&local_db_service));
        handle.manage(Arc::clone(&crawler_service));
        handle.manage(Arc::clone(&vector_db_service));

        handle.manage(Arc::clone(&app_save_service));

        Self {
            search_service,
            local_db_service,
            crawler_service,
            vector_db_service,
        }
    }

    fn create_file_indexer_config(app_save_service: &Arc<AppSaveService>) -> FileIndexerConfig {
        FileIndexerConfig {
            buffer_size: 50_000_000,
            indexer_batch_size: 256,
            app_path: app_save_service.save_dir.clone(),
        }
    }

    fn initialize_files_display_state() -> Arc<RwLock<FilesDisplayState>> {
        Arc::new(RwLock::new(FilesDisplayState::new()))
    }

    fn initialize_search_service(
        config: &FileIndexerConfig,
        vector_db_service: &Arc<VectorDbService>,
    ) -> Arc<SearchIndexService> {
        let clone = Arc::clone(vector_db_service);
        Arc::new(SearchIndexService::new(config, clone))
    }

    fn initialize_app_save_service(save_dir: AppSavePath, app_name: &str) -> Arc<AppSaveService> {
        Arc::new(AppSaveService::new(save_dir, app_name))
    }

    async fn initialize_sqlx_service(
        app_save_service: &Arc<AppSaveService>,
    ) -> Arc<LocalDbService> {
        Arc::new(LocalDbService::new_async(app_save_service).await)
    }

    fn initialize_vector_service() -> Arc<VectorDbService> {
        Arc::new(VectorDbService::new())
    }

    async fn initialize_crawler_service(
        max_concurrent: usize,
        save_after_iters: usize,
        db_service:Arc<LocalDbService>
    ) -> Arc<FileCrawlerService> {
        Arc::new(
            FileCrawlerService::new_async(max_concurrent, save_after_iters, db_service).await,
        )
    }
}

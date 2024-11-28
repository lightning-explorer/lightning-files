use super::services::{
    app_save::service::{AppSavePath, AppSaveService},
    local_crawler::{analyzer::service::FileCrawlerAnalyzerService, service::FileCrawlerService},
    local_db::service::LocalDbService,
    search_index::service::SearchIndexService,
    vector_db::service::VectorDbService,
};
use std::{
    path::PathBuf,
    sync::{Arc, RwLock},
};

use tauri::{AppHandle, Manager};

use crate::FilesDisplayState;

pub struct AppServiceContainer {
    pub search_service: Arc<SearchIndexService>,
    pub local_db_service: Arc<LocalDbService>,
    pub crawler_service: Arc<FileCrawlerService>,
    pub crawler_analyzer_service: Arc<FileCrawlerAnalyzerService>,
}

impl AppServiceContainer {
    pub async fn new_async(handle: &AppHandle) -> Self {
        let app_name = "DesktopSearch";

        // Ensure that the app service is initialized before the rest to ensure that the AppData save path is created
        let app_save_service = Self::initialize_app_save_service(
            AppSavePath::Other(PathBuf::from("D:\\DesktopSearch")),
            app_name, 
        );
        let app_path = app_save_service.save_dir.clone();

        let files_display_state = Self::initialize_files_display_state();

        let vector_db_service = Self::initialize_vector_service();
        let search_service = Self::initialize_search_service(50_000_000,app_path, &vector_db_service);

        // TODO: Remove this:
        vector_db_service.delete_all_collections().await;

        let local_db_service = Self::initialize_sqlx_service(&app_save_service).await;

        // TODO: Attach the analyzer to the crawling operation
        let crawler_analyzer_service = Self::initialize_crawler_analyzer_service(15);

        let crawler_service =
            Self::initialize_crawler_service(8, Arc::clone(&local_db_service)).await;

        handle.manage(Arc::clone(&files_display_state));
        handle.manage(Arc::clone(&search_service));
        handle.manage(Arc::clone(&local_db_service));
        handle.manage(Arc::clone(&crawler_service));
        handle.manage(Arc::clone(&crawler_analyzer_service));
        handle.manage(Arc::clone(&vector_db_service));

        handle.manage(Arc::clone(&app_save_service));

        Self {
            search_service,
            local_db_service,
            crawler_service,
            crawler_analyzer_service,
        }
    }

    fn initialize_files_display_state() -> Arc<RwLock<FilesDisplayState>> {
        Arc::new(RwLock::new(FilesDisplayState::new()))
    }

    fn initialize_search_service(
        buffer_size: usize,
        app_path: PathBuf,
        vector_db_service: &Arc<VectorDbService>,
    ) -> Arc<SearchIndexService> {
        let vector_db_clone = Arc::clone(vector_db_service);
        Arc::new(SearchIndexService::new(
            buffer_size,
            app_path,
            vector_db_clone,
        ))
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
        db_service: Arc<LocalDbService>,
    ) -> Arc<FileCrawlerService> {
        Arc::new(FileCrawlerService::new_async(max_concurrent, db_service).await)
    }

    fn initialize_crawler_analyzer_service(analyze_every: u64) -> Arc<FileCrawlerAnalyzerService> {
        Arc::new(FileCrawlerAnalyzerService::new(analyze_every))
    }
}

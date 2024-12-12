use std::{path::Path, sync::Arc};

use tauri::{AppHandle, Manager, State};
use tokio::sync::Mutex;

use crate::tantivy_file_indexer::service_container::AppServiceContainer;

pub struct IsAppRunning {
    pub running: Arc<Mutex<bool>>,
}

/// Check to see if the backend is fully initialized and all state is managed
#[tauri::command]
pub async fn is_running(is_running: State<'_, IsAppRunning>) -> Result<bool, String> {
    let running = *is_running.running.lock().await;
    println!("Pinged is_running. Is the app running?: {}", running);
    Ok(running)
}

pub fn initialize_app(handle: AppHandle) {
    handle.manage(IsAppRunning {
        running: Arc::new(Mutex::new(false)),
    });
}

pub async fn initialize_app_async(handle: AppHandle) {
    let index_files = false;

    let service_container = AppServiceContainer::new_async(&handle).await;
    let crawler_service = Arc::clone(&service_container.crawler_service);
    //let crawler_analyzer_service = Arc::clone(&service_container.crawler_analyzer_service);
    let search_service = Arc::clone(&service_container.search_service);
    //let db_service = Arc::clone(&service_container.local_db_service);

    // Notify that the app is all set up:
    *handle.state::<IsAppRunning>().running.lock().await = true;
    println!("All services managed");

    if index_files {
        // Old file crawlers + indexers:
        // let sender = service_container
        //     .search_service
        //     .spawn_indexer_db_connected(db_service, 128, 8);

        // crawler_service.spawn_crawler_with_analyzer(sender, crawler_analyzer_service);

        // New file crawlers:
        let index_writer = Arc::clone(&search_service.index_writer);
        let schema = search_service.schema.clone();
        let handles = crawler_service
            .spawn_indexing_crawlers_sqlite(index_writer, schema, 128)
            .await;

        crawler_service
            .push_dirs_default(vec![Path::new("C:\\").to_path_buf()])
            .await;

        handles.join_all().await;
    } else {
        println!("index_files in initialize_app is set to false. No files will be indexed and no file crawlers will be spawned.")
    }
}

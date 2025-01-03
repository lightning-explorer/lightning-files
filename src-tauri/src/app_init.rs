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
    Ok(running)
}

pub fn initialize_app(handle: AppHandle) {
    handle.manage(IsAppRunning {
        running: Arc::new(Mutex::new(false)),
    });

    let handle_clone = handle.clone();
    tauri::async_runtime::spawn(async move { initialize_app_async(handle_clone).await });
}

pub async fn initialize_app_async(handle: AppHandle) {
    let index_files = true;

    let service_container = AppServiceContainer::new_async(&handle).await;
    let crawler_service = Arc::clone(&service_container.crawler_service);
    //let crawler_analyzer_service = Arc::clone(&service_container.crawler_analyzer_service);

    // Notify that the app is all set up:
    *handle.state::<IsAppRunning>().running.lock().await = true;
    println!("All services managed");

    if index_files {
        let handles = crawler_service
            .crawlers_builder()
            .batch_size(512)
            .max_num_workers(4)
            .build_async()
            .await;

        crawler_service
            .push_dirs_default(vec![Path::new("C:\\").to_path_buf()])
            .await;

        handles.join_all().await;
    } else {
        println!("index_files in initialize_app_async is set to false. No files will be indexed and no file crawlers will be spawned.")
    }
}

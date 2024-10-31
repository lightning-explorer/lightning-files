use directory_nav_service::service::*;
use tantivy_file_indexer::{
    crawlers::crawler::Crawler, service::exports::search_index_query,
    service_container::AppServiceContainer,
};
use tauri::{AppHandle, Manager};
mod directory_nav_service;
mod shared;
mod tantivy_file_indexer;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                initialize_app(app_handle).await;
            });

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_files_as_dtos,
            format_path_into_dir,
            get_directory_path,
            get_root_path,
            get_parent_directory,
            open_file,
            is_path_a_file,
            get_drives,
            search_files_inline,
            search_index_query
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn initialize_app(handle: AppHandle) {
    let service_container = AppServiceContainer::new_async(&handle).await;
    let crawler = Crawler::new_from_service(&service_container);
    tokio::spawn(crawler.crawl("", 128, 6));
    handle.manage(service_container);
}

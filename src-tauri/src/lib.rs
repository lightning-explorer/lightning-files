use directory_nav_service::tauri_exports::*;
use std::{path::Path, sync::Arc};
use tantivy_file_indexer::{
    service_container::AppServiceContainer, services::app_save::tauri_exports::*,
    services::local_crawler::tauri_exports::*, /*services::local_db::tables::files::tauri_exports::*,*/
    services::local_db::tauri_exports::*, services::search_index::tauri_exports::*,
    services::vector_db::tauri_exports::*,
};
use tauri::AppHandle;
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
            get_files_as_models,
            format_path_into_dir,
            get_directory_path,
            get_root_path,
            get_parent_directory,
            open_file,
            is_path_a_file,
            get_drives,
            search_files_inline,
            search_index_query,
            search_index_query_streaming,
            add_dirs_to_crawler_queue,
            //get_num_stored_files,
            save_json_local,
            load_json_local,
            vector_db_query,
            view_crawler_queue,
            view_crawler_priority_counts,
            get_crawler_analyzer_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn initialize_app(handle: AppHandle) {
    let index_files = false;

    let service_container = AppServiceContainer::new_async(&handle).await;
    let crawler_service = Arc::clone(&service_container.crawler_service);
    //let crawler_analyzer_service = Arc::clone(&service_container.crawler_analyzer_service);
    let search_service = Arc::clone(&service_container.search_service);
    //let db_service = Arc::clone(&service_container.local_db_service);

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

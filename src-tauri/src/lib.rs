use directory_nav_service::tauri_exports::*;
use std::{path::Path, sync::Arc};
use tantivy_file_indexer::{
    service_container::AppServiceContainer, services::app_save::tauri_exports::*,
    services::local_crawler::tauri_exports::*, /*services::local_db::tables::files::tauri_exports::*,*/
    services::local_db::tauri_exports::*, services::search_index::tauri_exports::*,
    services::vector_db::tauri_exports::*,
};
use tauri::AppHandle;
mod app_init;
mod directory_nav_service;
mod shared;
mod tantivy_file_indexer;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_files_as_models,
            format_path_into_dir,
            get_directory_path,
            get_root_path,
            get_parent_directory,
            open_file,
            read_file_bytes,
            read_file,
            read_file_range,
            read_file_range_bytes,
            is_path_a_file,
            get_drives,
            search_files_inline,
            search_index_query,
            search_index_query_streaming,
            search_index_query_streaming_organized,
            //get_num_stored_files,
            save_json_local,
            load_json_local,
            //vector_db_query,
            add_dirs_to_crawler_queue,
            view_crawler_queue,
            view_crawler_priority_counts,
            get_crawler_analyzer_data,
            app_init::is_running,
            is_directory_accessible
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let app_handle2 = app.handle().clone();

            app_init::initialize_app(app_handle);

            tauri::async_runtime::spawn(async move {
                app_init::initialize_app_async(app_handle2).await
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

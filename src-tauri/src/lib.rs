use directory_nav_service::tauri_exports::*;
use tantivy_file_indexer::{
    services::app_save::tauri_exports::*,
    services::local_crawler::tauri_exports::*, /*services::local_db::tables::files::tauri_exports::*,*/
    services::local_db::tauri_exports::*, services::search_index::tauri_exports::*,
    services::local_db::tables::app_kv_store::tauri_exports::*
};
mod app_init;
mod directory_nav_service;
mod shared;
mod tantivy_file_indexer;


pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_drag::init())
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
            //get_crawler_analyzer_data,
            app_init::is_running,
            is_directory_accessible,
            get_file_from_index,
            upsert_file_to_index,
            get_sys_info,
            // Common commands:
            move_path_into_directory,
            delete_file,
            open_in_explorer,
            // Key value JSON storage:
            kv_store_set,
            kv_store_get,
            kv_store_subscribe_to_key,
            dispatch_crawlers,
            watch_directory,
            stop_watching_directory,
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            app_init::initialize_app(app_handle);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::{
    path::Path,
    sync::{Arc, RwLock},
};

use directory_nav_service::service::*;
use tantivy_file_indexer::{
    configs::file_indexer_config::FileIndexerConfig, search_index_service::SearchIndexService,
};
mod directory_nav_service;
mod shared;
mod tantivy_file_indexer;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let files_display_state = Arc::new(RwLock::new(FilesDisplayState::new()));

    let buffer_size: usize = 50_000_000;
    let indexer_batch_size: usize = 128;
    let indexer_tasks_limit: usize = 6;

    let config = FileIndexerConfig {
        buffer_size,
        indexer_batch_size,
        indexer_tasks_limit,
    };

    let service = SearchIndexService::new(&config);

    service.spawn_crawler("D:\\".to_string());

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(files_display_state)
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

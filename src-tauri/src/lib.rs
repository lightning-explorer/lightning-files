use std::sync::{Arc, RwLock};

use directory_nav_service::service::*;
mod directory_nav_service;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let files_display_state = Arc::new(RwLock::new(FilesDisplayState::new()));
    
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
